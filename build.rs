fn main() {
    let ac = autocfg::new();

    ac.emit_expression_cfg("1f64.is_subnormal()", "has_is_subnormal");
    ac.emit_expression_cfg("1f64.total_cmp(&2f64)", "has_total_cmp");

    ac.emit_expression_cfg("1u32.to_ne_bytes()", "has_int_to_from_bytes");
    ac.emit_expression_cfg("3.14f64.to_ne_bytes()", "has_float_to_from_bytes");

    autocfg::rerun_path("build.rs");
}
