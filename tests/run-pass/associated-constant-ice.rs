// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


pub trait Trait {
    const CONSTANT: u8;
}

impl Trait for u8 {
    const CONSTANT: u8 = 2;
}

fn main() {
    println!("{}", u8::CONSTANT * 10);
}
