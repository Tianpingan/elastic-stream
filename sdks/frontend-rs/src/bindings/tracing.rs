use chrono::Local;
use log::info;
use minitrace::{prelude::{SpanRecord, Collector}, Span, local::Guard};
use tokio::sync::mpsc::{self, UnboundedSender};
use std::{net::SocketAddr, time::{Duration, self, UNIX_EPOCH, SystemTime}, alloc::System};

pub struct Tracer {
    span: Span,
    collector: Option<Collector>,
    tx: UnboundedSender<Collector>,
}
impl Tracer {
    pub fn new(root: Span, collector: Collector, tx: UnboundedSender<Collector>) -> Self {  
        Self { span: root, collector: Some(collector), tx}
    }
    pub fn get_child_span(&self, event: &'static str) -> Span {
        Span::enter_with_parent(event, &self.span)
    }
    pub fn get_child_span_with_local_parent(&self, event: &'static str) -> Span {
        Span::enter_with_local_parent(event)
    }
    pub fn set_local_parent(&self) -> Option<Guard<impl FnOnce()>> {
        self.span.set_local_parent()
    }
}
impl Drop for Tracer {
    fn drop(&mut self) {
        // send collector to tracing report thread
        if let Some(collector) = self.collector.take() {
            let _ = self.tx.send(collector);
        }
    }
}

pub struct TracingService {
    tx: mpsc::UnboundedSender<Collector>,
    trace_id: u64,
}
impl TracingService {
    pub fn new(threshold: Duration) -> Self {
        let (tx, mut rx) = mpsc::unbounded_channel::<Collector>();
        let _ = std::thread::Builder::new()
        .name("TracingServiceReportThread".to_string())
        .spawn(move || {
            let now = Local::now();
            let base_trace_id = now.timestamp_millis() as u64;
            let datetime_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let service_name = "JNI#".to_owned() + &datetime_str;
            let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
            let mut trace_id = 0;
            rt.block_on(async {
                loop {
                    match rx.recv().await {
                        Some(collector) => { 
                            trace_id = trace_id + 1;
                            println!("trace_id = {trace_id}");
                            let service_name = service_name.clone();
                            tokio::spawn(async move {
                                Self::report_tracing(threshold, collector, service_name, base_trace_id + trace_id).await;
                            });
                        }
                        None => { 
                            info!("tracing service report channel is dropped");
                            break;
                        }
                    }
                }
            });
        });
        Self {
            tx,
            trace_id: 0,
        }
    }
    pub fn new_tracer(&self, event: &'static str) -> Tracer {
        let (root, collector) = Span::root(event);
        Tracer::new(root, collector, self.tx.clone())
    }
    pub async fn report_tracing(threshold: Duration, collector: Collector, service_name: String, trace_id: u64) {
        let spans = collector.collect().await;
        let total_duration = spans.iter().map(|span| span.duration_ns).max().unwrap();
        if total_duration >= threshold.as_nanos() as u64 {
            // TODO: 
            let addr = SocketAddr::new("127.0.0.1".parse().unwrap(), 6831);
            let bytes = minitrace_jaeger::encode(
                service_name,
                trace_id,
                0,
                0,
                &spans,
            )
            .expect("encode error");
            let _ = minitrace_jaeger::report(addr, &bytes).await;
        }
    }    
}

