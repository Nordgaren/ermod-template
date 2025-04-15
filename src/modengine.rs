use device_query::{DeviceQuery, DeviceState, Keycode};
use eldenring::cs::{CSTaskGroupIndex, CSTaskImp, WorldChrMan};
use eldenring_util::program::Program;
use eldenring_util::singleton::LookupError;
use eldenring_util::task::CSTaskImpExt;
use modengine2_rs::ModEngine2ExtVmt;
use std::ffi::{c_char, CString};
use std::process::Command;
use std::time::Duration;
use vtable_rs::VPtr;

#[repr(C)]
pub struct ModEngine2Extension {
    vftable: VPtr<dyn ModEngine2ExtVmt, Self>,
    id: CString,
}

impl Default for ModEngine2Extension {
    fn default() -> Self {
        Self {
            vftable: VPtr::default(),
            id: CString::new("ermod").expect("Could not make C string from package name."),
        }
    }
}

impl ModEngine2ExtVmt for ModEngine2Extension {
    extern "C" fn on_attach(&self) {
        std::thread::spawn(|| {
            eldenring_util::system::wait_for_system_init(
                &Program::current(),
                Duration::from_secs(60),
            ).expect("Could not wait for system init");

            let device_state = DeviceState::new();

            let task = eldenring_util::task::RecurringTask::new(move |a| {
                let keys = device_state.get_keys();

                if !keys.contains(&Keycode::O) {
                    return;
                }

                match unsafe { eldenring_util::singleton::get_instance::<WorldChrMan>() } {
                    Ok(w) => match w {
                        Some(wcm) => {
                            println!(
                                "Count: {}",
                                wcm.open_field_chr_set.base.characters().count()
                            );
                        }
                        _ => {}
                    },
                    _ => {}
                };
            });

            let csTaskImp = unsafe { eldenring_util::singleton::get_instance::<CSTaskImp>() }
                .expect("Could not get CSTaskImp Static")
                .expect("Could not get CSTaskImp Instance");


            csTaskImp.run_recurring(task, CSTaskGroupIndex::GameMan);
        });
    }
    extern "C" fn id(&self) -> *const c_char {
        self.id.as_ptr()
    }
}
