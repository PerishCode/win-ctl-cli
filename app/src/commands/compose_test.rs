#[cfg(test)]
mod tests {
    use super::super::compose::ComposeDocument;

    #[test]
    fn parses_minimal_compose_document() {
        let doc: ComposeDocument = serde_json::from_str(
            r#"{"kind":"compose","steps":[{"target":"window.active","query":"bounds"}]}"#,
        )
        .unwrap();
        assert_eq!(doc.kind, "compose");
        assert_eq!(doc.steps.len(), 1);
        assert_eq!(doc.steps[0].target, "window.active");
        assert_eq!(doc.steps[0].query, "bounds");
    }

    #[test]
    fn parses_active_window_class_query() {
        let doc: ComposeDocument = serde_json::from_str(
            r#"{"kind":"compose","steps":[{"target":"window.active","query":"class"}]}"#,
        )
        .unwrap();
        assert_eq!(doc.steps[0].target, "window.active");
        assert_eq!(doc.steps[0].query, "class");
    }

    #[test]
    fn parses_active_window_pid_query() {
        let doc: ComposeDocument = serde_json::from_str(
            r#"{"kind":"compose","steps":[{"target":"window.active","query":"pid"}]}"#,
        )
        .unwrap();
        assert_eq!(doc.steps[0].target, "window.active");
        assert_eq!(doc.steps[0].query, "pid");
    }

    #[test]
    fn parses_window_bounds_compose_step_with_hwnd_input() {
        let doc: ComposeDocument = serde_json::from_str(
            r#"{"kind":"compose","steps":[{"target":"window.bounds","query":"bounds","input":{"hwnd":123}}]}"#,
        )
        .unwrap();
        assert_eq!(doc.steps[0].target, "window.bounds");
        assert_eq!(doc.steps[0].query, "bounds");
        assert_eq!(doc.steps[0].input.as_ref().unwrap().hwnd, 123);
    }
}
