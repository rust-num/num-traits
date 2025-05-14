fn main() {
    let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
    let ac = autocfg::AutoCfg::with_dir(temp_dir.path()).unwrap();

    ac.emit_expression_cfg("1f64.total_cmp(&2f64)", "has_total_cmp"); // 1.62

    autocfg::rerun_path("build.rs");
}
