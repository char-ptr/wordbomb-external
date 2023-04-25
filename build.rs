fn main() {
    println!("cargo:rustc-link-lib=dylib=FakerInputDll");
    println!(r"cargo:rustc-link-search=native=X:\meta\dev\rust\cumin-cli");
}