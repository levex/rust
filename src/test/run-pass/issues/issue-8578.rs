// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
// pretty-expanded FIXME #23616

pub struct UninterpretedOption_NamePart {
    name_part: Option<String>,
}

impl<'a> UninterpretedOption_NamePart {
    pub fn default_instance() -> &'static UninterpretedOption_NamePart {
        static instance: UninterpretedOption_NamePart = UninterpretedOption_NamePart {
            name_part: None,
        };
        &instance
    }
}

pub fn main() {}
