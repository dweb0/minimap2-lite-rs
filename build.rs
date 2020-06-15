use std::process::Command;
use std::fs;
use std::env;

fn main() {

    let out_dir = env::var("OUT_DIR").unwrap();
    let c_src_path = "minimap2";
    
    Command::new("make")
        .current_dir(c_src_path)
        .output()
        .expect("Failed to make minimap2");

    let target_file = format!("{}/libminimap2.a", out_dir);
    fs::copy(&format!("{}/libminimap2.a", c_src_path), target_file).unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=minimap2");
}
