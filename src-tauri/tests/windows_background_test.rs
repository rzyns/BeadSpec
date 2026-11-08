#![cfg(windows)]

use beadspec_lib::bd::api::BeadsDataSource;
use beadspec_lib::bd::runner::{spawn_managed, BdRunner};
use beadspec_lib::commands::gitnexus_status::get_gitnexus_status;
use serde_json::{json, Value};
use std::time::Duration;

#[tokio::test]
#[ignore = "requires BEADSPEC_TEST_GITNEXUS_PROJECT; optional CLI may be unavailable"]
async fn real_periodic_gitnexus_check_reports_its_actual_result() {
    let project = std::env::var("BEADSPEC_TEST_GITNEXUS_PROJECT").unwrap();
    let status = get_gitnexus_status(project).await.unwrap();
    eprintln!(
        "GitNexus available={}: {}",
        status.available, status.message
    );
    if !status.available {
        assert!(status.message.contains("gitnexus_"));
        assert!(!status.message.contains("index is fresh"));
    } else {
        assert!(status.last_indexed_at.is_some());
    }
}

#[tokio::test]
async fn managed_git_keeps_success_and_failure_diagnostics() {
    let fixture = tempfile::tempdir().unwrap();
    let cwd = fixture.path();
    let initialized = spawn_managed(
        "git",
        &["init", "--quiet"],
        cwd,
        Duration::from_secs(10),
        &[],
    )
    .await
    .unwrap();
    assert_eq!(initialized.exit_code, Some(0));
    let status = spawn_managed(
        "git",
        &["status", "--porcelain"],
        cwd,
        Duration::from_secs(10),
        &[],
    )
    .await
    .unwrap();
    assert_eq!(status.exit_code, Some(0));
    assert!(status.stdout.is_empty());
    let failure = spawn_managed(
        "git",
        &["not-a-real-git-command"],
        cwd,
        Duration::from_secs(10),
        &[],
    )
    .await
    .unwrap();
    assert_ne!(failure.exit_code, Some(0));
    assert!(String::from_utf8_lossy(&failure.stderr).contains("not-a-real-git-command"));
}

#[tokio::test]
#[ignore = "requires BEADSPEC_TEST_BD; creates its own disposable Beads workspace"]
async fn real_windows_background_beads_commands_preserve_results() {
    let binary = std::env::var("BEADSPEC_TEST_BD").expect("explicit Beads binary");
    let fixture = tempfile::tempdir().unwrap();
    let project = fixture.path().join("project with spaces");
    std::fs::create_dir(&project).unwrap();
    let initialized = spawn_managed(
        &binary,
        &["init", "--prefix=hidden", "--skip-hooks", "--skip-agents"],
        &project,
        Duration::from_secs(60),
        &[("BD_NON_INTERACTIVE", "1")],
    )
    .await
    .unwrap();
    assert_eq!(
        initialized.exit_code,
        Some(0),
        "{}",
        String::from_utf8_lossy(&initialized.stderr)
    );

    let canonical = std::fs::canonicalize(&project).unwrap();
    let project = canonical.to_string_lossy();
    let runner = BdRunner::new_with_override(&project, &binary).unwrap();
    let created: Value = serde_json::from_str(
        &runner
            .run(&[
                "create",
                "Hidden command acceptance",
                "--type=task",
                "--labels=human",
                "--json",
            ])
            .await
            .unwrap(),
    )
    .unwrap();
    let id = created["id"].as_str().unwrap();
    let shown: Value =
        serde_json::from_str(&runner.run(&["show", id, "--json"]).await.unwrap()).unwrap();
    assert_eq!(shown[0]["title"], "Hidden command acceptance");
    let failure = runner.run(&["not-a-real-beads-command"]).await.unwrap_err();
    assert!(failure.to_string().contains("not-a-real-beads-command"));

    let source = BeadsDataSource::connect(&project, &binary, "hidden-acceptance")
        .await
        .unwrap();
    let result =
        async {
            let human_queue: Value =
                serde_json::from_str(&source.run_cli(&["human", "list", "--json"]).await.unwrap())
                    .unwrap();
            assert!(
                human_queue
                    .as_array()
                    .is_some_and(|rows| rows.iter().any(|row| row["id"] == id)),
                "periodic human queue returns the human-labelled fixture: {human_queue}"
            );
            for _ in 0..3 {
                let issues = source.list_issues().await.unwrap();
                assert!(issues.iter().any(|issue| issue["id"] == id));
            }
            source.write(
            &["update", id, "--notes=background write succeeded"],
            "issues.update", "PATCH", &format!("/issues/{id}"),
            json!({"actor":"hidden-acceptance","patch":{"notes":"background write succeeded"}}),
        ).await.unwrap();
            assert_eq!(
                source.get_issue(id).await.unwrap()["notes"],
                "background write succeeded"
            );
            let error = source.get_issue("hidden-does-not-exist").await.unwrap_err();
            assert!(
                !error.is_empty(),
                "read errors must remain visible to the caller"
            );
        };
    result.await;
    source.stop().await;
}
