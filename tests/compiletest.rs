#![cfg(feature = "nightly")]
use ui_test::{Config, dependencies::DependencyBuilder, run_tests};

fn main() -> ui_test::color_eyre::Result<()> {
    let mut config = Config::rustc("tests/compile-fail");
    config.skip_files.extend([
        ".fixed".into(),
        ".stderr".into(),
        "compile_fail_lib.in".into(),
    ]);

    config.comment_defaults.base().set_custom(
        "dependencies",
        DependencyBuilder {
            crate_manifest_path: "tests/compile-fail/Cargo.toml".into(),
            bless_lockfile: true,
            ..DependencyBuilder::default()
        },
    );

    // Compile all `.rs` files in the given directory (relative to your
    // Cargo.toml) and compare their output against the corresponding
    // `.stderr` files.
    run_tests(config)
}
