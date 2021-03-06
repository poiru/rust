// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Tests that type parameter bounds on an implementation need not match the
// trait exactly, as long as the implementation doesn't demand *more* bounds
// than the trait.

trait A {
    fn foo<T: Eq + Ord>(&self);
}

impl A for int {
    fn foo<T: Ord + Ord>(&self) {}
}

fn main() {}


