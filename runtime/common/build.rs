use cbindgen::{Config, Language, RenameRule};
use std::env;

fn main() {
    let exported_symbols = ["RtCall", "RtCallLogArg"];

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config = Config::default();
    config.enumeration.prefix_with_name = true;

    config.language = Language::C;
    config.enumeration.rename_variants = RenameRule::ScreamingSnakeCase;
    config.no_includes = true;
    config.includes.push("stdint.h".to_string());
    config.cpp_compat = true;

    for symbol in exported_symbols {
        config.export.include.push(symbol.to_string());
    }

    cbindgen::Builder::new()
        .with_crate(manifest_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file("../../libreplc.h");
}
