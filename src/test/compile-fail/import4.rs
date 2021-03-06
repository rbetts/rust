// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// error-pattern: import

mod a {
    #[legacy_exports]; import foo = b::foo; export foo; }
mod b {
    #[legacy_exports]; import foo = a::foo; export foo; }

fn main(args: ~[str]) { debug!("loop"); }
