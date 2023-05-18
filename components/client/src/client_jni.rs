use std::cell::{OnceCell, RefCell};
use std::ffi::c_void;
use std::sync::Arc;

// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::{JNIEnv, JavaVM};

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JByteArray, JClass, JObject, JString, JValueGen};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jint, jstring, JNI_VERSION_1_8};
use tokio::runtime::{Builder, Runtime};

static mut RUNTIME: OnceCell<Runtime> = OnceCell::new();

thread_local! {
    static JAVA_VM: RefCell<Option<Arc<JavaVM>>> = RefCell::new(None);
    static JENV: RefCell<Option<*mut jni::sys::JNIEnv>> = RefCell::new(None);
}

#[no_mangle]
pub unsafe extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    // TODO: make this configurable in the future
    let thread_count = 4;
    println!("Hello From OnLoad");

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
        // async_func().await
        // do async function

        // send the reponse(serialize it)
        JENV.with(|cell| {
            let env_ptr = cell.borrow().unwrap();
            let mut env = JNIEnv::from_raw(env_ptr).unwrap();

            // build result
            let buf = [5, 4, 3, 6];
            let output = env.byte_array_from_slice(&buf).unwrap();
            let s = JValueGen::from(JObject::from(output));
            let _ = env
                .call_method(future, "complete", "(Ljava/lang/Object;)Z", &[s.borrow()])
                .unwrap();
        });
    };
    RUNTIME.get().unwrap().spawn(x);
}
