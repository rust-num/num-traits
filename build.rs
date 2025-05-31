fn main() {
    let ac = autocfg::new();

    ac.emit_expression_cfg("1f64.total_cmp(&2f64)", "has_total_cmp"); // 1.62

    autocfg::rerun_path("build.rs");

    // FIXME: use autocfg to emit it
    println!("cargo:rustc-cfg=has_f16");
    println!("cargo:rustc-check-cfg=cfg(has_f16)");
    println!("cargo:rustc-cfg=has_f128");
    println!("cargo:rustc-check-cfg=cfg(has_f128)");
}
