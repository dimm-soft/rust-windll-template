#[cfg(not(target_os = "windows"))]
compile_error!("this only works for windows");

use winapi::shared::minwindef;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::consoleapi;

use std::process;

#[no_mangle] // call it "DllMain" in the compiled DLL
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: DWORD,
    reserved: LPVOID)
    -> BOOL
{
    match call_reason {
        winapi::um::winnt::DLL_PROCESS_ATTACH => dll_attach(),
        winapi::um::winnt::DLL_PROCESS_DETACH => dll_detach(),
        _ => ()
    }
    minwindef::TRUE
}

// Run on attachment of the Rust DLL.
fn dll_attach() {
    unsafe { consoleapi::AllocConsole() };

    let pid = process::id();
    println!("attachment of the Rust DLL");
    println!("the PID is {}", pid);
}
 
/// Run on the detachment of the Rust DLL.
fn dll_detach() {  
    println!("detachment of the Rust DLL");
}

#[no_mangle]
pub extern "C" fn test_fn() {
    println!("Hello world from Rust!")
}