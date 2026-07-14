fn main() {
    let mut ac = autocfg::new();
    ac.set_no_std(true);

    ac.emit_expression_cfg("1f64.total_cmp(&2f64)", "has_total_cmp"); // 1.62
    ac.emit_path_cfg("core::num::Saturating", "has_num_saturating"); // 1.74

    // round_ties_even is only available in `std`
    ac.set_no_std(false);
    ac.emit_expression_cfg("1.5f64.round_ties_even()", "has_round_ties_even"); // 1.77

    autocfg::rerun_path("build.rs");
}
