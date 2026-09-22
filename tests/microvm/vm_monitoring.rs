#[cfg(test)]
mod tests {
    use nexus_browser::microvm::vm_monitor::VmMonitor;

    #[test]
    fn monitor_can_be_created() {
        let monitor = VmMonitor::new();

        assert_eq!(monitor.monitored_count(), 0);
    }

    #[test]
    fn vm_can_be_monitored() {
        let mut monitor = VmMonitor::new();

        monitor
            .monitor("vm-1")
            .expect("monitoring should start");

        assert_eq!(monitor.monitored_count(), 1);
        assert!(monitor.is_monitored("vm-1"));
    }

    #[test]
    fn vm_can_be_removed_from_monitoring() {
        let mut monitor = VmMonitor::new();

        monitor
            .monitor("vm-1")
            .expect("monitoring should start");

        monitor
            .unmonitor("vm-1")
            .expect("monitoring should stop");

        assert!(!monitor.is_monitored("vm-1"));
    }
}
