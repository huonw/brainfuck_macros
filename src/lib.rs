//! A macro that parses brainfuck code at compile time.

#![crate_name="brainfuck_macros"]
#![crate_type="dylib"]

#![feature(quote, plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

use syntax::ast;
use syntax::ast::TokenTree;
use syntax::ptr::P;
use syntax::codemap;
use syntax::ext::base::{ExtCtxt, MacResult, MacEager};
use syntax::ext::build::AstBuilder;
use syntax::parse::token;

use rustc_plugin::Registry;

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(registrar: &mut Registry) {
    registrar.register_macro("brainfuck", brainfuck)
}


// This essentially translates token-wise, using the symbol mappings
// given in the table at:
// http://en.wikipedia.org/wiki/Brainfuck#Commands
fn brainfuck(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult+'static> {
    let bf = BF {
        array: quote_expr!(&mut *cx, _array),
        idx: quote_expr!(&mut *cx, _i),
        rdr: quote_expr!(&mut *cx, _r),
        wtr: quote_expr!(&mut *cx, _w),
        cx: cx,
    };
    let core_code = bf.tts_to_expr(sp, tts);
    
    MacEager::expr(quote_expr!(bf.cx, {
        use std::io;
        fn run(_r: &mut io::Read, _w: &mut io::Write) -> io::Result<Vec<u8>> {
            let mut _array = vec![0u8; 30_000];
            let mut _i = 0;
            $core_code;
            Ok(_array)
        }
        run
    }))
}

struct BF<'a> {
    cx: &'a ExtCtxt<'a>,
    array: P<ast::Expr>,
    idx: P<ast::Expr>,
    rdr: P<ast::Expr>,
    wtr: P<ast::Expr>
}

impl<'a> BF<'a> {
    fn tts_to_expr(&self, sp: codemap::Span, tts: &[ast::TokenTree]) -> P<ast::Expr> {
        let v = tts.iter()
            .filter_map(|tt| self.tt_to_expr(sp,tt).map(|e| self.cx.stmt_expr(e)))
            .collect();

        let block = self.cx.block(sp, v, None);
        self.cx.expr_block(block)
    }

    fn tt_to_expr(&self, _sp: codemap::Span, tt: &ast::TokenTree) -> Option<P<ast::Expr>> {
        match *tt {
            TokenTree::Token(sp, ref tok) => self.token_to_expr(sp, tok),

            // [...] or (...) or {...}
            TokenTree::Delimited(sp, ref toks) => {
                if toks.delim == token::Bracket {
                    // [...]
                    let centre = self.tts_to_expr(sp, &*toks.tts);

                    let array = &self.array;
                    let idx = &self.idx;

                    Some(quote_expr!(self.cx, {
                        while $array[$idx] != 0 {
                            $centre
                        }
                    }))
                } else {
                    // not [...], so just translate directly (the
                    // delimiters are definitely invalid, so just
                    // ignoring them is fine)
                    Some(self.tts_to_expr(sp,toks.tts.as_slice()))
                }
            }
            TokenTree::Sequence(sp, _) => {
                self.cx.span_err(sp, "sequences unsupported in `brainfuck!`");
                None
            }
        }
    }

    fn token_to_expr(&self, sp: codemap::Span,
                     tok: &token::Token) -> Option<P<ast::Expr>> {
        // some tokens consist of multiple characters that brainfuck
        // needs to know about, so we do the obvious thing of just
        // taking each one and combining into a single expression.
        macro_rules! recompose {
            ($($token: expr),*) => {
                {
                    let stmts = vec!(
                        $(
                            {
                                let e = self.token_to_expr(sp,&$token)
                                    .expect("brainfuck: invalid token decomposition?");
                                self.cx.stmt_expr(e)
                            } ),* );
                    Some(self.cx.expr_block(self.cx.block(sp, stmts, None)))
                }
            }
        }
        let idx = &self.idx;
        let array = &self.array;
        match *tok {
            token::Lt | token::Gt => {
                let left = *tok == token::Lt;
                Some(quote_expr!(self.cx, {
                    if $left {
                        if $idx > 0 {
                            $idx -= 1;
                        }
                    } else {
                        if $idx < $array.len() - 1 {
                            $idx += 1;
                        }
                    }
                }))
            }

            // = does nothing, so just ignore it in += >>= etc.
            token::BinOpEq(a) => recompose!(token::BinOp(a)),
            // <<
            token::BinOp(token::Shl) => {
                recompose!(token::Lt, token::Lt)
            }
            // >>
            token::BinOp(token::Shr) => {
                recompose!(token::Gt, token::Gt)
            }

            token::Dot => {
                let wtr = &self.wtr;
                Some(quote_expr!(self.cx, try!($wtr.write(&[$array[$idx]]))))
            }
            // ..
            token::DotDot => recompose!(token::Dot, token::Dot),
            // ...
            token::DotDotDot => recompose!(token::Dot, token::Dot, token::Dot),


            token::Comma => {
                let rdr = &self.rdr;
                Some(quote_expr!(self.cx, {
                    let mut buffer = [0; 1];
                    $array[$idx] = match $rdr.read(&mut buffer[..]) {
                        Ok(1) => buffer[0],
                        Ok(_) => return Ok($array), //end the execution with the output we have, if we attempt to read past the input size
                        Err(e) => return Err(e)
                    }
                }))
            }


            token::BinOp(token::Plus) => {
                Some(quote_expr!(self.cx, {
                    $array[$idx] += 1
                }))
            }
            token::BinOp(token::Minus) => {
                Some(quote_expr!(self.cx, {
                    $array[$idx] -= 1
                }))
            }


            // =>
            token::FatArrow => recompose!(token::Gt),
            // ->
            token::RArrow => recompose!(token::BinOp(token::Minus), token::Gt),
            // <-
            token::LArrow => recompose!(token::Lt, token::BinOp(token::Minus)),

            _ => {
                None
            }
        }
    }
}
