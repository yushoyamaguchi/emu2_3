use std::env;
use std::process;
use std::fs;


fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: emu filename");
        process::exit(1);
    }
    let filename = &args[1];
    println!("filename = {}", filename);
}
