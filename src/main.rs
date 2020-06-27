use libcli::args;

fn main() {
    let specs = [
        args::OptionSpec::new(
            '\0',
            "(unnamed)",
            "Specify which files and/or directories to watch",
            true,
            args::OptionPolicy::AtLeast(1),
        ),
        args::OptionSpec::new(
            'h',
            "help",
            "Show usage",
            false,
            args::OptionPolicy::FinalizeIgnore(),
        ),
        args::OptionSpec::new(
            'p',
            "print",
            "Print modified files and directories to stdout",
            false,
            args::OptionPolicy::FinalizeIgnore(),
        ),
        args::OptionSpec::new(
            'i',
            "interval",
            "Time in milliseconds between checks (default 100)",
            false,
            args::OptionPolicy::Exact(1),
        ),
    ];
    let config = match args::Config::new_env(&specs) {
        Ok(config) => config,
        Err(msg) => {
            println!("{}\nUse --help for usage", msg);
            std::process::exit(1)
        }
    };

    if let Some(_) = config.option("help") {
        println!(
            "rswatch - watch files for changes and execute commands\n\n{}",
            args::Config::generate_usage(&specs, true, true)
        );
        return;
    }

    watch::run(config);
}
