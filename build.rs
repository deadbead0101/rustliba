
use std::env;
use std::path::Path;

// Create a rustc flag to link C/libhello.a
#[allow(unused_must_use)]
fn main() {

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    
    let lib_dir = Path::new(&manifest_dir)
        .join("C");
    print_it(&lib_dir);
}

fn print_it(lib_dir: &Path) {
    println!("cargo:rustc-flags=-L {}", lib_dir.to_string_lossy());
    //println!("cargo:rustc-link-lib=static={}", "hello");
}

