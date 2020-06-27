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
            'v',
            "verbose",
            "Verbose output",
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
        args::OptionSpec::new(
            'k',
            "kill",
            "Specifies to kill the child process `exec` rather than wait for it",
            false,
            args::OptionPolicy::Exact(0),
        ),
        args::OptionSpec::new(
            'c',
            "clear",
            "Clears the screen before restarting childprocess `exec`",
            false,
            args::OptionPolicy::Exact(0),
        ),
        args::OptionSpec::new(
            's', 
            "start",
             "Start the process `exec` on startup instead of waiting for a file to change before doing so", 
             false, 
             args::OptionPolicy::Exact(0)
        ),
        args::OptionSpec::new(
            'e',
            "exec",
            "Command to execute when files change\nAll following arguments are given to the specified command\n {} is to be replaced by the changed files, to run command for each changed file separately\nWatch is not blocked during execution when process is spawned but will wait until previous finished before rerunning next check\nTo kill and restart process rather than waiting, use option --kill",
            false,
            args::OptionPolicy::Finalize(),
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
