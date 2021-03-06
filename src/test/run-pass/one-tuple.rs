// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Why one-tuples? Because macros.


pub fn main() {
    match ('c',) {
        (x,) => {
            assert_eq!(x, 'c');
        }
    }
    // test the 1-tuple type too
    let x: (char,) = ('d',);
    let (y,) = x;
    assert_eq!(y, 'd');
}
