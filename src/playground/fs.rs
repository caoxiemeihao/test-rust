use std::fs;
use std::path;

pub fn read() {
    let filename = "src/playground/hello_warp.rs";
    println!("In file {}", &filename);
    
    let tmp = path::Path::new("hello_warp.rs");
    println!("{}", tmp.display());

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
