use coomi_services::ProgressiveMemoryStore;

#[test]
fn appends_each_turn_once_and_marks_summary_due() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let store = ProgressiveMemoryStore::open(directory.path(), "dlg-test", 2, 1_000)
        .expect("open store");

    let first = store
        .append_turn("turn-1", "remember the project decision", "recorded")
        .expect("append first turn");
    assert!(!first.summary_due);

    let second = store
        .append_turn("turn-2", "the next task is build the APK", "queued")
        .expect("append second turn");
    assert!(second.summary_due);
    assert_eq!(second.turn_number, 2);

    let duplicate = store
        .append_turn("turn-2", "the next task is build the APK", "queued")
        .expect("duplicate append is idempotent");
    assert_eq!(duplicate.turn_number, 2);
    assert_eq!(store.raw_turn_count().expect("count turns"), 2);
}

#[test]
fn context_is_recalled_with_a_hard_character_budget() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let store = ProgressiveMemoryStore::open(directory.path(), "dlg-test", 10, 80)
        .expect("open store");
    store
        .append_turn(
            "turn-1",
            "project memory alpha should be recalled",
            "assistant alpha details",
        )
        .expect("append turn");

    let context = store
        .context("project memory alpha")
        .expect("build context");
    assert!(context.len() <= 80);
    assert!(context.contains("project memory alpha"));
}
