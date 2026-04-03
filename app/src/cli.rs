use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "win-ctl-cli", version, disable_help_subcommand = true)]
pub struct Cli {
    #[command(flatten)]
    pub runtime: RuntimeArgs,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Args, Clone)]
pub struct RuntimeArgs {
    #[arg(long)]
    pub profile: Option<PathBuf>,

    #[arg(long, global = true, default_value_t = false)]
    pub quiet: bool,

    #[arg(short = 'v', long = "verbose", global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[arg(long, global = true, value_enum)]
    pub log_level: Option<LogLevel>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Window(WindowCommand),
    Screen(ScreenCommand),
    Compose(ComposeCommand),
    SelfUpdate,
}

#[derive(Debug, Args)]
pub struct WindowCommand {
    #[command(subcommand)]
    pub command: WindowSubcommand,
}

#[derive(Debug, Clone, Subcommand)]
pub enum WindowSubcommand {
    List {
        #[arg(long)]
        json: bool,
    },
    Active {
        #[arg(long)]
        json: bool,

        #[arg(long)]
        bounds: bool,

        #[arg(long)]
        class: bool,

        #[arg(long)]
        pid: bool,
    },
    Bounds {
        hwnd: isize,

        #[arg(long)]
        json: bool,
    },
}

impl WindowSubcommand {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            WindowSubcommand::Active {
                json,
                bounds,
                class,
                pid,
            } => {
                let selected = [*bounds, *class, *pid]
                    .into_iter()
                    .filter(|value| *value)
                    .count();
                if selected > 1 {
                    return Err(String::from(
                        "--bounds, --class, and --pid are mutually exclusive",
                    ));
                }

                if (*class || *pid) && *json {
                    return Err(String::from(
                        "--json is only valid with identity or bounds output",
                    ));
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Args)]
pub struct ScreenCommand {
    #[command(subcommand)]
    pub command: ScreenSubcommand,
}

#[derive(Debug, Args)]
pub struct ComposeCommand {
    #[arg(short = 'f', long = "file")]
    pub file: PathBuf,
}

#[derive(Debug, Subcommand)]
pub enum ScreenSubcommand {
    /// Capture the screen or a window.
    Capture {
        #[arg(long, value_enum)]
        target: CaptureTarget,

        #[arg(long, requires = "target")]
        hwnd: Option<isize>,

        #[arg(long, value_enum)]
        format: Option<CaptureFormat>,

        output: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum CaptureTarget {
    Active,
    Window,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CaptureFormat {
    Png,
    Bmp,
}

impl ScreenSubcommand {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            ScreenSubcommand::Capture { target, hwnd, .. } => match target {
                CaptureTarget::Active if hwnd.is_some() => {
                    Err(String::from("--hwnd is only valid with --target window"))
                }
                CaptureTarget::Window if hwnd.is_none() => Err(String::from(
                    "--hwnd is required when --target window is used",
                )),
                _ => Ok(()),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
