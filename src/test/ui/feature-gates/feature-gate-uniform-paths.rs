// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// edition:2018

pub mod foo {
    pub use bar::Bar; //~ ERROR imports can only refer to extern crate names

    pub mod bar {
        pub struct Bar;
    }
}

use inline; //~ ERROR imports can only refer to extern crate names

use Vec; //~ ERROR imports can only refer to extern crate names

use vec; //~ ERROR imports can only refer to extern crate names

fn main() {
    let _ = foo::Bar;
}
