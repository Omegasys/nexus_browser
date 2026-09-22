#[cfg(test)]
mod tests {
    use nexus_browser::browser_kernel::compatibility_layer::CompatibilityLayer;

    #[test]
    fn compatibility_layer_can_be_created() {
        let layer = CompatibilityLayer::new();

        assert!(layer.is_enabled());
    }

    #[test]
    fn compatibility_mode_can_be_selected() {
        let mut layer = CompatibilityLayer::new();

        layer
            .set_mode("balanced")
            .expect("mode should be accepted");

        assert_eq!(
            layer.mode(),
            "balanced"
        );
    }

    #[test]
    fn unknown_mode_is_rejected() {
        let mut layer = CompatibilityLayer::new();

        assert!(
            layer
                .set_mode("invalid")
                .is_err()
        );
    }
}
