// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_id = "github.com/mozilla-servo/rust-core-text#core_text:0.1"]
#![crate_type = "lib"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

extern crate libc;
extern crate std;

extern crate core_foundation;
extern crate core_graphics;

pub mod font;
pub mod font_collection;
pub mod font_descriptor;
pub mod font_manager;
