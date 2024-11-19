use minils;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        match args[1].as_str() {
            "--help" | "-?" => {
                println!("{help_msg}", help_msg = minils::HELP);
                return;
            }
            "--version" | "-v" => {
                println!(
                    "{name} - {description}",
                    name = env!("CARGO_PKG_NAME"),
                    description = env!("CARGO_PKG_DESCRIPTION")
                );
                println!("v{version}", version = env!("CARGO_PKG_VERSION"));
                return;
            }
            _ => (),
        }
    }

    let mut display_options = minils::DisplayOptions {
        oneline: false,
        grid: true,
        long: false,
        recurse: false,
    };

    let mut filtering_options = minils::FilteringOptions {
        all: false,
        list_dirs: false,
        only_dirs: false,
        only_files: false,
    };

    let entries = minils::parse_arguments(&args, &mut display_options, &mut filtering_options);

    minils::run(entries, display_options, filtering_options);
}
