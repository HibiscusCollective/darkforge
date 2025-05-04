use assert_cmd::{Command, cargo_bin};

#[test]
fn test_prints_version_given_version_flag_is_set() {
    Command::new(cargo_bin!("example-act"))
        .arg("--version")
        .assert()
        .success()
        .stdout("Forge Actions Example 0.1.0\n");
}
