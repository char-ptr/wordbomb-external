use path_absolutize::Absolutize;

fn main() {
    let this_path = std::env::current_dir().expect("unable to get cwd??");
    let absolute = this_path.absolutize().expect("unable to absolutize");
    let strab = absolute.to_str().expect("unable to get as str");

    println!("cargo:rustc-link-lib=dylib=FakerInputDll");
    println!(r"cargo:rustc-link-search=native={strab}");
}

