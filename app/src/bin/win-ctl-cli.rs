use win_ctl_cli::commands;

const SCREEN_CAPTURE_USAGE: &str =
    "usage: win-ctl-cli screen capture [--format png|bmp] <output.png|output.bmp>";

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|arg| arg == "--version" || arg == "-V") {
        println!("win-ctl-cli {version}");
        return;
    }

    if args.get(0).map(|s| s.as_str()) == Some("window")
        && args.get(1).map(|s| s.as_str()) == Some("list")
    {
        let json = args.iter().any(|arg| arg == "--json");
        if let Err(err) = commands::window::list(json) {
            eprintln!("{err}");
            std::process::exit(1);
        }
        return;
    }

    if args.get(0).map(|s| s.as_str()) == Some("screen")
        && args.get(1).map(|s| s.as_str()) == Some("capture")
    {
        match capture_output_path_and_format(&args) {
            Ok((path, format)) => {
                let result = match format {
                    Some(format) => commands::screen::capture_with_format(&path, &format),
                    None => commands::screen::capture(&path),
                };
                if let Err(err) = result {
                    eprintln!("{err}");
                    std::process::exit(1);
                }
            }
            Err(err) => {
                eprintln!("{err}");
                std::process::exit(1);
            }
        }
        return;
    }

    if args.get(0).map(|s| s.as_str()) == Some("window")
        && args.get(1).map(|s| s.as_str()) == Some("active")
    {
        let json = args.iter().any(|arg| arg == "--json");
        if let Err(err) = commands::window::active(json) {
            eprintln!("{err}");
            std::process::exit(1);
        }
        return;
    }

    if args.iter().any(|arg| arg == "self-update") {
        println!("win-ctl-cli self-update: skeleton placeholder");
        return;
    }

    println!("win-ctl-cli: repository skeleton");
}

fn capture_output_path_and_format(
    args: &[String],
) -> Result<(std::path::PathBuf, Option<String>), String> {
    let mut positional = Vec::new();
    let mut format = None;
    let mut iter = args.iter().skip(2);

    while let Some(arg) = iter.next() {
        if arg == "--json" {
            return Err(String::from("--json is not supported for screen capture"));
        }
        if arg == "--format" {
            let value = iter
                .next()
                .ok_or_else(|| String::from(SCREEN_CAPTURE_USAGE))?;
            let normalized = value.to_ascii_lowercase();
            if normalized != "png" && normalized != "bmp" {
                return Err(String::from("unsupported format: use png or bmp"));
            }
            format = Some(normalized);
            continue;
        }
        if arg.starts_with('-') {
            return Err(String::from(SCREEN_CAPTURE_USAGE));
        }
        positional.push(arg.clone());
    }

    if positional.len() != 1 {
        return Err(String::from(SCREEN_CAPTURE_USAGE));
    }

    let output_path = std::path::PathBuf::from(&positional[0]);
    if let Some(format) = &format {
        let path_format = match output_path.extension().and_then(|value| value.to_str()) {
            Some(ext) if ext.eq_ignore_ascii_case("png") => Some("png"),
            Some(ext) if ext.eq_ignore_ascii_case("bmp") => Some("bmp"),
            Some(_) => None,
            None => None,
        };

        if path_format.is_none() {
            return Err(String::from(
                "output file extension must be .png or .bmp when --format is set",
            ));
        }

        if path_format != Some(format.as_str()) {
            return Err(String::from(
                "--format must match the output file extension (.png or .bmp)",
            ));
        }
    }

    Ok((output_path, format))
}

#[cfg(test)]
mod tests {
    use super::capture_output_path_and_format;
    use std::path::PathBuf;

    #[test]
    fn capture_accepts_extension_only() {
        let args = vec!["screen".into(), "capture".into(), "shot.png".into()];
        let parsed = capture_output_path_and_format(&args).expect("should parse png path");

        assert_eq!(parsed, (PathBuf::from("shot.png"), None));
    }

    #[test]
    fn capture_normalizes_format_override() {
        let args = vec![
            "screen".into(),
            "capture".into(),
            "shot.png".into(),
            "--format".into(),
            "PNG".into(),
        ];
        let parsed = capture_output_path_and_format(&args).expect("should parse uppercase format");

        assert_eq!(
            parsed,
            (PathBuf::from("shot.png"), Some(String::from("png")))
        );
    }

    #[test]
    fn capture_rejects_unsupported_format_override() {
        let args = vec![
            "screen".into(),
            "capture".into(),
            "shot.png".into(),
            "--format".into(),
            "gif".into(),
        ];

        let err = capture_output_path_and_format(&args).expect_err("should reject format");
        assert_eq!(err, "unsupported format: use png or bmp");
    }

    #[test]
    fn capture_rejects_mismatched_extension_and_override() {
        let args = vec![
            "screen".into(),
            "capture".into(),
            "shot.bmp".into(),
            "--format".into(),
            "png".into(),
        ];

        let err = capture_output_path_and_format(&args).expect_err("should reject mismatch");
        assert_eq!(
            err,
            "--format must match the output file extension (.png or .bmp)"
        );
    }

    #[test]
    fn capture_rejects_json_flag() {
        let args = vec![
            "screen".into(),
            "capture".into(),
            "shot.png".into(),
            "--json".into(),
        ];

        let err = capture_output_path_and_format(&args).expect_err("should reject json");
        assert_eq!(err, "--json is not supported for screen capture");
    }
}
