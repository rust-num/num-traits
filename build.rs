fn main() {
    let mut ac = autocfg::new();
    ac.set_no_std(true);

    ac.emit_expression_cfg("1f64.total_cmp(&2f64)", "has_total_cmp"); // 1.62
    ac.emit_path_cfg("core::num::Saturating", "has_num_saturating"); // 1.74

    autocfg::rerun_path("build.rs");
}
