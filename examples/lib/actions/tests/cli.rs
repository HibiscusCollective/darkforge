use std::{path::Path, sync::LazyLock};

use assert_cmd::{Command, cargo_bin};
use indoc::indoc;
use rstest::rstest;

static BINARY: LazyLock<&Path> = LazyLock::new(|| cargo_bin!("forge"));

#[rstest]
#[case::short("-V")]
#[case::full("--version")]
fn test_prints_version_given_version_flag_is_set(#[case] flag: &str) {
    Command::new(BINARY.clone())
        .arg(flag)
        .assert()
        .success()
        .stdout("Forge Actions Example 0.1.0\n");
}

#[rstest]
#[case::short("-h")]
#[case::full("--help")]
fn test_prints_help_given_help_flag_is_set(#[case] flag: &str) {
    Command::new(BINARY.clone()).arg(flag).assert().success().stdout(indoc!(
        "
            Usage: forge

            Options:
              -h, --help     Print help
              -V, --version  Print version
        "
    ));
}
