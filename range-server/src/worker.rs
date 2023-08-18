use crate::{
    connection_tracker::ConnectionTracker,
    heartbeat::Heartbeat,
    metadata::{MetadataManager, MetadataWatcher},
    range_manager::RangeManager,
    worker_config::WorkerConfig,
};
use client::{client::Client, DefaultClient};
use log::{debug, error, info, warn};
use observation::metrics::{
    store_metrics::RangeServerStatistics,
    sys_metrics::{DiskStatistics, MemoryStatistics},
    uring_metrics::UringStatistics,
};
use pd_client::PlacementDriverClient;
use protocol::rpc::header::RangeServerState;
use std::{
    cell::RefCell,
    error::Error,
    rc::Rc,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::broadcast;
use tokio_uring::net::TcpListener;
use util::metrics::http_serve;

/// A server aggregates one or more `Worker`s and each `Worker` takes up a dedicated CPU
/// processor, following the Thread-per-Core design paradigm.
///
/// There is only one primary worker, which is in charge of the stream/range management
/// and communication with the placement driver.
///
/// Inter-worker communications are achieved via channels.
pub(crate) struct Worker<M, W, Meta> {
    config: WorkerConfig,
    range_manager: Rc<M>,
    client: Rc<DefaultClient>,

    /// `MetadataWatcher` is having a `PlacementDriverClient` embedded in, which lists and watches
    /// resources of interest.
    ///
    /// Once the watcher fetches a set of resource events, it dispatches them to all metadata managers
    /// per worker through MPSC channels.
    metadata_watcher: Option<W>,

    /// `MetadataManager` caches all relevant metadata it receives and notifies all registered observers.
    metadata_manager: Meta,

    state: Rc<RefCell<RangeServerState>>,
}

impl<M, W, Meta> Worker<M, W, Meta>
where
    M: RangeManager + 'static,
    W: MetadataWatcher + 'static,
    Meta: MetadataManager + 'static,
{
    pub fn new(
        config: WorkerConfig,
        range_manager: Rc<M>,
        client: Rc<DefaultClient>,
        metadata_watcher: Option<W>,
        metadata_manager: Meta,
    ) -> Self {
        Self {
            config,
            range_manager,
            client,
            metadata_watcher,
            metadata_manager,
            state: Rc::new(RefCell::new(
                RangeServerState::RANGE_SERVER_STATE_READ_WRITE,
            )),
        }
    }

    pub fn serve<P>(&mut self, pd_client: Box<P>, shutdown: broadcast::Sender<()>)
    where
        P: PlacementDriverClient + 'static,
    {
        core_affinity::set_for_current(self.config.core_id);
        if self.config.primary {
            info!(
                "Bind primary worker to processor-{}",
                self.config.core_id.id
            );
        } else {
            info!("Bind worker to processor-{}", self.config.core_id.id);
        }

        info!(
            "The number of Submission Queue entries in uring: {}",
            self.config.server_config.server.uring.queue_depth
        );
        tokio_uring::builder()
            .entries(self.config.server_config.server.uring.queue_depth)
            .uring_builder(
                tokio_uring::uring_builder()
                    .dontfork()
                    .setup_attach_wq(self.config.sharing_uring),
            )
            .start(async {
                // Run dispatching service of Store.
                if let Some(ref mut watcher) = self.metadata_watcher {
                    watcher.start(pd_client);
                }

                self.metadata_manager.start().await;

                self.range_manager.start().await;

                let bind_address = &self.config.server_config.server.addr;
                let listener =
                    match TcpListener::bind(bind_address.parse().expect("Failed to bind")) {
                        Ok(listener) => {
                            info!("Server starts OK, listening {}", bind_address);
                            listener
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            return;
                        }
                    };

                if self.config.primary {
                    let port = self.config.server_config.observation.metrics.port;
                    let host = self.config.server_config.observation.metrics.host.clone();
                    tokio_uring::spawn(async move {
                        http_serve(&host, port).await;
                    });
                    self.report_metrics(shutdown.subscribe());
                }

                // TODO: report after pd can handle the request.
                // self.report_range_progress(shutdown.subscribe());

                self.heartbeat(shutdown.clone(), Rc::clone(&self.state));

                match self.run(listener, shutdown.subscribe()).await {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Runtime failed. Cause: {}", e.to_string());
                    }
                }
            });
    }

    fn report_metrics(&self, mut shutdown_rx: broadcast::Receiver<()>) {
        let client = Rc::clone(&self.client);
        let config = Arc::clone(&self.config.server_config);
        let state = Rc::clone(&self.state);
        tokio_uring::spawn(async move {
            // TODO: Modify it to client report metrics interval, instead of config.client_heartbeat_interval()
            let mut interval = tokio::time::interval(config.client_heartbeat_interval());
            let mut uring_statistics = UringStatistics::new();
            let mut range_server_statistics = RangeServerStatistics::new();
            let mut disk_statistics = DiskStatistics::new();
            let mut memory_statistics = MemoryStatistics::new();
            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        info!("Received shutdown signal. Stop accepting new connections.");
                        break;
                    }
                    _ = interval.tick() => {
                        uring_statistics.record();
                        range_server_statistics.record();
                        disk_statistics.record();
                        memory_statistics.record();
                        let state = *state.borrow();
                        let _ = client
                        .report_metrics(
                            &config.placement_driver, state, &uring_statistics, &range_server_statistics, &disk_statistics, &memory_statistics)
                        .await;
                    }

                }
            }
        });
    }

    fn heartbeat(&self, shutdown: broadcast::Sender<()>, state: Rc<RefCell<RangeServerState>>) {
        let client = Rc::clone(&self.client);
        let config = Arc::clone(&self.config.server_config);
        let heartbeat = Heartbeat::new(client, config, state, shutdown);
        heartbeat.run();
    }

    #[allow(dead_code)]
    fn report_range_progress(&self, mut shutdown_rx: broadcast::Receiver<()>) {
        let client = self.client.clone();
        let config = Arc::clone(&self.config.server_config);
        let range_manager = self.range_manager.clone();
        tokio_uring::spawn(async move {
            let mut interval = tokio::time::interval(config.client_heartbeat_interval());
            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        info!("Received shutdown signal. Stop report range progress.");
                        break;
                    }
                    _  = interval.tick() => {
                        let range_progress = range_manager.get_range_progress().await;
                        let _ = client.report_range_progress(range_progress).await;
                    }
                }
            }
        });
    }

    async fn run(
        &self,
        listener: TcpListener,
        mut shutdown_rx: broadcast::Receiver<()>,
    ) -> Result<(), Box<dyn Error>> {
        let connection_tracker = Rc::new(RefCell::new(ConnectionTracker::new()));
        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    info!("Received shutdown signal. Stop accepting new connections.");
                    break;
                }

                incoming = listener.accept() => {
                    let (stream, peer_socket_address) = match incoming {
                        Ok((stream, socket_addr)) => {
                            debug!("Accepted a new connection from {socket_addr:?}");
                            stream.set_nodelay(true).unwrap_or_else(|e| {
                                warn!("Failed to disable Nagle's algorithm. Cause: {e:?}, PeerAddress: {socket_addr:?}");
                            });
                            debug!("Nagle's algorithm turned off");

                            (stream, socket_addr)
                        }
                        Err(e) => {
                            error!(
                                "Failed to accept a connection. Cause: {}",
                                e.to_string()
                            );
                            break;
                        }
                    };

                    let session = super::session::Session::new(
                        Arc::clone(&self.config.server_config),
                        stream, peer_socket_address,
                        Rc::clone(&self.range_manager),
                        Rc::clone(&connection_tracker)
                       );
                    session.process();
                }
            }
        }

        // Send GoAway frame to all existing connections
        connection_tracker.borrow_mut().go_away();

        // Await till all existing connections disconnect
        let start = Instant::now();
        loop {
            let remaining = connection_tracker.borrow().len();
            if 0 == remaining {
                break;
            }

            info!(
                "There are {} connections left. Waited for {}/{} seconds",
                remaining,
                start.elapsed().as_secs(),
                self.config.server_config.server_grace_period().as_secs()
            );
            tokio::time::sleep(Duration::from_secs(1)).await;
            if Instant::now() >= start + self.config.server_config.server_grace_period() {
                break;
            }
        }

        info!("Server#run completed");
        Ok(())
    }
}
