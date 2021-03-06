// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![feature(tool_lints)]

#![allow(clippy::print_literal, clippy::write_literal)]
#![warn(clippy::print_stdout, clippy::use_debug)]

use std::fmt::{Debug, Display, Formatter, Result};

#[allow(dead_code)]
struct Foo;

impl Display for Foo {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", 43.1415)
    }
}

impl Debug for Foo {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // ok, we can use `Debug` formatting in `Debug` implementations
        write!(f, "{:?}", 42.718)
    }
}

fn main() {
    println!("Hello");
    print!("Hello");

    print!("Hello {}", "World");

    print!("Hello {:?}", "World");

    print!("Hello {:#?}", "#orld");

    assert_eq!(42, 1337);

    vec![1, 2];
}
