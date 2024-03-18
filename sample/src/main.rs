use std::ffi::{c_char, CString};

#[repr(C)]
struct AIBinder {}

#[link(name = "binder_ndk")]
extern "C" {
    fn AIBinder_isAlive() -> bool;
    // Loaded at runtime because it cannot be linked dynamically through the NDK as the bindings are only in LL-NDK:
    // https://cs.android.com/android/platform/superproject/main/+/main:frameworks/native/libs/binder/ndk/libbinder_ndk.map.txt;l=96;drc=a7aa4ce8e2471f00f41c0f46495c76af478ddb06
    // fn AServiceManager_checkService(instance: *const c_char) -> *mut AIBinder;
}
type AServiceManager_checkService = extern "system" fn(instance: *const c_char) -> *mut AIBinder;

fn main() {
    let binder = unsafe { libloading::Library::new("libbinder_ndk.so").unwrap() };
    let service_manager_check_service =
        unsafe { binder.get::<AServiceManager_checkService>(b"AServiceManager_checkService\0") }
            .unwrap();

    let names = [
        "power",
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
        dbg!(name, service);
    }
}
