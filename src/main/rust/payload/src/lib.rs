use std::ffi::CStr;
use std::mem::transmute;
use std::ptr::null_mut;
use jni::sys::{jint, jsize};
use winapi::shared::minwindef::{DWORD, FARPROC, HINSTANCE, LPVOID};
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::processthreadsapi::CreateThread;

/*
 * Type alias for the external JNI C function 'JNI_GetCreatedJavaVMs'.
 */
#[allow(non_snake_case)]
type JNIGetCreatedJavaVMs = unsafe extern "system" fn(vmBuf: *mut *mut jni::sys::JavaVM,
                                                      bufLen: jsize,
                                                      nVMs: *mut jsize) -> jint;

/**
 * DllMain is the first method called when the dynamic link library successfully attaches to the VM.
 */
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn DllMain(_dll_instance: HINSTANCE,
                                       fdw_reason: DWORD,
                                       _reserved: LPVOID) -> i32
{
    match fdw_reason
    {
        winapi::um::winnt::DLL_PROCESS_ATTACH => {
            // If the reason for the function call is DLL_PROCESS_ATTACH, we know the remote attachment succeeded.
            // Here, we can create and run our own async thread on the target program.
            CreateThread(
                null_mut(),
                0,
                Some(target_injection_function), // pointer to our target function
                null_mut(),
                0,
                null_mut()
            );
        }
        _ => ()
    }

    return true as i32;
}

/*
 * Our function that we intend to attach to the remote process.
 */
#[allow(non_snake_case)]
pub unsafe extern "system" fn target_injection_function(_lpvoid: LPVOID) -> u32
{
    let jvm_dll = CStr::from_bytes_with_nul(b"jvm.dll\0").unwrap();
    let jvm_dll = GetModuleHandleA(jvm_dll.as_ptr());

    let get_java_vms = CStr::from_bytes_with_nul(b"JNI_GetCreatedJavaVMs\0").unwrap();
    let get_java_vms = GetProcAddress(jvm_dll, get_java_vms.as_ptr());

    let get_java_vms_fn: JNIGetCreatedJavaVMs = transmute::<FARPROC, JNIGetCreatedJavaVMs>(get_java_vms);

    let mut buffer: Vec<*mut jni::sys::JavaVM> = Vec::with_capacity(1);
    buffer.push(null_mut());

    get_java_vms_fn(buffer.as_mut_ptr(), 1, null_mut());

    let sys_jvm = buffer.pop().unwrap();
    let jvm = jni::JavaVM::from_raw(sys_jvm).unwrap();

    // We now have a usable JNIEnv attached to our target process.

    // We can use this to dynamically load classes into the target process, call methods, rewrite memory,
    // and essentially have full control over the process remotely.

    // But in this case, we will just print a message :)

    let env = jvm.attach_current_thread_permanently().unwrap();

    let java_lang_System_out = env.get_static_field(
        env.find_class("java/lang/System").unwrap(),
        "out",
        "Ljava/io/PrintStream;"
    ).unwrap().l().unwrap();

    env.call_method(
        java_lang_System_out,
        "println",
        "(Ljava/lang/String;)V",
        &[env.new_string("\nRemote thread successfully injected").unwrap().into()]
    ).unwrap().v().unwrap();

    return 0;
}

