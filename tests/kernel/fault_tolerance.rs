#[cfg(test)]
mod tests {
    use nexus_browser::browser_kernel::fault_tolerance::FaultTolerance;

    #[test]
    fn fault_tolerance_can_be_created() {
        let manager = FaultTolerance::new();

        assert!(manager.is_enabled());
    }

    #[test]
    fn failure_can_be_recorded() {
        let mut manager = FaultTolerance::new();

        manager.record_failure(
            "renderer",
            "renderer crashed",
        );

        assert_eq!(
            manager.failure_count(),
            1
        );
    }

    #[test]
    fn repeated_failures_can_be_detected() {
        let mut manager = FaultTolerance::new();

        manager.record_failure(
            "renderer",
            "crash",
        );

        manager.record_failure(
            "renderer",
            "crash",
        );

        assert!(
            manager.has_repeated_failure("renderer")
        );
    }
}
