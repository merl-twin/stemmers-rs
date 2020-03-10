extern crate snowball;

use snowball::compile;

use std::path::PathBuf;
use std::env;

fn main() {
    let languages = ["Russian","English"];
       
    let root_dir = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut lng_dir = root_dir.clone();
    lng_dir.push("languages");
    let lib_dir = PathBuf::from(&env::var("OUT_DIR").unwrap());

    for lng in &languages {
        let lng = lng.to_lowercase();
        let mut src = lng_dir.clone();
        src.push(format!("{}.sbl",lng));
        println!("cargo:rerun-if-changed={}",src.to_string_lossy());
        let mut dst = lib_dir.clone();
        dst.push(format!("{}.rs",lng));      
        compile(src.to_string_lossy().as_ref(),dst.to_string_lossy().as_ref()).unwrap();
    }
}
