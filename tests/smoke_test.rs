//! Test/examples for the brainfuck macro.

#![feature(plugin)]
#![plugin(brainfuck_macros)]

#[cfg(bf_bf_interpreter)]
extern crate bf_bf_interpreter;

extern crate rand;

use std::io;

/// Takes a compiled brainfuck program, feeds it `input` one byte at a
/// time, and compares the output against `expected_output`.
fn run(bf: fn(&mut io::Read, &mut io::Write) -> io::Result<Vec<u8>>,
       input: &str,
       expected_output: &str) {
    let mut input = input.as_bytes();
    let mut out = Vec::new();

    assert!(bf(&mut input, &mut out).is_ok());

    assert_eq!(std::str::from_utf8(&mut out).ok().expect("non-UTF8 bf output"),
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
    // test: print input (random) string to output as is 
    use rand::Rng;

    let bf = brainfuck!(,+[-.,+]);
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        let len = rng.gen::<usize>() % 200;
        let s = rng.gen_ascii_chars().take(len).collect::<String>();
        run(bf, &*s, &*s)
    }
}
