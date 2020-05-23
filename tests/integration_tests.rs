use assert_cmd::Command;
use predicates::boolean::PredicateBooleanExt;
use predicates::prelude::predicate::str as pstr;
use tempfile::TempDir;

#[test]
fn test_persist_multiple_pages() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = TempDir::new()?;
    let filepath = tmp_dir.path().join("journal.bin");

    // Function that returns a configured "Command" object.
    let mk_cmd = || -> Result<Command, Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("journal")?;
        cmd.env("JOURNAL_STORAGE_PATH", &filepath);
        Ok(cmd)
    };

    mk_cmd()?
        .arg("new")
        .arg("-t")
        .arg("First page")
        .assert()
        .success()
        .stdout(pstr::contains("id_cnt: 1"));

    mk_cmd()?
        .arg("new")
        .arg("-t")
        .arg("Second page")
        .assert()
        .success()
        .stdout(pstr::contains("id_cnt: 2"));

    mk_cmd()?.arg("list").assert().success().stdout(
        pstr::contains("id_cnt: 2")
            .and(pstr::contains("First page"))
            .and(pstr::contains("Second page")),
    );

    Ok(())
}
