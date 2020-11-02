fn main() {
    #[cfg(feature = "ffi")]
    {
        use std::env;
        use cbindgen::{ Builder, Language };

        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        Builder::new()
            .with_language(Language::C)
            .with_crate(crate_dir)
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file("headers/casper_client.h");
    }
}
