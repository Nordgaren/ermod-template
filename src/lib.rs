#![allow(non_snake_case)]

mod modengine;

use std::cell::OnceCell;
use std::ffi::c_void;
use dll_proxy::proxy_dll;
use crate::modengine::ModEngine2Extension;

proxy_dll!("dinput8.dll");

const DLL_PROCESS_ATTACH: u32 = 1;
const DLL_PROCESS_DETACH: u32 = 0;

#[link(name = "kernel32", kind = "raw-dylib")]
#[cfg(feature = "Console")]
extern "system" {
    pub fn AllocConsole() -> u32;
    pub fn AttachConsole(dwProcessId: u32) -> u32;
}

#[no_mangle]
#[allow(unused)]
pub extern "stdcall" fn DllMain(hinstDLL: usize, dwReason: u32, lpReserved: *mut usize) -> i32 {
    match dwReason {
        DLL_PROCESS_ATTACH => unsafe {
            #[cfg(feature = "Console")]
            {
                AllocConsole();
                AttachConsole(u32::MAX);
            }
            let path = match init_proxy(hinstDLL) {
                Ok(p) => p,
                Err(e) => panic!("Could not proxy dll: {e}"),
            };
            println!("DLL Proxy started");
            1
        },
        DLL_PROCESS_DETACH => {
            1
        }
        _ => 0,
    }
}

static mut EXTENSION: OnceCell<ModEngine2Extension> = OnceCell::new();
#[unsafe(no_mangle)]
pub unsafe extern "C" fn modengine_ext_init(
    _connector: *const c_void,
    extension: *mut *mut ModEngine2Extension,
) -> bool {
    println!("modengine_ext_init internal");
    EXTENSION.get_or_init(ModEngine2Extension::default);
    *extension = EXTENSION.get_mut().unwrap();

    true
}




