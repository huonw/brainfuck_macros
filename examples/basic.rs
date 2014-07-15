#![feature(phase)]

#[phase(plugin)] extern crate brainfuck_macros;

use std::io;

fn main() {
    let hello_world = brainfuck!{
        ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>
        ---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
    };

    hello_world(&mut io::stdin(), &mut io::stdout()).unwrap();
}
