use std::{env, path::PathBuf};

fn main() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    bindings.write_to_file("src/bindings.rs").unwrap();
}
