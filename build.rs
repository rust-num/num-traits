fn main() {
    let ac = autocfg::new();

    ac.emit_expression_cfg("1f64.total_cmp(&2f64)", "has_total_cmp"); // 1.62

    autocfg::rerun_path("build.rs");

    let version: semver::Version = 
    std::env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION must be set").parse().expect("Cargo.toml of this crate to be correct version format");
    
    if version.major >= 1 {
        println!("cargo:rustc-cfg=unstable_1")
    }
}
