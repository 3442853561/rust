// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// revisions: rpass1 cfail2
// compile-flags: -Z query-dep-graph

#![allow(warnings)]
#![feature(rustc_attrs)]

// Sanity check for the dirty-clean system. Give the opposite
// annotations that we expect to see, so that we check that errors are
// reported.

fn main() { }

mod x {
    #[cfg(rpass1)]
    pub fn x() -> usize {
        22
    }

    #[cfg(cfail2)]
    pub fn x() -> u32 {
        22
    }
}

mod y {
    use x;

    #[rustc_clean(label="TypeckTables", cfg="cfail2")]
    pub fn y() {
        //[cfail2]~^ ERROR `TypeckTables(y::y)` should be clean but is not
        x::x();
    }
}

mod z {
    #[rustc_dirty(label="TypeckTables", cfg="cfail2")]
    pub fn z() {
        //[cfail2]~^ ERROR `TypeckTables(z::z)` should be dirty but is not
    }
}
