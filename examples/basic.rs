#![feature(plugin, io)]

#[plugin] #[no_link] extern crate brainfuck_macros;

use std::old_io as io;

fn main() {
    let hello_world = brainfuck!{
        ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>
        ---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
    };

    hello_world(&mut io::stdin(), &mut io::stdout()).unwrap();
}
