#[cfg(test)]
mod tests {
    use nexus_browser::microvm::vm_escape_detection::{
        EscapeEvent,
        EscapeSeverity,
        VmEscapeDetector,
    };

    #[test]
    fn detector_starts_clean() {
        let detector = VmEscapeDetector::new();

        assert_eq!(
            detector.event_count(),
            0
        );

        assert!(!detector.has_detected_escape());
    }

    #[test]
    fn suspicious_event_can_be_recorded() {
        let mut detector = VmEscapeDetector::new();

        detector.record(
            EscapeEvent::new(
                "vm-1",
                EscapeSeverity::Warning,
                "unexpected host access",
            ),
        );

        assert_eq!(
            detector.event_count(),
            1
        );
    }

    #[test]
    fn confirmed_escape_is_detected() {
        let mut detector = VmEscapeDetector::new();

        detector.record(
            EscapeEvent::new(
                "vm-1",
                EscapeSeverity::Critical,
                "confirmed isolation boundary violation",
            ),
        );

        assert!(
            detector.has_detected_escape()
        );
    }

    #[test]
    fn vm_can_be_quarantined_after_escape() {
        let mut detector = VmEscapeDetector::new();

        detector.record(
            EscapeEvent::new(
                "vm-1",
                EscapeSeverity::Critical,
                "confirmed escape",
            ),
        );

        detector
            .quarantine("vm-1")
            .expect("quarantine should succeed");

        assert!(detector.is_quarantined("vm-1"));
    }
}
