mod chroma;

fn main() {
    println!("Hello, huey!");
    let color = unsafe { chroma::ffi::chroma_init_hex(0xFF8800) };
    let converted = unsafe { chroma::ffi::chroma_convert(color, chroma::ffi::Space::Oklch) };
    let oklch = unsafe { converted.data.oklch };
    println!("L={}, C={}, H={}", oklch.l, oklch.c, oklch.h);
}
