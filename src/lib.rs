// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "glx"]
#![comment = "GLX 1.4 bindings"]
#![license = "ASL2"]
#![crate_type = "lib"]

#![feature(phase)]
#![feature(globs)]

#[phase(plugin)]
extern crate gl_generator;

/// GLX bindings
generate_gl_bindings!{
	api: "glx",
	profile: "core",
	version: "1.4",
	generator: "static",
	extensions: [ "GLX_EXT_texture_from_pixmap" ]
}
