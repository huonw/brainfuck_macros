[![Build Status](https://travis-ci.org/huonw/brainfuck_macros.png)](https://travis-ci.org/huonw/brainfuck_macros)

A simple procedural macro that turns a
[Brainfuck](http://en.wikipedia.org/wiki/Brainfuck) program into
native code.

Example:

```rust
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
```


See `tests/` and `examples/` for some basic examples and the
`bf_bf_interpreter` subpackage for
[brainfuck interpreter written in brainfuck](http://homepages.xnet.co.nz/~clive/eigenratios/cgbfi2.b).

This is [Cargo enabled](http://crates.io/), and so can be used
by adding a `[dependencies.brainfuck_macros]` section pointing to this
git repository to your Cargo.toml. If you wish to also use
`bf_bf_interpreter`, simply add `[dependencies.bf_bf_interpreter]`
section also pointing to this repository.

## Specs

Normal brainfuck, with:
- a tape length of 30000,
- cells storing unsigned bytes (with wrapping),
- EOF indicated by returning -1, and
- out-of-bounds index steps ignored (i.e. `<` when pointing at the
  first cell is just ignored, and similarly for `>` at the last).
