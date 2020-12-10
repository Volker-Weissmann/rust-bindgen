extern crate bindgen;

use std::env;
use std::io::Write;

fn main() {
    println!("cargo:rerun-if-changed=example.cpp"); //TODO: bindgen itself should print this line
    {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .truncate(true)
            .open("generated.cpp")
            .unwrap();

        file.write_all(b"#include \"example.cpp\"\n")
            .expect("unable to write");
    }

    //enable-cxx-namespaces --whitelist-type outer::inner::Helper

    let _ = bindgen::Builder::default()
        .header("example.cpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg("-x")
        .clang_arg("c++")
        //.gen_safe_wrappers(true)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&format!(
            "{}{}",
            env::var("OUT_DIR").unwrap(),
            "/bindings.rs"
        ))
        .expect("Couldn't write bindings!");

    
        panic!("todo remove this");
}
