use jni::objects::{JByteArray, JClass, JObject, JString, JValue, JValueGen};
use jni::sys::{jint, jlong, jstring, JNI_VERSION_1_8};
use jni::{JNIEnv, JavaVM};
use std::cell::{OnceCell, RefCell};
use std::ffi::c_void;
use std::io::IoSlice;
use std::slice::from_raw_parts;
use std::sync::Arc;

use tokio::runtime::{Builder, Runtime};

use crate::{Stream, StreamManager, StreamOptions};

static mut RUNTIME: OnceCell<Runtime> = OnceCell::new();

thread_local! {
    static JAVA_VM: RefCell<Option<Arc<JavaVM>>> = RefCell::new(None);
    static JENV: RefCell<Option<*mut jni::sys::JNIEnv>> = RefCell::new(None);
}

#[no_mangle]
pub unsafe extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    // TODO: make this configurable in the future
    let thread_count = num_cpus::get();
    let java_vm = Arc::new(vm);
    let runtime = Builder::new_multi_thread()
        .worker_threads(thread_count)
        .on_thread_start(move || {
            JENV.with(|cell| {
                let env = java_vm.attach_current_thread_as_daemon().unwrap();
                *cell.borrow_mut() = Some(env.get_raw());
            });
            JAVA_VM.with(|cell| {
                *cell.borrow_mut() = Some(java_vm.clone());
            });
        })
        .on_thread_stop(move || {
            JENV.with(|cell| {
                *cell.borrow_mut() = None;
            });
            JAVA_VM.with(|cell| unsafe {
                if let Some(vm) = cell.borrow_mut().take() {
                    vm.detach_current_thread();
                }
            });
        })
        .enable_time()
        .build()
        .unwrap();
    RUNTIME.set(runtime).unwrap();
    JNI_VERSION_1_8
}

/// # Safety
///
/// This function could be only called by java vm when unload this lib.
#[no_mangle]
pub unsafe extern "system" fn JNI_OnUnload(_: JavaVM, _: *mut c_void) {
    if let Some(runtime) = RUNTIME.take() {
        runtime.shutdown_background();
    }
}

/// StreamManager
#[no_mangle]
pub unsafe extern "system" fn Java_sdk_elastic_stream_jni_StreamManager_getStreamManager(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let stream_manager = StreamManager::new();
    Box::into_raw(Box::new(stream_manager)) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_sdk_elastic_stream_jni_StreamManager_freeStreamManager(
    mut _env: JNIEnv,
    _class: JClass,
    ptr: *mut StreamManager,
) {
    // Take ownership of the pointer by wrapping it with a Box
    let _ = Box::from_raw(ptr);
}

#[no_mangle]
pub unsafe extern "system" fn Java_sdk_elastic_stream_jni_StreamManager_create(
    env: JNIEnv,
    _class: JClass,
    ptr: *mut StreamManager,
    replica: jint,
    future: JObject,
) {
    let stream_manager = &mut *ptr;
    let options = StreamOptions::new(replica.try_into().unwrap());
    let future = env.new_global_ref(future).unwrap();
    RUNTIME.get().unwrap().spawn(async move {
        let result = stream_manager.create(options).await;
        match result {
            Ok(stream) => {
                JENV.with(|cell| {
                    let ptr = Box::into_raw(Box::new(stream)) as jlong;
                    let env_ptr = cell.borrow().unwrap();
                    let mut env = JNIEnv::from_raw(env_ptr).unwrap();
                    let stream_class = env.find_class("sdk/elastic/stream/jni/Stream").unwrap();
                    let obj = env
                        .new_object(stream_class, "(J)V", &[jni::objects::JValueGen::Long(ptr)])
                        .unwrap();
                    let s = JValueGen::from(obj);
                    let _ = env
                        .call_method(future, "complete", "(Ljava/lang/Object;)Z", &[s.borrow()])
                        .unwrap();
                });
            }
            Err(err) => {
                // TODO
                JENV.with(|cell| {
                    let env_ptr = cell.borrow().unwrap();
                    let mut env = JNIEnv::from_raw(env_ptr).unwrap();

                    let exception_class = env.find_class("java/lang/Exception").unwrap();
                    let message = env.new_string("a demo exception for test").unwrap();
                    let obj = env
                        .new_object(
                            exception_class,
                            "(Ljava/lang/String;)V",
                            &[JValue::Object(message.as_ref())],
                        )
                        .unwrap();
                    let s = JValueGen::from(obj);
                    let _ = env
                        .call_method(
                            future,
                            "completeExceptionally",
                            "(Ljava/lang/Throwable;)Z",
                            &[s.borrow()],
                        )
                        .unwrap();
                });
            }
        };
    });
}
#[no_mangle]
pub unsafe extern "system" fn Java_sdk_elastic_stream_jni_StreamManager_open(
    mut env: JNIEnv,
    _class: JClass,
    ptr: *mut StreamManager,
    id: jlong,
    future: JObject,
) {
    let stream_manager = &mut *ptr;
    let future = env.new_global_ref(future).unwrap();
    RUNTIME.get().unwrap().spawn(async move {
        let result = stream_manager.open(id).await;
        match result {
            Ok(stream) => {
                JENV.with(|cell| {
                    let ptr = Box::into_raw(Box::new(stream)) as jlong;
                    let env_ptr = cell.borrow().unwrap();
                    let mut env = JNIEnv::from_raw(env_ptr).unwrap();
                    let stream_class = env.find_class("sdk/elastic/stream/jni/Stream").unwrap();
                    let obj = env
                        .new_object(stream_class, "(J)V", &[jni::objects::JValueGen::Long(ptr)])
                        .unwrap();
                    let s = JValueGen::from(obj);
                    let _ = env
                        .call_method(future, "complete", "(Ljava/lang/Object;)Z", &[s.borrow()])
                        .unwrap();
                });
            }
            Err(_) => {
                todo!();
            }
        };
    });
}

// Stream

#[no_mangle]
pub unsafe extern "system" fn Java_sdk_elastic_stream_jni_Stream_freeStream(
    mut env: JNIEnv,
    _class: JClass,
    ptr: *mut Stream,
) {
    // Take ownership of the pointer by wrapping it with a Box
    let _ = Box::from_raw(ptr);
}

#[no_mangle]
pub unsafe extern "system" fn Java_sdk_elastic_stream_jni_Stream_minOffset(
    mut env: JNIEnv,
    _class: JClass,
    ptr: *mut Stream,
    future: JObject,
) {
    let stream = &mut *ptr;
    let future = env.new_global_ref(future).unwrap();
    RUNTIME.get().unwrap().spawn(async move {
        let result = stream.min_offset().await;
        match result {
            Ok(offset) => {
                JENV.with(|cell| {
                    let env_ptr = cell.borrow().unwrap();
                    let mut env = JNIEnv::from_raw(env_ptr).unwrap();
                    let long_class = env.find_class("java/lang/Long").unwrap();
                    let obj = env
                        .new_object(long_class, "(J)V", &[jni::objects::JValueGen::Long(offset)])
                        .unwrap();
                    let s = JValueGen::from(obj);
                    let _ = env
                        .call_method(future, "complete", "(Ljava/lang/Object;)Z", &[s.borrow()])
                        .unwrap();
                });
            }
            Err(_) => {
                todo!();
            }
        };
    });
}

#[no_mangle]
pub unsafe extern "system" fn Java_sdk_elastic_stream_jni_Stream_maxOffset(
    mut env: JNIEnv,
    _class: JClass,
    ptr: *mut Stream,
    future: JObject,
) {
    let stream = &mut *ptr;
    let future = env.new_global_ref(future).unwrap();
    RUNTIME.get().unwrap().spawn(async move {
        let result = stream.max_offset().await;
        match result {
            Ok(offset) => {
                JENV.with(|cell| {
                    let env_ptr = cell.borrow().unwrap();
                    let mut env = JNIEnv::from_raw(env_ptr).unwrap();
                    let long_class = env.find_class("java/lang/Long").unwrap();
                    let obj = env
                        .new_object(long_class, "(J)V", &[jni::objects::JValueGen::Long(offset)])
                        .unwrap();
                    let s = JValueGen::from(obj);
                    let _ = env
                        .call_method(future, "complete", "(Ljava/lang/Object;)Z", &[s.borrow()])
                        .unwrap();
                });
            }
            Err(_) => {
                todo!();
            }
        };
    });
}

#[no_mangle]
pub unsafe extern "system" fn Java_sdk_elastic_stream_jni_Stream_append(
    mut env: JNIEnv,
    _class: JClass,
    ptr: *mut Stream,
    mut data: JByteArray,
    future: JObject,
) {
    let stream = &mut *ptr;
    let future = env.new_global_ref(future).unwrap();
    let array = env
        .get_array_elements(&data, jni::objects::ReleaseMode::NoCopyBack)
        .unwrap();
    let len = env.get_array_length(&data).unwrap();
    let slice = unsafe { from_raw_parts(array.as_ptr() as *mut u8, len.try_into().unwrap()) };
    RUNTIME.get().unwrap().spawn(async move {
        let result = stream.append(IoSlice::new(slice)).await;
        match result {
            Ok(result) => {
                let base_offset = result.base_offset;
                JENV.with(|cell| {
                    let env_ptr = cell.borrow().unwrap();
                    let mut env = JNIEnv::from_raw(env_ptr).unwrap();
                    let long_class = env.find_class("java/lang/Long").unwrap();
                    let obj = env
                        .new_object(
                            long_class,
                            "(J)V",
                            &[jni::objects::JValueGen::Long(base_offset)],
                        )
                        .unwrap();
                    let s = JValueGen::from(obj);
                    let _ = env
                        .call_method(future, "complete", "(Ljava/lang/Object;)Z", &[s.borrow()])
                        .unwrap();
                });
            }
            Err(_) => {
                todo!();
            }
        };
    });
}

#[no_mangle]
pub unsafe extern "system" fn Java_sdk_elastic_stream_jni_Stream_read(
    mut env: JNIEnv,
    _class: JClass,
    ptr: *mut Stream,
    offset: jlong,
    limit: jint,
    max_bytes: jint,
    future: JObject,
) {
    let stream = &mut *ptr;
    let future = env.new_global_ref(future).unwrap();
    RUNTIME.get().unwrap().spawn(async move {
        let result = stream.read(offset, limit, max_bytes).await;
        match result {
            Ok(result) => {
                let result: &[u8] = result.as_ref();
                JENV.with(|cell| {
                    let env_ptr = cell.borrow().unwrap();
                    let mut env = JNIEnv::from_raw(env_ptr).unwrap();
                    let output = env.byte_array_from_slice(&result).unwrap();
                    let s = JValueGen::from(JObject::from(output));
                    let _ = env
                        .call_method(future, "complete", "(Ljava/lang/Object;)Z", &[s.borrow()])
                        .unwrap();
                });
            }
            Err(_) => {
                todo!();
            }
        };
    });
}
