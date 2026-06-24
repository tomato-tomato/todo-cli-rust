use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn setup_test_home() -> TempDir {
    let tmp = tempfile::tempdir().unwrap();
    unsafe {
        std::env::set_var("HOME", tmp.path());
    }
    tmp
}

#[test]
fn test_add_and_list() {
    let _tmp = setup_test_home();

    // 添加一条待办
    Command::cargo_bin("tasky")
        .unwrap()
        .args(&["add", "测试任务"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added #1"));

    // 列出待办
    Command::cargo_bin("tasky")
        .unwrap()
        .args(&["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("测试任务"));
}

#[test]
fn test_done_marks_completed() {
    let _tmp = setup_test_home();

    // 先添加
    Command::cargo_bin("tasky")
        .unwrap()
        .args(&["add", "完成任务"])
        .assert()
        .success();

    // 标记完成
    Command::cargo_bin("tasky")
        .unwrap()
        .args(&["done", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Done #1"));

    // 默认 list 不应显示已完成的
    Command::cargo_bin("tasky")
        .unwrap()
        .args(&["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Nothing here"));
}

#[test]
fn test_remove_nonexistent() {
    let _tmp = setup_test_home();

    Command::cargo_bin("tasky")
        .unwrap()
        .args(&["remove", "99"])
        .assert()
        .failure();
}
