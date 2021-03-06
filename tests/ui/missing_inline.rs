// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![feature(tool_lints)]

/* This file incorporates work covered by the following copyright and
 * permission notice:
 *   Copyright 2013 The Rust Project Developers. See the COPYRIGHT
 *   file at the top-level directory of this distribution and at
 *   http://rust-lang.org/COPYRIGHT.
 *
 *   Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 *   http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 *   <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 *   option. This file may not be copied, modified, or distributed
 *   except according to those terms.
 */
#![warn(clippy::missing_inline_in_public_items)]
#![crate_type = "dylib"]
// When denying at the crate level, be sure to not get random warnings from the
// injected intrinsics by the compiler.
#![allow(dead_code, non_snake_case)]

type Typedef = String;
pub type PubTypedef = String;

struct Foo {} // ok
pub struct PubFoo { } // ok
enum FooE {} // ok
pub enum PubFooE {} // ok

mod module {} // ok
pub mod pub_module {} // ok

fn foo() {}
pub fn pub_foo() {} // missing #[inline]
#[inline] pub fn pub_foo_inline() {} // ok
#[inline(always)] pub fn pub_foo_inline_always() {} // ok

#[allow(clippy::missing_inline_in_public_items)]
pub fn pub_foo_no_inline() {}

trait Bar {
    fn Bar_a(); // ok
    fn Bar_b() {} // ok
}


pub trait PubBar {
    fn PubBar_a(); // ok
    fn PubBar_b() {} // missing #[inline]
    #[inline] fn PubBar_c() {} // ok
}

// none of these need inline because Foo is not exported
impl PubBar for Foo {
    fn PubBar_a() {} // ok
    fn PubBar_b() {} // ok
    fn PubBar_c() {} // ok
}

// all of these need inline because PubFoo is exported
impl PubBar for PubFoo {
    fn PubBar_a() {} // missing #[inline]
    fn PubBar_b() {} // missing #[inline]
    fn PubBar_c() {} // missing #[inline]
}

// do not need inline because Foo is not exported
impl Foo {
    fn FooImpl() {} // ok
}

// need inline because PubFoo is exported
impl PubFoo {
    pub fn PubFooImpl() {} // missing #[inline]
}
