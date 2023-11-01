use std::ffi::{c_void, CStr};
use std::mem::transmute;
use std::ptr::null_mut;
use jni::{JavaVM, JNIEnv};
use jni::objects::{JClass, JString};
use jni::sys::{jint, JNI_VERSION_1_8};
use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::um::handleapi::CloseHandle;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
use winapi::um::processthreadsapi::{CreateRemoteThread, OpenProcess};
use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PROCESS_ALL_ACCESS};

/*
 * Called when System.load() is invoked on the Java side
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn JNI_OnLoad(_vm: JavaVM,
                                  _reserved: *mut c_void) -> jint
{
    println!("Injection library successfully loaded");
    JNI_VERSION_1_8
}

/*
 * Native implementation of me/bonasera/nativeinjector/Main#inject.
 */
#[no_mangle]
pub unsafe extern "system" fn Java_me_bonasera_nativeinjector_Main_inject(env: JNIEnv,
                                                                          _class: JClass,
                                                                          pid: jint,
                                                                          loader_path: JString)
{
    // Open a handle to the desired process
    let handle = OpenProcess(
        PROCESS_ALL_ACCESS,
        FALSE,
        pid as DWORD
    );

    let loader_path_internal = env.get_string(loader_path).unwrap();

    // Allocate memory in the target program to store the path of the DLL
    let dll_addr = VirtualAllocEx(
        handle,
        null_mut(),
        loader_path_internal.to_bytes_with_nul().len() + 1,
        MEM_COMMIT | MEM_RESERVE,
        PAGE_EXECUTE_READWRITE
    );

    // Write the path to the DLL to the open memory
    WriteProcessMemory(
        handle,
        dll_addr,
        loader_path_internal.as_ptr() as *mut _,
        loader_path_internal.to_bytes_with_nul().len() + 1,
        null_mut()
    );

    // Get a handle to kernel32.dll
    let krnl32 = CStr::from_bytes_with_nul(b"kernel32.dll\0").unwrap();
    let krnl32 = GetModuleHandleA(krnl32.as_ptr());

    // Get the address of the LoadLibraryA function.
    let loadlib = CStr::from_bytes_with_nul(b"LoadLibraryA\0").unwrap();
    let loadlib = GetProcAddress(krnl32, loadlib.as_ptr());

    // Create a remote thread that calls LoadLibraryA with the DLL path, loading the binary into the process.
    let thread = CreateRemoteThread(
        handle,
        null_mut(),
        0,
        Some(transmute(loadlib)),
        dll_addr,
        0,
        null_mut(),
    );

    println!("\nSuccessfully spawned remote thread.",);

    CloseHandle(thread);
}

