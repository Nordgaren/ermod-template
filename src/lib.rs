#![allow(non_snake_case)]

use dll_proxy::proxy_dll;
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
            1
        },
        DLL_PROCESS_DETACH => {
            1
        }
        _ => 0,
    }
}

