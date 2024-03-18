use std::{
    ffi::{c_char, CString},
    ptr::NonNull,
};

use ndk::binder::IBinder;
use ndk_sys::AIBinder;
use rustix::fd::AsFd;

type AServiceManager_checkService = extern "system" fn(instance: *const c_char) -> *mut AIBinder;

fn main() {
    let binder = unsafe { libloading::Library::new("libbinder_ndk.so").unwrap() };
    let service_manager_check_service =
        unsafe { binder.get::<AServiceManager_checkService>(b"AServiceManager_checkService\0") }
            .unwrap();

    let names = [
        // "power",
        "power.stats-vendor",
        "powerstats", // https://cs.android.com/android/platform/superproject/main/+/main:frameworks/base/services/core/java/com/android/server/powerstats/PowerStatsService.java, https://cs.android.com/android/platform/superproject/main/+/main:frameworks/base/services/core/java/com/android/server/powerstats/PowerStatsService.java;l=295;drc=15978b577cc30ad2c5b8f5e9d1cd5986ea7f09ae;bpv=1;bpt=1
        "android.hardware.power.stats.IPowerStats/default", // https://android.googlesource.com/device/google/pantah/+/3c30a32/powerstats/panther/service.cpp#88
        "android.hardware.power.IPower/default",
        // Doesn't exist:
        "power.stats", // Could be this mock? https://android.googlesource.com/platform/hardware/interfaces/+/refs/heads/main/power/stats/1.0/default/service.cpp
    ];

    for name in names {
        let name = CString::new(name).unwrap();
        // let service = unsafe { AServiceManager_checkService(name.as_ptr()) };
        let service = service_manager_check_service(name.as_ptr());
        let Some(binder) = NonNull::new(service) else {
            continue;
        };
        let binder = unsafe { IBinder::from_ptr(binder) };
        // dbg!(binder.class());
        dbg!(&name, &binder);
        dbg!(binder.is_alive());
        dbg!(binder.is_remote());

        let (r, w) = rustix::pipe::pipe().unwrap();

        let dump = std::thread::spawn(move || {
            let mut dump = String::new();
            let mut data = vec![0u8; 1024];
            loop {
                let len = rustix::io::read(&r, &mut data).unwrap();
                if len > 0 {
                    let data = &data[..len];
                    dump.push_str(std::str::from_utf8(data).unwrap());
                } else {
                    break dump;
                }
            }
        });

        if let Err(e) = binder.dump(w.as_fd(), &[]) {
            eprintln!("Failed to dump {name:?}: {e:?}")
        } else {
            drop(w);
            println!("{}", dump.join().unwrap());
        }
    }
}
