extern crate gl_generator;

use std::env;
use std::fs::File;
use std::path::PathBuf;
use gl_generator::{Registry, Api, Profile, Fallbacks};

fn main() {
    let dest = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();
    Registry::new(Api::Glx, (1, 4), Profile::Core, Fallbacks::All, ["GLX_EXT_texture_from_pixmap"])
        .write_bindings(gl_generator::StaticGenerator, &mut file).unwrap();
}
