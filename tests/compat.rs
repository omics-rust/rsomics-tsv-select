use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn ours() -> Command {
    Command::new(env!("CARGO_BIN_EXE_rsomics-tsv-select"))
}

fn golden() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden/data.tsv")
}

fn csvtk_available() -> bool {
    Command::new("csvtk")
        .arg("version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

#[test]
fn runs() {
    assert!(
        ours()
            .arg(golden())
            .args(["-c", "1"])
            .output()
            .unwrap()
            .status
            .success()
    );
}

// Column selection must match `csvtk cut` (the named upstream), by index and name.
#[test]
fn matches_csvtk() {
    if !csvtk_available() {
        eprintln!("skipping: csvtk not found");
        return;
    }
    let by_idx_ours = ours()
        .arg(golden())
        .args(["-c", "1", "3"])
        .output()
        .unwrap();
    let by_idx_csvtk = Command::new("csvtk")
        .args(["cut", "-tT", "-f", "1,3"])
        .arg(golden())
        .output()
        .unwrap();
    assert!(by_idx_csvtk.status.success());
    assert_eq!(by_idx_ours.stdout, by_idx_csvtk.stdout, "by-index mismatch");

    let by_name_ours = ours()
        .arg(golden())
        .args(["-c", "gene", "log2fc"])
        .output()
        .unwrap();
    let by_name_csvtk = Command::new("csvtk")
        .args(["cut", "-tT", "-f", "gene,log2fc"])
        .arg(golden())
        .output()
        .unwrap();
    assert!(by_name_csvtk.status.success());
    assert_eq!(
        by_name_ours.stdout, by_name_csvtk.stdout,
        "by-name mismatch"
    );
}
