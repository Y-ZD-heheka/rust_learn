use std::fs;

use tempfile::tempdir;

use super::{
    Priority, Status, Task, TaskLoadError, TaskLoadOutcome, TaskManager, TaskManagerLoadState,
    TaskStatistics, TaskStorage, TaskStorageConfig,
};

#[test]
fn test_task_creation() {
    let task = Task::new(1, "Test task", Priority::High);
    assert_eq!(task.id(), 1);
    assert_eq!(task.title(), "Test task");
    assert_eq!(task.priority(), Priority::High);
    assert_eq!(task.status(), Status::Pending);
}

#[test]
fn test_task_completion() {
    let mut task = Task::new(1, "Test task", Priority::Medium);
    assert_eq!(task.status(), Status::Pending);
    assert!(task.completed_at().is_none());

    task.complete();
    assert_eq!(task.status(), Status::Completed);
    assert!(task.completed_at().is_some());
}

#[test]
fn test_priority_from_str() {
    assert_eq!("high".parse::<Priority>().unwrap(), Priority::High);
    assert_eq!("LOW".parse::<Priority>().unwrap(), Priority::Low);
    assert_eq!("u".parse::<Priority>().unwrap(), Priority::Urgent);
    assert!("unknown".parse::<Priority>().is_err());
}

#[test]
fn test_task_manager_uses_injected_storage_path() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("tasks.json");

    let mut manager = TaskManager::with_storage_path(&storage_path).unwrap();
    let id = manager
        .add_task(Task::new(0, "Injected path task", Priority::Low))
        .unwrap();

    assert_eq!(id, 1);
    assert_eq!(manager.storage_path(), storage_path.as_path());
    assert!(storage_path.exists());
}

#[test]
fn test_task_manager_reloads_existing_tasks_from_injected_storage() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("tasks.json");

    {
        let mut manager = TaskManager::with_storage_path(&storage_path).unwrap();
        let task = Task::new(0, "Persisted task", Priority::Urgent)
            .with_description("should survive reload")
            .with_tags(vec!["persist".to_string()]);
        manager.add_task(task).unwrap();
    }

    let reloaded_manager = TaskManager::with_storage_path(&storage_path).unwrap();
    let tasks = reloaded_manager.list_tasks(None);

    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].title(), "Persisted task");
    assert_eq!(tasks[0].priority(), Priority::Urgent);
    assert_eq!(tasks[0].status(), Status::Pending);
    assert_eq!(tasks[0].tags(), ["persist".to_string()]);
}

#[test]
fn test_task_manager_persists_status_and_assigns_next_id_after_reload() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("tasks.json");

    {
        let mut manager = TaskManager::with_storage_path(&storage_path).unwrap();
        let first_id = manager
            .add_task(Task::new(0, "Persisted first", Priority::Medium))
            .unwrap();
        let second_id = manager
            .add_task(Task::new(0, "Persisted second", Priority::High))
            .unwrap();

        manager
            .complete_task(second_id)
            .expect("second task should be completed before reload");

        assert_eq!(first_id, 1);
        assert_eq!(second_id, 2);
    }

    let mut reloaded_manager = TaskManager::with_storage_path(&storage_path).unwrap();
    let reloaded_completed_task = reloaded_manager
        .get_task(2)
        .expect("completed task should be reloaded from storage");

    assert_eq!(reloaded_completed_task.status(), Status::Completed);
    assert!(
        reloaded_completed_task.completed_at().is_some(),
        "completed timestamp should survive persistence"
    );

    let next_id = reloaded_manager
        .add_task(Task::new(0, "Added after reload", Priority::Low))
        .unwrap();
    assert_eq!(next_id, 3, "next id should continue from persisted max id");
}

#[test]
fn test_task_manager_search_matches_title_description_and_tags_case_insensitively() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("tasks.json");
    let mut manager = TaskManager::with_storage_path(&storage_path).unwrap();

    manager
        .add_task(
            Task::new(0, "Write Rust tests", Priority::High)
                .with_description("Cover CLI parsing and aliases")
                .with_tags(vec!["quality".to_string(), "cli".to_string()]),
        )
        .unwrap();
    manager
        .add_task(
            Task::new(0, "Prepare release", Priority::Medium)
                .with_description("Coordinate roadmap with QA")
                .with_tags(vec!["release".to_string()]),
        )
        .unwrap();

    let title_matches = manager.search_tasks("rust");
    assert_eq!(title_matches.len(), 1);
    assert_eq!(title_matches[0].title(), "Write Rust tests");

    let description_matches = manager.search_tasks("ALIASES");
    assert_eq!(description_matches.len(), 1);
    assert_eq!(description_matches[0].title(), "Write Rust tests");

    let tag_matches = manager.search_tasks("Qa");
    assert_eq!(tag_matches.len(), 1);
    assert_eq!(tag_matches[0].title(), "Prepare release");
}

#[test]
fn test_task_manager_statistics_and_pending_filter_reflect_task_states() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("tasks.json");
    let mut manager = TaskManager::with_storage_path(&storage_path).unwrap();

    let urgent_pending = manager
        .add_task(Task::new(0, "Urgent pending", Priority::Urgent))
        .unwrap();
    let in_progress = manager
        .add_task(Task::new(0, "Work in progress", Priority::High))
        .unwrap();
    let completed = manager
        .add_task(Task::new(0, "Already done", Priority::Medium))
        .unwrap();
    let cancelled = manager
        .add_task(Task::new(0, "Cancelled item", Priority::Low))
        .unwrap();

    manager
        .start_task(in_progress)
        .expect("in-progress task should exist");
    manager
        .complete_task(completed)
        .expect("completed task should exist");
    manager
        .cancel_task(cancelled)
        .expect("cancelled task should exist");

    let pending_tasks = manager.list_tasks(Some(Status::Pending));
    assert_eq!(pending_tasks.len(), 1, "only one task should remain pending");
    assert_eq!(pending_tasks[0].id(), urgent_pending);
    assert_eq!(pending_tasks[0].priority(), Priority::Urgent);

    let stats = manager.get_statistics();
    assert_eq!(
        stats,
        TaskStatistics {
            total: 4,
            completed: 1,
            pending: 1,
            in_progress: 1,
            urgent: 1,
        },
        "statistics should reflect each task state and count unfinished urgent work"
    );
}

#[test]
fn test_list_tasks_orders_by_priority_before_recency() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("tasks.json");
    let mut manager = TaskManager::with_storage_path(&storage_path).unwrap();

    manager
        .add_task(Task::new(0, "Low priority", Priority::Low))
        .unwrap();
    manager
        .add_task(Task::new(0, "Urgent priority", Priority::Urgent))
        .unwrap();
    manager
        .add_task(Task::new(0, "High priority", Priority::High))
        .unwrap();
    manager
        .add_task(Task::new(0, "Medium priority", Priority::Medium))
        .unwrap();

    let ordered_titles: Vec<&str> = manager
        .list_tasks(None)
        .into_iter()
        .map(|task| task.title())
        .collect();

    assert_eq!(
        ordered_titles,
        vec![
            "Urgent priority",
            "High priority",
            "Medium priority",
            "Low priority",
        ],
        "list_tasks should present work in priority order for learners"
    );
}

#[test]
fn test_storage_load_missing_file_returns_not_found_outcome_without_creating_file() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("missing_tasks.json");
    let storage = TaskStorage::new(TaskStorageConfig::from_path(&storage_path)).unwrap();

    let outcome = storage
        .load_tasks()
        .expect("missing storage file should be treated as first-run state");

    assert!(
        matches!(outcome, TaskLoadOutcome::NotFound),
        "missing file should map to TaskLoadOutcome::NotFound"
    );
    assert!(
        !storage_path.exists(),
        "loading a missing file should not eagerly create the storage file"
    );
}

#[test]
fn test_storage_load_invalid_json_returns_parse_error() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("damaged_tasks.json");
    fs::write(&storage_path, "{ invalid json").unwrap();
    let storage = TaskStorage::new(TaskStorageConfig::from_path(&storage_path)).unwrap();

    let error = storage
        .load_tasks()
        .expect_err("damaged JSON should surface a parse error");

    assert!(
        matches!(&error, TaskLoadError::Parse { path, .. } if path == &storage_path),
        "damaged JSON should return TaskLoadError::Parse, got {error:?}"
    );
}

#[test]
fn test_storage_load_directory_path_returns_read_error() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("tasks_as_directory");
    fs::create_dir(&storage_path).unwrap();
    let storage = TaskStorage::new(TaskStorageConfig::from_path(&storage_path)).unwrap();

    let error = storage
        .load_tasks()
        .expect_err("reading from a directory path should fail as a read error");

    assert!(
        matches!(&error, TaskLoadError::Read { path, .. } if path == &storage_path),
        "directory-backed storage path should return TaskLoadError::Read, got {error:?}"
    );
}

#[test]
fn test_task_manager_load_state_distinguishes_first_run_from_reload() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("stateful_tasks.json");

    let empty_manager = TaskManager::with_storage_path(&storage_path).unwrap();
    assert_eq!(
        empty_manager.load_state(),
        TaskManagerLoadState::InitializedEmpty,
        "missing storage should initialize the manager in empty first-run mode"
    );
    assert!(
        empty_manager.list_tasks(None).is_empty(),
        "first-run initialization should start with no in-memory tasks"
    );
    drop(empty_manager);

    let mut seeded_manager = TaskManager::with_storage_path(&storage_path).unwrap();
    seeded_manager
        .add_task(Task::new(0, "Persisted after first run", Priority::High))
        .unwrap();
    drop(seeded_manager);

    let reloaded_manager = TaskManager::with_storage_path(&storage_path).unwrap();
    assert_eq!(
        reloaded_manager.load_state(),
        TaskManagerLoadState::LoadedFromStorage,
        "existing storage data should be reported as a successful reload"
    );
    assert_eq!(
        reloaded_manager.list_tasks(None).len(),
        1,
        "reloaded manager should hydrate persisted tasks"
    );
}

#[test]
fn test_add_task_returns_error_and_rolls_back_when_persist_fails() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("blocked_add.json");
    let mut manager = TaskManager::with_storage_path(&storage_path).unwrap();
    fs::create_dir(&storage_path).unwrap();

    let error = manager
        .add_task(Task::new(0, "should fail to persist", Priority::Medium))
        .expect_err("add_task should report persistence failures instead of succeeding silently");

    let error_message = format!("{error:#}");
    assert!(
        error_message.contains("Failed to persist newly added task"),
        "add_task error should preserve rollback context, got: {error_message}"
    );
    assert!(
        manager.list_tasks(None).is_empty(),
        "failed persistence should roll back the inserted task from memory"
    );
    assert!(
        manager.get_task(1).is_none(),
        "rolled-back add_task should not leave the failed task addressable"
    );

    fs::remove_dir(&storage_path).unwrap();
    let recovered_id = manager
        .add_task(Task::new(0, "succeeds after unblock", Priority::Low))
        .expect("manager should remain usable after rolling back a failed write");
    assert_eq!(
        recovered_id, 1,
        "failed persistence should also roll back next_id so ids are not skipped"
    );
}

#[test]
fn test_delete_task_returns_error_and_restores_task_when_persist_fails() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("blocked_delete.json");
    let mut manager = TaskManager::with_storage_path(&storage_path).unwrap();
    let task_id = manager
        .add_task(Task::new(0, "delete rollback", Priority::High))
        .unwrap();

    fs::remove_file(&storage_path).unwrap();
    fs::create_dir(&storage_path).unwrap();

    let error = manager
        .delete_task(task_id)
        .expect_err("delete_task should report persistence failures instead of succeeding silently");

    let error_message = format!("{error:#}");
    assert!(
        error_message.contains("Failed to persist task deletion"),
        "delete_task error should preserve rollback context, got: {error_message}"
    );
    let restored_task = manager
        .get_task(task_id)
        .expect("failed delete_task should restore the removed task in memory");
    assert_eq!(
        restored_task.title(), "delete rollback",
        "rollback should restore the original task contents after a failed delete"
    );
    assert_eq!(
        manager.list_tasks(None).len(),
        1,
        "failed delete_task should keep the task visible after rollback"
    );
}

#[test]
fn test_update_task_returns_error_and_restores_task_when_persist_fails() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("blocked_update.json");
    let mut manager = TaskManager::with_storage_path(&storage_path).unwrap();
    let task_id = manager
        .add_task(Task::new(0, "update rollback", Priority::High))
        .unwrap();

    fs::remove_file(&storage_path).unwrap();
    fs::create_dir(&storage_path).unwrap();

    let error = manager
        .complete_task(task_id)
        .expect_err("task update should report persistence failures instead of succeeding silently");

    let error_message = format!("{error:#}");
    assert!(
        error_message.contains("Failed to persist task update"),
        "task update error should preserve rollback context, got: {error_message}"
    );

    let restored_task = manager
        .get_task(task_id)
        .expect("failed update should restore the original task in memory");
    assert_eq!(
        restored_task.status(),
        Status::Pending,
        "rollback should restore the original task status after a failed update"
    );
    assert!(
        restored_task.completed_at().is_none(),
        "rollback should also restore completion metadata"
    );
}

#[test]
fn test_storage_save_replaces_existing_file_without_leaving_temp_file() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("atomic_tasks.json");
    let storage = TaskStorage::new(TaskStorageConfig::from_path(&storage_path)).unwrap();

    let mut first_tasks = std::collections::HashMap::new();
    first_tasks.insert(1, Task::new(1, "first", Priority::Low));
    storage.save_tasks(&first_tasks).unwrap();

    let mut second_tasks = std::collections::HashMap::new();
    let mut replacement = Task::new(2, "second", Priority::High);
    replacement.complete();
    second_tasks.insert(2, replacement);
    storage.save_tasks(&second_tasks).unwrap();

    let reloaded = match storage.load_tasks().unwrap() {
        TaskLoadOutcome::Loaded(tasks) => tasks,
        TaskLoadOutcome::NotFound => panic!("saved storage should exist"),
    };

    assert_eq!(reloaded.len(), 1);
    let task = reloaded.get(&2).expect("replacement task should be present");
    assert_eq!(task.title(), "second");
    assert_eq!(task.status(), Status::Completed);

    let entries = fs::read_dir(temp_dir.path())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    assert_eq!(entries, vec!["atomic_tasks.json".to_string()]);
}
