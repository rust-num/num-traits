fn main() {
    let ac = autocfg::new();

    ac.emit_expression_cfg("1f64.total_cmp(&2f64)", "has_total_cmp"); // 1.62

    ac.emit_expression_cfg("3.14f64.to_ne_bytes()", "has_float_to_from_bytes");

    autocfg::rerun_path("build.rs");
}
