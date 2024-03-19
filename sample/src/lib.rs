#[no_mangle]
fn ANativeActivity_onCreate() {
    android_powerstats::pull_data().unwrap()
}
