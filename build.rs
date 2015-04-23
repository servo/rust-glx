extern crate gl_generator;
extern crate khronos_api;

use std::env;
use std::fs::File;
use std::path::PathBuf;

fn main() {
    let dest = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();
    gl_generator::generate_bindings(gl_generator::StaticGenerator,
                                    gl_generator::registry::Ns::Glx,
                                    gl_generator::Fallbacks::All,
                                    khronos_api::GLX_XML,
                                    vec!["GLX_EXT_texture_from_pixmap".to_string()],
                                    "1.4", "core", &mut file).unwrap();
}
