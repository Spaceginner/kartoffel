#![no_std]
#![no_main]

#![feature(let_chains)]
#![feature(ascii_char)]

extern crate alloc;


mod prog;
mod utils;
mod motor;
mod dir;
mod compass;
mod radar;
mod sysk;


#[no_mangle]
fn main() {
    compass::reset();
    
    // prog::maze::main()
    // prog::heist::main()
    // prog::flags::main()
    // prog::arena::main()
    // prog::wonder::main()
    prog::spammer::main()
}
