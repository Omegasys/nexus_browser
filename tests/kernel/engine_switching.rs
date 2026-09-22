#[cfg(test)]
mod tests {
    use nexus_browser::browser_kernel::engine_switcher::EngineSwitcher;

    #[test]
    fn switcher_can_be_created() {
        let switcher = EngineSwitcher::new();

        assert!(switcher.current_engine().is_none());
    }

    #[test]
    fn engine_can_be_selected() {
        let mut switcher = EngineSwitcher::new();

        switcher
            .register_engine("blink")
            .expect("engine registration should succeed");

        assert!(
            switcher
                .switch_to("blink")
                .is_ok()
        );

        assert_eq!(
            switcher.current_engine(),
            Some("blink")
        );
    }

    #[test]
    fn unknown_engine_cannot_be_selected() {
        let mut switcher = EngineSwitcher::new();

        assert!(
            switcher
                .switch_to("missing")
                .is_err()
        );
    }

    #[test]
    fn engine_switch_can_be_repeated() {
        let mut switcher = EngineSwitcher::new();

        switcher
            .register_engine("blink")
            .expect("registration should succeed");

        switcher
            .register_engine("servo")
            .expect("registration should succeed");

        switcher
            .switch_to("blink")
            .expect("switch should succeed");

        switcher
            .switch_to("servo")
            .expect("switch should succeed");

        assert_eq!(
            switcher.current_engine(),
            Some("servo")
        );
    }
}
