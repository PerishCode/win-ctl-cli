use clap::Parser;

fn main() {
    let cli = win_ctl_cli::cli::Cli::parse();
    let profile = match win_ctl_cli::profile::load(cli.runtime.profile.clone()) {
        Ok(profile) => profile,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };

    let runtime = win_ctl_cli::core::build_runtime_config(&cli, profile);
    win_ctl_cli::logging::init(runtime.quiet, runtime.verbose, runtime.log_level);

    if let Err(err) = win_ctl_cli::core::dispatch(cli, runtime) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
