use bytes::Bytes;
use jni::objects::{GlobalRef, JClass, JObject, JString, JValue, JValueGen};
use jni::sys::{jint, jlong, JNINativeInterface_, JNI_VERSION_1_8};
use jni::{JNIEnv, JavaVM};
use log::{error, info};
use std::alloc::Layout;
use std::cell::{OnceCell, RefCell};
use std::ffi::c_void;
use std::slice;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};

use crate::{ClientError, Frontend, Stream, StreamOptions};

use super::cmd::Command;

static mut TX: OnceCell<mpsc::UnboundedSender<Command>> = OnceCell::new();

thread_local! {
    static JAVA_VM: RefCell<Option<Arc<JavaVM>>> = RefCell::new(None);
    static JENV: RefCell<Option<*mut jni::sys::JNIEnv>> = RefCell::new(None);
}

async fn process_command(cmd: Command<'_>) {
    match cmd {
        Command::CreateStream {
            front_end,
            replica,
            ack_count,
            retention,
            future,
        } => {
            process_create_stream_command(front_end, replica, ack_count, retention, future).await;
        }
        Command::GetFrontend { access_point, tx } => {
            process_get_frontend_command(access_point, tx);
        }
        Command::OpenStream {
            front_end,
            stream_id,
            epoch,
            future,
        } => {
            process_open_stream_command(front_end, stream_id, epoch, future).await;
        }
        Command::StartOffset { stream, future } => {
            process_start_offset_command(stream, future).await;
        }
        Command::NextOffset { stream, future } => {
            process_next_offset_command(stream, future).await;
        }
        Command::Append {
            stream,
            buf,
            future,
        } => {
            process_append_command(stream, buf, future).await;
        }
        Command::Read {
            stream,
            start_offset,
            end_offset,
            batch_max_bytes,
            future,
        } => {
            process_read_command(stream, start_offset, end_offset, batch_max_bytes, future).await;
        }
    }
}
async fn process_append_command(stream: &mut Stream, buf: Bytes, future: GlobalRef) {
    let result = stream.append(buf).await;
    match result {
        Ok(result) => {
            let base_offset = result.base_offset;
            complete_future_with_jlong(future, base_offset);
        }
        Err(err) => {
            complete_future_with_error(future, err.to_string());
        }
    };
}
async fn process_read_command(
    stream: &mut Stream,
    start_offset: i64,
    end_offset: i64,
    batch_max_bytes: i32,
    future: GlobalRef,
) {
    let result = stream.read(start_offset, end_offset, batch_max_bytes).await;
    match result {
        Ok(buffers) => {
            // Copy buffers to `DirectByteBuffer`
            let total = buffers.iter().map(|buf| buf.len()).sum();
            if let Ok(layout) = Layout::from_size_align(total, 1) {
                let ptr = unsafe { std::alloc::alloc(layout) };
                let mut p = 0;
                buffers.iter().for_each(|buf| {
                    unsafe { std::ptr::copy_nonoverlapping(buf.as_ptr(), ptr.add(p), buf.len()) };
                    p += buf.len();
                });

                JENV.with(|cell| {
                    let mut env = get_thread_local_jenv(cell);
                    if let Ok(obj) = unsafe { env.new_direct_byte_buffer(ptr, total) } {
                        call_future_complete_method(env, future, JObject::from(obj));
                    } else {
                        error!("Failed to create a new direct_byte_buffer");
                    }
                });
            } else {
                error!("Bad alignment");
            }
        }
        Err(err) => {
            complete_future_with_error(future, err.to_string());
        }
    };
}

async fn process_start_offset_command(stream: &mut Stream, future: GlobalRef) {
    let result = stream.start_offset().await;
    match result {
        Ok(offset) => {
            complete_future_with_jlong(future, offset);
        }
        Err(err) => {
            complete_future_with_error(future, err.to_string());
        }
    };
}

async fn process_next_offset_command(stream: &mut Stream, future: GlobalRef) {
    let result = stream.next_offset().await;
    match result {
        Ok(offset) => {
            complete_future_with_jlong(future, offset);
        }
        Err(err) => {
            complete_future_with_error(future, err.to_string());
        }
    };
}

fn process_get_frontend_command(
    access_point: String,
    tx: oneshot::Sender<Result<Frontend, ClientError>>,
) {
    let result = Frontend::new(&access_point);
    if let Err(_e) = tx.send(result) {
        error!("Failed to dispatch JNI command to tokio-uring runtime");
    }
}

async fn process_open_stream_command(
    front_end: &mut Frontend,
    stream_id: u64,
    epoch: u64,
    future: GlobalRef,
) {
    let result = front_end.open(stream_id, epoch).await;
    match result {
        Ok(stream) => {
            let ptr = Box::into_raw(Box::new(stream)) as jlong;
            complete_future_with_stream(future, ptr);
        }
        Err(err) => {
            complete_future_with_error(future, err.to_string());
        }
    };
}

async fn process_create_stream_command(
    front_end: &mut Frontend,
    replica: u8,
    ack_count: u8,
    retention: Duration,
    future: GlobalRef,
) {
    let options = StreamOptions {
        replica: replica,
        ack: ack_count,
        retention: retention,
    };
    let result = front_end.create(options).await;
    match result {
        Ok(stream_id) => {
            complete_future_with_jlong(future, stream_id as i64);
        }
        Err(err) => {
            complete_future_with_error(future, err.to_string());
        }
    };
}
/// # Safety
///
/// This function could be only called by java vm when onload this lib.
#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    let java_vm = Arc::new(vm);
    let (tx, mut rx) = mpsc::unbounded_channel();
    if let Err(_) = unsafe { TX.set(tx) } {
        error!("Failed to set command channel sender");
    }
    let _ = std::thread::Builder::new()
        .name("Runtime".to_string())
        .spawn(move || {
            JENV.with(|cell| {
                if let Ok(env) = java_vm.attach_current_thread_as_daemon() {
                    *cell.borrow_mut() = Some(env.get_raw());
                } else {
                    error!("Failed to attach current thread as daemon");
                }
            });
            JAVA_VM.with(|cell| {
                *cell.borrow_mut() = Some(java_vm.clone());
            });
            tokio_uring::start(async move {
                loop {
                    match rx.recv().await {
                        Some(cmd) => {
                            tokio_uring::spawn(async move { process_command(cmd).await });
                        }
                        None => {
                            info!("JNI command channel is dropped");
                            break;
                        }
                    }
                }
            });
        });
    JNI_VERSION_1_8
}

/// # Safety
///
/// This function could be only called by java vm when unload this lib.
#[no_mangle]
pub extern "system" fn JNI_OnUnload(_: JavaVM, _: *mut c_void) {
    unsafe { TX.take() };
}

/// Bind to Frontend.getFrontend()
#[no_mangle]
pub extern "system" fn Java_com_automq_elasticstream_client_jni_Frontend_getFrontend(
    mut env: JNIEnv,
    _class: JClass,
    access_point: JString,
) -> jlong {
    let (tx_frontend, rx_frontend) = oneshot::channel();
    let command = env
        .get_string(&access_point)
        .map(|access_point| Command::GetFrontend {
            access_point: access_point.into(),
            tx: tx_frontend,
        });
    if let Ok(command) = command {
        match unsafe { TX.get() } {
            Some(tx) => match tx.send(command) {
                Ok(_) => match rx_frontend.blocking_recv() {
                    Ok(result) => match result {
                        Ok(frontend) => Box::into_raw(Box::new(frontend)) as jlong,
                        Err(err) => {
                            throw_exception(&mut env, &err.to_string());
                            0
                        }
                    },
                    Err(_) => {
                        error!("Failed to receive GetFrontend command response from tokio-uring runtime");
                        0
                    }
                },
                Err(_) => {
                    error!("Failed to dispatch GetFrontend command to tokio-uring runtime");
                    0
                }
            },
            None => {
                info!("JNI command channel was dropped. Ignore a GetFrontend request");
                0
            }
        }
    } else {
        info!("Failed to construct GetFrontend command. Ignore a GetFrontend request");
        0
    }
}

/// Bind to Frontend.freeFrontend()
#[no_mangle]
pub extern "system" fn Java_com_automq_elasticstream_client_jni_Frontend_freeFrontend(
    mut _env: JNIEnv,
    _class: JClass,
    ptr: *mut Frontend,
) {
    // Take ownership of the pointer by wrapping it with a Box
    let _ = unsafe { Box::from_raw(ptr) };
}
/// Bind to Frontend.create(replica, ack, retention_millis)
#[no_mangle]
pub extern "system" fn Java_com_automq_elasticstream_client_jni_Frontend_create(
    mut env: JNIEnv,
    _class: JClass,
    ptr: *mut Frontend,
    replica: jint,
    ack: jint,
    retention_millis: jlong,
    future: JObject,
) {
    let command = env.new_global_ref(future).map(|future| {
        let front_end = unsafe { &mut *ptr };
        Command::CreateStream {
            front_end: front_end,
            replica: replica as u8,
            ack_count: ack as u8,
            retention: Duration::from_millis(retention_millis as u64),
            future,
        }
    });
    if let Ok(command) = command {
        if let Some(tx) = unsafe { TX.get() } {
            if let Err(_e) = tx.send(command) {
                error!("Failed to dispatch CreateStream command to tokio-uring runtime");
            }
        } else {
            info!("JNI command channel was dropped. Ignore a CreateStream request");
        }
    } else {
        info!("Failed to construct CreateStream command");
    };
}
/// Bind to Frontend.open(id, epoch)
#[no_mangle]
pub extern "system" fn Java_com_automq_elasticstream_client_jni_Frontend_open(
    mut env: JNIEnv,
    _class: JClass,
    ptr: *mut Frontend,
    id: jlong,
    epoch: jlong,
    future: JObject,
) {
    debug_assert!(id >= 0, "Stream ID should be non-negative");
    let command = env.new_global_ref(future).map(|future| {
        let front_end = unsafe { &mut *ptr };
        Command::OpenStream {
            front_end,
            stream_id: id as u64,
            epoch: epoch as u64,
            future: future,
        }
    });
    if let Ok(command) = command {
        if let Some(tx) = unsafe { TX.get() } {
            if let Err(_e) = tx.send(command) {
                error!("Failed to dispatch OpenStream command to tokio-uring runtime");
            }
        } else {
            info!("JNI command channel was dropped. Ignore an OpenStream request");
        }
    } else {
        info!("Failed to construct OpenStream command");
    }
}
/// Bind to Stream.freeStream()
#[no_mangle]
pub extern "system" fn Java_com_automq_elasticstream_client_jni_Stream_freeStream(
    env: JNIEnv,
    _class: JClass,
    ptr: *mut Stream,
) {
    // Take ownership of the pointer by wrapping it with a Box
    let _ = unsafe { Box::from_raw(ptr) };
}
/// Bind to Stream.startOffset()
#[no_mangle]
pub extern "system" fn Java_com_automq_elasticstream_client_jni_Stream_startOffset(
    mut env: JNIEnv,
    _class: JClass,
    ptr: *mut Stream,
    future: JObject,
) {
    let command = env.new_global_ref(future).map(|future| {
        let stream = unsafe { &mut *ptr };
        Command::StartOffset {
            stream: stream,
            future: future,
        }
    });
    if let Ok(command) = command {
        if let Some(tx) = unsafe { TX.get() } {
            if let Err(_e) = tx.send(command) {
                error!("Failed to dispatch StartOffset command to tokio-uring runtime");
            }
        } else {
            info!("JNI command channel was dropped. Ignore a StartOffset request");
        }
    } else {
        info!("Failed to construct StartOffset command.");
    }
}
/// Bind to Stream.nextOffset()
#[no_mangle]
pub extern "system" fn Java_com_automq_elasticstream_client_jni_Stream_nextOffset(
    mut env: JNIEnv,
    _class: JClass,
    ptr: *mut Stream,
    future: JObject,
) {
    let command = env.new_global_ref(future).map(|future| {
        let stream = unsafe { &mut *ptr };
        Command::NextOffset {
            stream: stream,
            future: future,
        }
    });
    if let Ok(command) = command {
        if let Some(tx) = unsafe { TX.get() } {
            if let Err(_e) = tx.send(command) {
                error!("Failed to dispatch NextOffset command to tokio-uring runtime");
            }
        } else {
            info!("JNI command channel was dropped. Ignore a NextOffset request");
        }
    } else {
        info!("Failed to construct NextOffset command");
    }
}
/// Bind to Stream.append(data)
#[no_mangle]
pub extern "system" fn Java_com_automq_elasticstream_client_jni_Stream_append(
    mut env: JNIEnv,
    _class: JClass,
    ptr: *mut Stream,
    data: JObject,
    future: JObject,
) {
    let buf = env.get_direct_buffer_address((&data).into());
    let len = env.get_direct_buffer_capacity((&data).into());
    let future = env.new_global_ref(future);
    let command: Result<Command, ()> = match (buf, len, future) {
        (Ok(buf), Ok(len), Ok(future)) => {
            let stream = unsafe { &mut *ptr };
            let buf = Bytes::copy_from_slice(unsafe { slice::from_raw_parts(buf, len) });
            Ok(Command::Append {
                stream,
                buf,
                future,
            })
        }
        _ => Err(()),
    };
    if let Ok(command) = command {
        if let Some(tx) = unsafe { TX.get() } {
            if let Err(_e) = tx.send(command) {
                error!("Failed to dispatch Append command to tokio-uring runtime");
            }
        } else {
            info!("JNI command channel was dropped. Ignore an Append request");
        }
    } else {
        info!("Failed to construct Append command");
    }
}

/// Bind to Stream.read(start_offset, end_offset, batch_max_bytes)
#[no_mangle]
pub extern "system" fn Java_com_automq_elasticstream_client_jni_Stream_read(
    env: JNIEnv,
    _class: JClass,
    ptr: *mut Stream,
    start_offset: jlong,
    end_offset: jlong,
    batch_max_bytes: jint,
    future: JObject,
) {
    let command = env.new_global_ref(future).map(|future| {
        let stream = unsafe { &mut *ptr };
        Command::Read {
            stream: stream,
            start_offset: start_offset,
            end_offset: end_offset,
            batch_max_bytes: batch_max_bytes,
            future: future,
        }
    });
    if let Ok(command) = command {
        if let Some(tx) = unsafe { TX.get() } {
            if let Err(_e) = tx.send(command) {
                error!("Failed to dispatch Read command to tokio-uring runtime");
            }
        } else {
            info!("JNI command channel was dropped. Ignore a Read request");
        }
    } else {
        info!("Failed to construct Read command");
    }
}
/// Call Java future complete() method and put an object as argument
///
/// # Arguments
/// `env` - JNIEnv struct
/// `future` - Java future
/// `obj` - Java Object
/// # Returns
///
fn call_future_complete_method(mut env: JNIEnv, future: GlobalRef, obj: JObject) {
    let s = JValueGen::from(obj);
    if let Err(_) = env.call_method(future, "complete", "(Ljava/lang/Object;)Z", &[s.borrow()]) {
        error!("Failed to call future complete method");
    }
}
/// Call Java future completeExceptionally() method
///
/// # Arguments
/// `env` - JNIEnv struct
/// `future` - Java future
/// `err_msg` - Error message that used to create exception object
/// # Returns
///
fn call_future_complete_exceptionally_method(env: &mut JNIEnv, future: GlobalRef, err_msg: String) {
    let exception_class = env.find_class("java/lang/Exception");
    let message = env.new_string(err_msg);
    if let (Ok(exception_class), Ok(message)) = (exception_class, message) {
        let obj = env.new_object(
            exception_class,
            "(Ljava/lang/String;)V",
            &[JValue::Object(message.as_ref())],
        );
        if let Ok(obj) = obj {
            let s = JValueGen::from(obj);
            if let Err(_) = env.call_method(
                future,
                "completeExceptionally",
                "(Ljava/lang/Throwable;)Z",
                &[s.borrow()],
            ) {
                error!("Failed to call future completeExceptionally method");
            }
        } else {
            error!("Failed to create exception object");
        }
    } else {
        error!("Failed to get exception_class or error_message");
    }
}

/// Get thread local JNIEnv that used to call future method
///
/// # Arguments
/// `cell` -
/// # Returns
/// thread local JNIEnv
fn get_thread_local_jenv(cell: &RefCell<Option<*mut *const JNINativeInterface_>>) -> JNIEnv {
    let env_ptr = cell
        .borrow()
        .expect("Couldn't get raw ptr of thread local jenv");
    unsafe { JNIEnv::from_raw(env_ptr).expect("Couldn't create a JNIEnv from raw pointer") }
}

/// Throw an java exception
///
/// # Arguments
/// `env` - Current JNIEnv struct
/// `msg` - Error message
/// # Returns
///
fn throw_exception(env: &mut JNIEnv, msg: &str) {
    let _ = env.exception_clear();
    if let Err(_) = env.throw_new("java/lang/Exception", msg) {
        error!("Failed to throw new exception");
    }
}

/// Call Java future complete() method and put a "com/automq/elasticstream/client/jni/Stream" object as argument
///
/// # Arguments
/// `future` - Java future
/// `ptr` - The memory address of the stream
/// # Returns
///
fn complete_future_with_stream(future: GlobalRef, ptr: i64) {
    JENV.with(|cell| {
        let mut env = get_thread_local_jenv(cell);
        let class_name = "com/automq/elasticstream/client/jni/Stream";
        if let Ok(stream_class) = env.find_class(class_name) {
            if let Ok(obj) =
                env.new_object(stream_class, "(J)V", &[jni::objects::JValueGen::Long(ptr)])
            {
                call_future_complete_method(env, future, obj);
            } else {
                error!("Couldn't create {} object", class_name);
            }
        } else {
            error!("Couldn't find {} class", class_name);
        }
    });
}

/// Call Java future complete() method and put a "java/lang/Long" object as argument
///
/// # Arguments
/// `future` - Java future
/// `value` - The value of argument
/// # Returns
///
fn complete_future_with_jlong(future: GlobalRef, value: i64) {
    JENV.with(|cell| {
        let mut env = get_thread_local_jenv(cell);
        let class_name = "java/lang/Long";
        if let Ok(long_class) = env.find_class(class_name) {
            if let Ok(obj) =
                env.new_object(long_class, "(J)V", &[jni::objects::JValueGen::Long(value)])
            {
                call_future_complete_method(env, future, obj);
            } else {
                error!("Failed to create {} object", class_name);
            }
        } else {
            error!("Failed to find {} class", class_name);
        }
    });
}
/// Call Java future completeExceptionally() method
///
/// # Arguments
/// `future` - Java future
/// `err_msg` - Error message that used to create exception object
/// # Returns
///
fn complete_future_with_error(future: GlobalRef, err_msg: String) {
    JENV.with(|cell| {
        let mut env = get_thread_local_jenv(cell);
        call_future_complete_exceptionally_method(&mut env, future, err_msg);
    });
}
