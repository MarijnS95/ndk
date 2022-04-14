use log::info;
use ndk::media::media_codec::MediaCodec;

#[cfg_attr(
    target_os = "android",
    ndk_glue::main(backtrace = "on", logger(level = "debug", tag = "hello-world"))
)]
fn main() {
    let x = MediaCodec::from_decoder_type("video/avc").unwrap();
    dbg!(&x);
    dbg!(x.output_format());
    dbg!(x.input_format());
    // let _trace;
    // if trace::is_trace_enabled() {
    //     _trace = trace::Section::new("ndk-rs example main").unwrap();
    // }
    // info!("hello world");
}
