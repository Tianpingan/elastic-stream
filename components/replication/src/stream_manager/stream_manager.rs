use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use client::Client;
use config::Configuration;
use log::warn;
use model::stream::StreamMetadata;
use tokio::sync::{broadcast, mpsc, oneshot};

use crate::{
    request::{
        AppendRequest, AppendResponse, CloseStreamRequest, CreateStreamRequest,
        CreateStreamResponse, OpenStreamRequest, OpenStreamResponse, ReadRequest, ReadResponse,
        Request, TrimRequest,
    },
    stream_manager::replication_stream::ReplicationStream,
    ReplicationError,
};

use super::replication_stream::StreamAppendContext;

pub(crate) struct StreamManager {
    config: Arc<Configuration>,
    rx: mpsc::UnboundedReceiver<Request>,
    client: Rc<Client>,
    streams: Rc<RefCell<HashMap<u64, Rc<ReplicationStream>>>>,
}

impl StreamManager {
    pub(crate) fn new(config: Arc<Configuration>, rx: mpsc::UnboundedReceiver<Request>) -> Self {
        let (shutdown, _rx) = broadcast::channel(1);
        let client = Rc::new(Client::new(Arc::clone(&config), shutdown));
        let streams = Rc::new(RefCell::new(HashMap::new()));
        Self {
            config,
            rx,
            client,
            streams,
        }
    }

    pub(crate) fn spawn_loop(mut this: Self) {
        tokio_uring::spawn(async move {
            loop {
                match this.rx.recv().await {
                    Some(request) => match request {
                        Request::Append { request, tx } => {
                            this.append(request, tx);
                        }
                        Request::Read { request, tx } => {
                            this.fetch(request, tx);
                        }
                        Request::CreateStream { request, tx } => {
                            this.create(request, tx);
                        }
                        Request::OpenStream { request, tx } => {
                            this.open(request, tx);
                        }
                        Request::CloseStream { request, tx } => {
                            this.close(request, tx);
                        }
                        Request::StartOffset { request, tx } => {
                            this.start_offset(request, tx);
                        }
                        Request::NextOffset { request, tx } => {
                            this.next_offset(request, tx);
                        }
                        Request::Trim { request, tx } => {
                            this.trim(request, tx);
                        }
                    },
                    None => {
                        break;
                    }
                }
            }
        });
    }

    fn append(
        &mut self,
        request: AppendRequest,
        tx: oneshot::Sender<Result<AppendResponse, ReplicationError>>,
    ) {
        let stream = if let Some(stream) = self.streams.borrow().get(&request.stream_id) {
            Some(stream.clone())
        } else {
            None
        };
        if let Some(stream) = stream {
            let stream = stream.clone();
            tokio_uring::spawn(async move {
                let result = stream
                    .append(request.data, StreamAppendContext::new(request.count))
                    .await
                    .map(|offset| AppendResponse { offset });
                let _ = tx.send(result);
            });
        } else {
            let _ = tx.send(Err(ReplicationError::StreamNotExist));
        }
    }

    fn fetch(
        &mut self,
        request: ReadRequest,
        tx: oneshot::Sender<Result<ReadResponse, ReplicationError>>,
    ) {
        let stream = if let Some(stream) = self.streams.borrow().get(&request.stream_id) {
            Some(stream.clone())
        } else {
            None
        };
        if let Some(stream) = stream {
            let stream = stream.clone();
            tokio_uring::spawn(async move {
                let result = stream
                    .fetch(
                        request.start_offset,
                        request.end_offset,
                        request.batch_max_bytes,
                    )
                    .await
                    .map(|data| ReadResponse { data });
                let _ = tx.send(result);
            });
        } else {
            let _ = tx.send(Err(ReplicationError::StreamNotExist));
        }
    }

    fn create(
        &mut self,
        request: CreateStreamRequest,
        tx: oneshot::Sender<Result<CreateStreamResponse, ReplicationError>>,
    ) {
        let metadata = StreamMetadata {
            stream_id: None,
            replica: request.replica,
            ack_count: request.ack_count,
            retention_period: request.retension_period,
        };
        let client = self.client.clone();
        tokio_uring::spawn(async move {
            let result = client
                .create_stream(metadata)
                .await
                .map(|metadata| CreateStreamResponse {
                    // TODO: unify stream_id type
                    stream_id: metadata.stream_id.unwrap(),
                })
                .map_err(|e| {
                    warn!("Failed to create stream, {}", e);
                    ReplicationError::Internal
                });
            let _ = tx.send(result);
        });
    }

    fn open(
        &mut self,
        request: OpenStreamRequest,
        tx: oneshot::Sender<Result<OpenStreamResponse, ReplicationError>>,
    ) {
        let client = self.client.clone();
        let streams = self.streams.clone();
        tokio_uring::spawn(async move {
            let client = Rc::downgrade(&client);
            let stream = ReplicationStream::new(request.stream_id as i64, request.epoch, client);
            if let Err(e) = stream.open().await {
                let _ = tx.send(Err(e));
                return;
            }
            streams.borrow_mut().insert(request.stream_id, stream);
            let _ = tx.send(Ok(OpenStreamResponse {}));
        });
    }

    fn close(
        &mut self,
        request: CloseStreamRequest,
        tx: oneshot::Sender<Result<(), ReplicationError>>,
    ) {
        let stream = if let Some(stream) = self.streams.borrow_mut().remove(&request.stream_id) {
            Some(stream.clone())
        } else {
            None
        };
        if let Some(stream) = stream {
            tokio_uring::spawn(async move {
                stream.close().await;
                let _ = tx.send(Ok(()));
            });
        } else {
            let _ = tx.send(Err(ReplicationError::StreamNotExist));
        }
    }

    fn start_offset(&mut self, stream_id: u64, tx: oneshot::Sender<Result<u64, ReplicationError>>) {
        let result = if let Some(stream) = self.streams.borrow().get(&stream_id) {
            Ok(stream.start_offset())
        } else {
            Err(ReplicationError::StreamNotExist)
        };
        let _ = tx.send(result);
    }

    fn next_offset(&mut self, stream_id: u64, tx: oneshot::Sender<Result<u64, ReplicationError>>) {
        let result = if let Some(stream) = self.streams.borrow().get(&stream_id) {
            Ok(stream.next_offset())
        } else {
            Err(ReplicationError::StreamNotExist)
        };
        let _ = tx.send(result);
    }

    fn trim(&mut self, request: TrimRequest, tx: oneshot::Sender<Result<(), ReplicationError>>) {
        let stream = if let Some(stream) = self.streams.borrow_mut().remove(&request.stream_id) {
            Some(stream.clone())
        } else {
            None
        };
        if let Some(stream) = stream {
            tokio_uring::spawn(async move {
                let _ = tx.send(stream.trim(request.new_start_offset).await);
            });
        } else {
            let _ = tx.send(Err(ReplicationError::StreamNotExist));
        }
    }
}
