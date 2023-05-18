use jni::objects::{JByteArray, JClass, JObject, JString, JValueGen};
use jni::sys::{jint, jlong, jstring, JNI_VERSION_1_8};
use jni::{JNIEnv, JavaVM};
use std::cell::{OnceCell, RefCell};
use std::ffi::c_void;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use tokio::time::sleep;

use crate::client::Client;
use crate::session_manager::SessionManager;

static mut RUNTIME: OnceCell<Runtime> = OnceCell::new();

thread_local! {
    static JAVA_VM: RefCell<Option<Arc<JavaVM>>> = RefCell::new(None);
    static JENV: RefCell<Option<*mut jni::sys::JNIEnv>> = RefCell::new(None);
}

#[no_mangle]
pub unsafe extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    // TODO: make this configurable in the future
    let thread_count = num_cpus::get();
    println!("thread_count: {}", thread_count);

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

#[no_mangle]
pub unsafe extern "system" fn Java_ClientJNI_asyncComputation(
    env: JNIEnv,
    _class: JClass,
    serialized_req: JByteArray,
    future: JObject,
) {
    let serialized_req = env.convert_byte_array(&serialized_req).unwrap();
    println!("req = {:?}", serialized_req);
    let future = env.new_global_ref(future).unwrap();
    let x = async move {
        sleep(Duration::from_secs(1)).await;
        JENV.with(|cell| {
            let env_ptr = cell.borrow().unwrap();
            let mut env = JNIEnv::from_raw(env_ptr).unwrap();
            // build result
            let buf = [1, 2, 3, 4];
            let output = env.byte_array_from_slice(&buf).unwrap();
            let s = JValueGen::from(JObject::from(output));
            let _ = env
                .call_method(future, "complete", "(Ljava/lang/Object;)Z", &[s.borrow()])
                .unwrap();
        });
    };
    RUNTIME.get().unwrap().spawn(x);
}

#[no_mangle]
pub unsafe extern "system" fn Java_ClientJNI_getClient(env: JNIEnv, _class: JClass) -> jlong {
    let client = Client::new();
    Box::into_raw(Box::new(client)) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_ClientJNI_freeClient(
    mut _env: JNIEnv,
    _class: JClass,
    ptr: *mut Client,
) {
    // Take ownership of the pointer by wrapping it with a Box
    let _ = Box::from_raw(ptr);
}

#[no_mangle]
pub unsafe extern "system" fn Java_ClientJNI_createStream(
    mut _env: JNIEnv,
    _class: JClass,
    ptr: *mut Client,
) {
    let client = &mut *ptr;
    let _ = RUNTIME.get().unwrap().block_on(client.create_stream());
    println!("end of Java_ClientJNI_createStream");
}

#[no_mangle]
pub unsafe extern "system" fn Java_ClientJNI_asyncCreateStream(
    env: JNIEnv,
    _class: JClass,
    ptr: *mut Client,
    future: JObject,
) {
    let client = &mut *ptr;
    let future = env.new_global_ref(future).unwrap();
    let x = async move {
        let _ = client.create_stream().await;
        JENV.with(|cell| {
            let env_ptr = cell.borrow().unwrap();
            let mut env = JNIEnv::from_raw(env_ptr).unwrap();
            // build result
            let buf = [1, 2, 3, 4];
            let output = env.byte_array_from_slice(&buf).unwrap();
            let s = JValueGen::from(JObject::from(output));
            let _ = env
                .call_method(future, "complete", "(Ljava/lang/Object;)Z", &[s.borrow()])
                .unwrap();
        });
    };
    RUNTIME.get().unwrap().spawn(x);
    println!("end of Java_ClientJNI_asyncCreateStream");
}
