A simple procedural macro that turns a brainfuck program into native
code.

Example:

```rust
#![feature(phase)]

#[phase(syntax)] extern crate brainfuck;

use std::io;

fn main() {
    let hello_world = brainfuck!{
        ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>
        ---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
    };

    hello_world(&mut io::stdin(), &mut io::stdout());
}
```


See `test.rs` for some basic examples and `bf_bf_interpreter.rs` for
[brainfuck interpreter written in brainfuck](http://homepages.xnet.co.nz/~clive/eigenratios/cgbfi2.b).

Compiles with `e454851`.
