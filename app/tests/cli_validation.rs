use clap::Parser;

use win_ctl_cli::cli::Cli;

#[test]
fn capture_active_rejects_hwnd() {
    let cli = Cli::try_parse_from([
        "win-ctl-cli",
        "screen",
        "capture",
        "--target",
        "active",
        "--hwnd",
        "12",
        "out.png",
    ])
    .expect("expected clap to parse active capture with hwnd");

    let err = match cli.command {
        win_ctl_cli::cli::Command::Screen(screen) => screen.command.validate(),
        _ => unreachable!("test input should parse to screen capture"),
    };

    assert!(err.unwrap_err().contains("--hwnd"));
}

#[test]
fn window_active_rejects_conflicting_query_flags() {
    let cli = Cli::try_parse_from(["win-ctl-cli", "window", "active", "--bounds", "--class"])
        .expect("expected clap to parse active window flags");

    let err = match cli.command {
        win_ctl_cli::cli::Command::Window(window) => window.command.validate(),
        _ => unreachable!("test input should parse to window active"),
    };

    assert!(err.unwrap_err().contains("mutually exclusive"));
}

#[test]
fn window_active_rejects_json_for_class_and_pid() {
    for args in [
        ["win-ctl-cli", "window", "active", "--class", "--json"],
        ["win-ctl-cli", "window", "active", "--pid", "--json"],
    ] {
        let cli = Cli::try_parse_from(args).expect("expected clap to parse active window flags");
        let err = match cli.command {
            win_ctl_cli::cli::Command::Window(window) => window.command.validate(),
            _ => unreachable!("test input should parse to window active"),
        };
        assert!(
            err.unwrap_err()
                .contains("only valid with identity or bounds output")
        );
    }
}

#[test]
fn capture_window_requires_hwnd() {
    let cli = Cli::try_parse_from([
        "win-ctl-cli",
        "screen",
        "capture",
        "--target",
        "window",
        "out.png",
    ])
    .expect("expected clap to parse window capture without hwnd");

    let err = match cli.command {
        win_ctl_cli::cli::Command::Screen(screen) => screen.command.validate(),
        _ => unreachable!("test input should parse to screen capture"),
    };

    assert!(err.unwrap_err().contains("--hwnd"));
}

#[test]
fn compose_accepts_active_window_class_query() {
    let doc = serde_json::json!({
        "kind": "compose",
        "steps": [{"target": "window.active", "query": "class"}]
    });

    let parsed: win_ctl_cli::commands::compose::ComposeDocument =
        serde_json::from_value(doc).expect("expected compose document to parse");
    assert_eq!(parsed.steps[0].query, "class");
}

#[test]
fn compose_accepts_window_bounds_query_with_hwnd_input() {
    let doc = serde_json::json!({
        "kind": "compose",
        "steps": [{"target": "window.bounds", "query": "bounds", "input": {"hwnd": 123}}]
    });

    let parsed: win_ctl_cli::commands::compose::ComposeDocument =
        serde_json::from_value(doc).expect("expected compose document to parse");
    assert_eq!(parsed.steps[0].target, "window.bounds");
    assert_eq!(parsed.steps[0].input.as_ref().unwrap().hwnd, 123);
}
