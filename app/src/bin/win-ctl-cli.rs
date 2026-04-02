fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|arg| arg == "--version" || arg == "-V") {
        println!("win-ctl-cli {version}");
        return;
    }

    if args.iter().any(|arg| arg == "self-update") {
        println!("win-ctl-cli self-update: skeleton placeholder");
        return;
    }

    println!("win-ctl-cli: repository skeleton");
}
