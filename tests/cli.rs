use assert_cmd::Command;
use assert_fs::TempDir;
use predicates::prelude::*;

#[test]
fn add_and_list_tasks() {
    let temp = TempDir::new().unwrap();
    Command::cargo_bin("rust-task-manager").unwrap()
        .current_dir(&temp)
        .args(["add", "Test task"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added task 1"));
    Command::cargo_bin("rust-task-manager").unwrap()
        .current_dir(&temp)
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1. [ ] Test task"));
}

#[test]
fn mark_and_delete_task() {
    let temp = TempDir::new().unwrap();
    Command::cargo_bin("rust-task-manager").unwrap()
        .current_dir(&temp)
        .args(["add", "Another task"])
        .assert().success();
    Command::cargo_bin("rust-task-manager").unwrap()
        .current_dir(&temp)
        .args(["done", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Marked task 1 done"));
    Command::cargo_bin("rust-task-manager").unwrap()
        .current_dir(&temp)
        .args(["delete", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Deleted task 1"));
}
