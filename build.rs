use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    if probe("fn main() { 0i128; }") {
        println!("cargo:rustc-cfg=has_i128");
    } else if env::var_os("CARGO_FEATURE_I128").is_some() {
        panic!("i128 support was not detected!");
    }

    // Verbose probe to ensure we can blanket impl associated const traits.
    // Early versions of associated consts prohibited this due to a restriction
    // of const expressions to non-Drop types.
    if probe("fn main() {
        trait Const { const CONST: Self; }
        impl<T: Const> Const for Option<T> {
            const CONST: Option<T> = Some(T::CONST);
        }
    }") {
        println!("cargo:rustc-cfg=has_associated_consts");
    }
}

/// Test if a code snippet can be compiled
fn probe(code: &str) -> bool {
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let out_dir = env::var_os("OUT_DIR").expect("environment variable OUT_DIR");

    let mut child = Command::new(rustc)
        .arg("--out-dir")
        .arg(out_dir)
        .arg("--emit=obj")
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()
        .expect("rustc probe");

    child
        .stdin
        .as_mut()
        .expect("rustc stdin")
        .write_all(code.as_bytes())
        .expect("write rustc stdin");

    child.wait().expect("rustc probe").success()
}
