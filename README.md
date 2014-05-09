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

Compiles with Rust master at `e454851` (run `make` to build everything
into the `build` subdirectory).


## Specs

Normal brainfuck, with:
- a tape length of 30000,
- cells storing unsigned bytes (with wrapping),
- EOF indicated by returning -1, and
- out-of-bounds index steps ignored (i.e. `<` when pointing at the
  first cell is just ignored, and similarly for `>` at the last).


## Copying

MIT:

```
Copyright (c) 2014 Huon Wilson

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
```
