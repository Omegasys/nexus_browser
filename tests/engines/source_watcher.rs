use nexus_browser::engines::source_watcher::SourceWatcher;

#[test]
fn source_watcher_can_be_created() {
    let watcher = SourceWatcher::new();

    assert!(!watcher.is_watching());
}

#[test]
fn source_watcher_can_start_and_stop() {
    let mut watcher = SourceWatcher::new();

    watcher.start();

    assert!(watcher.is_watching());

    watcher.stop();

    assert!(!watcher.is_watching());
}
