//! Test/examples for the brainfuck macro.

#![feature(phase)]

#[phase(syntax)] extern crate brainfuck;

#[cfg(bf_bf_interpreter)]
extern crate bf_bf_interpreter;


extern crate rand;

use std::io;
use std::io::{BufReader, MemWriter};

/// Takes a compiled brainfuck program, feeds it `input` one byte at a
/// time, and compares the output against `expected_output`.
fn run(bf: fn(&mut Reader, &mut Writer) -> io::IoResult<Vec<u8>>,
       input: &str,
       expected_output: &str) {
    let mut input = BufReader::new(input.as_bytes());
    let mut out = MemWriter::new();

    assert!(bf(&mut input, &mut out).is_ok());

    assert_eq!(std::str::from_utf8(out.unwrap().as_slice()).expect("non-UTF8 bf output"),
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
    use rand::{task_rng, Rng};

    let bf = brainfuck!(,+[-.,+]);
    let mut rng = task_rng();

    for _ in range(0, 100) {
        let len = rng.gen::<uint>() % 200;
        let s = rng.gen_ascii_str(len);
        run(bf, s, s)
    }
}



#[test]
#[cfg(bf_bf_interpreter)]
fn bf_interpreter() {
    run(bf_bf_interpreter::bf(),
        "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>
         ---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.!",
        "Hello World!\n");
}
