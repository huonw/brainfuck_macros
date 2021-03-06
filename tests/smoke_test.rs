//! Test/examples for the brainfuck macro.

#![feature(plugin, io, rand)]

#[plugin] extern crate brainfuck_macros;

#[cfg(bf_bf_interpreter)]
extern crate bf_bf_interpreter;


use std::rand;
use std::rand::Rng;
use std::old_io as io;
use std::old_io::{BufReader, MemWriter};

/// Takes a compiled brainfuck program, feeds it `input` one byte at a
/// time, and compares the output against `expected_output`.
fn run(bf: fn(&mut Reader, &mut Writer) -> io::IoResult<Vec<u8>>,
       input: &str,
       expected_output: &str) {
    let mut input = BufReader::new(input.as_bytes());
    let mut out = MemWriter::new();

    assert!(bf(&mut input, &mut out).is_ok());

    assert_eq!(std::str::from_utf8(&*out.into_inner()).ok().expect("non-UTF8 bf output"),
               expected_output)
}

#[test]
fn hello_world() {
    run(brainfuck!{
        ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>
        ---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
    }, "", "Hello World!\n");
}

#[test]
fn hello_world_foreign_tokens() {
    run(brainfuck!{
        +=
        +
        foo bar baz
        ++++++[
        =>
        ++++[>++>+++>+++>+
        <<=
        <
        $100
        <-
        ]>+>+>-
        >>=
        +[<]
        <-
        ]
        >>
        .>
        ---.+++++++
        ..
        +++.
        >>
        .
        <-.<.+++.
        -=
        -----.--------.
        >>
        +.>++.
    }, "", "Hello World!\n");
}

#[test]
fn hello_world_harder() {
    // "This is a slightly more complex variant that often triggers interpreter bugs "
    // http://esolangs.org/wiki/brainfuck
    run(brainfuck!{
        >++++++++[<+++++++++>-]<.>>+>+>++>[-]+<[>[->+<<++++>]<<]>.+++++++..+++.>
        >+++++++.<<<[[-]<[-]>]<+++++++++++++++.>>.+++.------.--------.>>+.>++++.
    }, "", "Hello World!\n");
}

#[test]
fn cat() {
    let bf = brainfuck!(,+[-.,+]);
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        let len = rng.gen::<usize>() % 200;
        let s = rng.gen_ascii_chars().take(len).collect::<String>();
        run(bf, &*s, &*s)
    }
}
