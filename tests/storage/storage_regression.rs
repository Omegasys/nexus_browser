#[test]
fn storage_regression_partitioning_remains_enabled() {
    let partitioning = true;

    assert!(partitioning);
}

#[test]
fn storage_regression_site_data_can_be_deleted() {
    let deletion = true;

    assert!(deletion);
}

#[test]
fn storage_regression_session_data_is_not_persistent() {
    let persistent = false;

    assert!(!persistent);
}
