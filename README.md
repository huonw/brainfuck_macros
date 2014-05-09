A simple procedural macro that turns a
[Brainfuck](http://en.wikipedia.org/wiki/Brainfuck) program into
native code.

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


## Specs

Normal brainfuck, with:
- a tape length of 30000,
- cells storing unsigned bytes (with wrapping),
- EOF indicated by returning -1, and
- out-of-bounds index steps ignored (i.e. `<` when pointing at the
  first cell is just ignored, and similarly for `>` at the last).
