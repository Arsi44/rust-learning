use std::env;

fn main() {
    // #1 From env!
    let path: &str = env!("PATH");
    println!("the $PATH variable at the time of compiling: {path}");
    println!("{}", env!("CARGO_PKG_VERSION"));

    // #2 From std::env
    let key = "HOME";
    match env::var(key) {
        Ok(val) => println!("{key}: {val:?}"),
        Err(e) => println!("couldn't interpret {key}: {e}"),
    }

    for (key, value) in env::vars() {
        println!("{key}: {value}");
       }

    // #3 include!
    // let strings = include!("data.rs");
    // include!(concat!(env!("OUT_DIR"), "/some_file.rs"));

    // #4 include_str!
    let mut spanish = include_str!("spanish.in");
    assert_eq!(spanish, "adiós");
    

}
