use std::{env, path::PathBuf};

use cbindgen::{Language, ParseConfig};

fn main() {
    env_logger::init();

    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR"));
    dbg!(&manifest_dir);

    let mut c_config = cbindgen::Config::default();

    // Only generate pure C definitions
    c_config.language = Language::C;

    c_config.pragma_once = true;
    c_config.usize_is_size_t = true;
    c_config.include_version = true;
    c_config.no_includes = true;
    c_config.sys_includes = ["stdint.h", "stdlib.h"].map(String::from).to_vec();
    c_config.parse = ParseConfig {
        clean: true,
        parse_deps: true,
        include: Some(vec!["a".into(), "b".into()]),
        // extra_bindings: vec![
        //     "a".into(),
        //     "b".into(),
        // ],
        ..Default::default()
    };
    // c_config.function = FunctionConfig::default();

    let builder = cbindgen::Builder::new()
        .with_config(c_config)
        .with_src(manifest_dir.join("src").join("ffi.rs"));

    // dbg!(&builder);

    builder
        .generate()
        .unwrap()
        .write_to_file("my_header_from_build.h");
}
