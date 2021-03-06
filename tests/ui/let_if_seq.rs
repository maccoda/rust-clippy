// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![feature(tool_lints)]


#![allow(unused_variables, unused_assignments, clippy::similar_names, clippy::blacklisted_name)]
#![warn(clippy::useless_let_if_seq)]

fn f() -> bool { true }
fn g(x: i32) -> i32 { x + 1 }

fn issue985() -> i32 {
    let mut x = 42;
    if f() {
        x = g(x);
    }

    x
}

fn issue985_alt() -> i32 {
    let mut x = 42;
    if f() {
        f();
    } else {
        x = g(x);
    }

    x
}

fn issue975() -> String {
    let mut udn = "dummy".to_string();
    if udn.starts_with("uuid:") {
        udn = String::from(&udn[5..]);
    }
    udn
}

fn early_return() -> u8 {
    // FIXME: we could extend the lint to include such cases:
    let foo;

    if f() {
        return 42;
    } else {
        foo = 0;
    }

    foo
}

fn main() {
    early_return();
    issue975();
    issue985();
    issue985_alt();

    let mut foo = 0;
    if f() {
        foo = 42;
    }

    let mut bar = 0;
    if f() {
        f();
        bar = 42;
    }
    else {
        f();
    }

    let quz;
    if f() {
        quz = 42;
    } else {
        quz = 0;
    }

    // `toto` is used several times
    let mut toto;
    if f() {
        toto = 42;
    } else {
        for i in &[1, 2] {
            toto = *i;
        }

        toto = 2;
    }

    // found in libcore, the inner if is not a statement but the block's expr
    let mut ch = b'x';
    if f() {
        ch = b'*';
        if f() {
            ch = b'?';
        }
    }

    // baz needs to be mut
    let mut baz = 0;
    if f() {
        baz = 42;
    }

    baz = 1337;
}
