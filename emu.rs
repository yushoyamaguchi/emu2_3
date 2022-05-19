use std::env;
use std::process;
use std::fs::File;
use Register::*;
use std::io::prelude::*;
use std::io;



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

fn type_of<T>(_: T) -> String{
  let a = std::any::type_name::<T>();
  return a.to_string();
}

fn create_emu(size:usize,eip:u32,esp:u32)-> Emulator{
    let mut emu = Emulator {
        // Clear all resisters by 0
        registers: [0; REGISTERS_COUNT],
        // Clear eflags by 0
        eflags: 0,
        // Init EIP register
        eip: eip,
        // Init memory
        memory: vec![0; size]
    };
    // Init ESP register
    emu.registers[ESP as usize] = esp;
    emu
}

fn bin_to_mem(file: &mut File, emu: &mut Emulator)-> Result<usize, io::Error>{
    let mut i=0;
    for byte in file.bytes(){
        emu.memory[i]=byte?;
        i+=1;
    }
    Ok(i)
}

fn init_instructions(){

}

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: emu filename");
        process::exit(1);
    }
    let filename = &args[1];
    println!("filename = {}", filename);
    // Open a binary file that contains a x86 machine code
    let mut file = File::open(filename).expect("file not found");
    let type_=type_of(file);
    println!("{}",type_);

    init_instructions();
}
