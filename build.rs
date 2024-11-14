use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

use walkdir::WalkDir;
use zip::{write::SimpleFileOptions, ZipWriter};
/*
fn main(){
    println!("cargo::rerun-if-changed=exercises");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let toml_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("exercises");
    let src_path = Path::new(&toml_dir).join("exercises");


    //command zip -p pass123 ccat-command.zip ccat-1.1.0/
    let _ = Command::new("zip").args(["-r", "--password", "strongest_password", dest_path.as_os_str().to_str().unwrap(), src_path.as_os_str().to_str().unwrap()]).output().unwrap();
    /*let s = String::from_utf8(c.stdout).unwrap();
    println!("{}", s);
    panic!("wooo");*/
}*/

fn main() {
    println!("cargo::rerun-if-changed=exercises/src");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let toml_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let src_path = Path::new(&toml_dir).join("exercises").join("src");
    let dest_path = Path::new(&out_dir).join("exercises.zip");
    let dest = fs::File::create(dest_path).unwrap();
    let mut zip = ZipWriter::new(dest);

    let zip_options =
        SimpleFileOptions::default().with_aes_encryption(zip::AesMode::Aes128, "sheet1/ex06.rs");

    let mut buffer = Vec::new();
    for entry in WalkDir::new(&src_path) {
        let entry = entry.unwrap();
        if entry.metadata().unwrap().is_dir() {
            continue;
        }
        let mut f = File::open(entry.path()).unwrap();
        if entry.path().ends_with("lib.rs") {
            continue;
        }
        if entry.path().ends_with("mod.rs") {
            continue;
        }
        let p = entry.path().strip_prefix(&src_path).unwrap();
        zip.start_file(p.to_string_lossy(), zip_options).unwrap();

        f.read_to_end(&mut buffer).unwrap();
        zip.write_all(&buffer).unwrap();
        buffer.clear();
    }
    zip.finish().unwrap();
}
