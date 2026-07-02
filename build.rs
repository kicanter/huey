fn main() {
    println!("cargo:rustc-link-search=native=vendor");
    println!("cargo:rustc-link-lib=static=chroma");
    println!("cargo:rerun-if-changed=vendor/libchroma.a");
}
