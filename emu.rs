use std::env;
use std::process;
use std::fs;

enum Register { EAX=0, ECX, EDX, EBX, ESP, EBP, ESI, EDI }
const REGISTERS_COUNT: usize = 8;
const REGISTERS_NAME: [&str; REGISTERS_COUNT] = ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];

const MEMORY_SIZE: usize = 1024 * 1024;
struct Emulator {
    // General-purpose Registers
    registers: [u32; REGISTERS_COUNT],
    // EFLAGS Register
    eflags: u32,
    // Instruction Pointer
    eip: u32,
    // Memory
    memory: Vec<u8>
}

//fn create_emu(size:usize,eip:u32,esp:u32)-> {
//}

fn ret_str()-> Box<String>{
    let mut str=String::from("aaaa");
    let mut b_str=Box::new(str);
    b_str
}


fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: emu filename");
        process::exit(1);
    }
    let filename = &args[1];
    println!("filename = {}", filename);
    let mut str=ret_str();
    println!("str = {}", str);
}
