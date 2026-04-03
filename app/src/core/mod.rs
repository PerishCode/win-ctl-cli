mod runtime_config;

pub use runtime_config::RuntimeConfig;

use crate::cli::{Cli, Command};

pub fn build_runtime_config(cli: &Cli, profile: crate::profile::Profile) -> RuntimeConfig {
    RuntimeConfig::from_cli(cli, profile)
}

pub fn dispatch(cli: Cli, runtime: RuntimeConfig) -> Result<(), String> {
    match cli.command {
        Command::Compose(command) => crate::commands::compose::run(&runtime, &command.file),
        Command::Window(command) => match command.command {
            crate::cli::WindowSubcommand::List { json } => {
                crate::commands::window::list(&runtime, json)
            }
            crate::cli::WindowSubcommand::Active {
                json,
                bounds,
                class,
                pid,
            } => {
                crate::cli::WindowSubcommand::Active {
                    json,
                    bounds,
                    class,
                    pid,
                }
                .validate()?;
                crate::commands::window::active(&runtime, json, bounds, class, pid)
            }
            crate::cli::WindowSubcommand::Bounds { hwnd, json } => {
                crate::commands::window::bounds(&runtime, hwnd, json)
            }
        },
        Command::Screen(command) => match command.command {
            screen_command => {
                screen_command.validate()?;
                match screen_command {
                    crate::cli::ScreenSubcommand::Capture {
                        target,
                        hwnd,
                        format,
                        output,
                    } => match target {
                        crate::cli::CaptureTarget::Active => {
                            crate::commands::screen::capture_target_active(
                                &runtime,
                                &output,
                                format.as_ref().map(|value| match value {
                                    crate::cli::CaptureFormat::Png => "png",
                                    crate::cli::CaptureFormat::Bmp => "bmp",
                                }),
                            )
                        }
                        crate::cli::CaptureTarget::Window => {
                            crate::commands::screen::capture_target_window(
                                &runtime,
                                &output,
                                hwnd.expect("validated by CLI"),
                                format.as_ref().map(|value| match value {
                                    crate::cli::CaptureFormat::Png => "png",
                                    crate::cli::CaptureFormat::Bmp => "bmp",
                                }),
                            )
                        }
                    },
                }
            }
        },
        Command::SelfUpdate => {
            println!("win-ctl-cli self-update: skeleton placeholder");
            Ok(())
        }
    }
}
