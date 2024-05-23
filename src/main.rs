use clap::Parser;
use std::process;
use traffic_manager::arguments::Arguments;
use traffic_manager::manager::Manager;

fn main() {
    // Parse the arguments
    let args = Arguments::parse();

    // Validate args
    let args = Arguments::validate(&args).unwrap_or_else(|err| {
        eprintln!("Problem validating arguments: {err}");
        process::exit(1)
    });

    // Build the manager based on args
    let manager = Manager::new(&args);

    // Run the manager
    manager.start().unwrap_or_else(|err| {
        eprintln!("Problem running manager: {err}");
        process::exit(1)
    });
}
