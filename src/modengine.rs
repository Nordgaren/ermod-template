use modengine2_ext::ModEngine2Extension;
use std::time::Duration;
use eldenring::cs::{CSTaskGroupIndex, CSTaskImp, WorldChrMan};
use eldenring_util::{
    program::Program,
    task::CSTaskImpExt
};

pub fn on_attach(_this: &ModEngine2Extension) {
    println!("on_attach!");
    std::thread::spawn(|| {
        eldenring_util::system::wait_for_system_init(&Program::current(), Duration::from_secs(60))
            .expect("Could not wait for system init");

        let task = eldenring_util::task::RecurringTask::new(move |_| {

            let wcm =
                unsafe { eldenring_util::singleton::get_instance::<WorldChrMan>() }.and_then(|c| {
                    c.and_then(|c| {
                        println!("Count: {}", c.open_field_chr_set.base.characters().count());
                        Some(())
                    });

                    Ok(())
                });

            match wcm {
                Ok(_) => {}
                _ => unreachable!(),
            };
        });

        let csTaskImp = unsafe { eldenring_util::singleton::get_instance::<CSTaskImp>() }
            .expect("Could not get CSTaskImp Static")
            .expect("Could not get CSTaskImp Instance");

        csTaskImp.run_recurring(task, CSTaskGroupIndex::GameMan);
    });
}

pub(crate) fn init() {
    modengine2_ext::init(on_attach, None, None);
}