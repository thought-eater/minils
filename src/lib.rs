use std::error::Error;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::{fs, process};

pub const HELP: &str = "\
List directory contents.
Ignore files and directories starting with a '.' by default

Usage: minils [options] [path]

META OPTIONS
  -?, --help
          show list of command-line options
  -v, --version
          show version of minils

Display Options
  -1, --oneline
          display one entry per line
  -l, --long
          display extended file metadata as a table
  -G, --grid
          display entries as a grid (default)


Filtering Options
  -a, --all
          show hidden and 'dot' files
  -d, --list-dirs
          list directories as files; don't list their contents
  -D, --only-dirs
          list only directories
  -f, --only-files
          list only files
";

fn err_handling<T, E: Error>(err: E) -> T {
    eprintln!("{err}");
    process::exit(1);
}

pub struct DisplayOptions {
    pub oneline: bool,
    pub grid: bool,
    pub long: bool,
    pub recurse: bool,
}

pub struct FilteringOptions {
    pub all: bool,
    pub list_dirs: bool,
    pub only_dirs: bool,
    pub only_files: bool,
}

pub fn parse_arguments(
    args: &Vec<String>,
    display_options: &mut DisplayOptions,
    filtering_options: &mut FilteringOptions,
) -> fs::ReadDir {
    let mut args_iter = args.iter();
    args_iter.next(); // No need to check first argument, it is the name of the program

    for (i, element) in args_iter.enumerate() {
        if element.starts_with("--") {
            match element.as_str() {
                "--oneline" => {
                    display_options.oneline = true;
                    display_options.grid = false;
                }
                "--long" => {
                    display_options.long = true;
                    display_options.oneline = true;
                    display_options.grid = false;
                }
                "--grid" => {
                    display_options.grid = true;
                    display_options.long = false;
                    display_options.oneline = false;
                }
                "--recurse" => {
                    display_options.recurse = true;
                    display_options.grid = false;
                }
                "--all" => filtering_options.all = true,
                "--list-dirs" => filtering_options.list_dirs = true,
                "--only-dirs" => {
                    filtering_options.only_dirs = true;
                    filtering_options.all = false;
                }
                "--only-files" => {
                    filtering_options.only_files = true;
                    filtering_options.all = false;
                }
                option => {
                    eprintln!(
                        "{}: Invalid option. For help, try running 'minils --help'",
                        option
                    );
                    process::exit(1);
                }
            }
        } else if element.starts_with("-") {
            let options = element.as_bytes();

            if options.len() < 2 {
                eprintln!("Option not specified. For help, try running 'minils --help'");
                process::exit(1);
            }

            let mut options_iter = options.iter();
            options_iter.next();

            for &option in options_iter {
                match option {
                    b'1' => {
                        display_options.oneline = true;
                        display_options.grid = false;
                    }
                    b'l' => {
                        display_options.long = true;
                        display_options.oneline = true;
                        display_options.grid = false;
                    }
                    b'G' => {
                        display_options.grid = true;
                        display_options.long = false;
                        display_options.oneline = false;
                    }
                    b'R' => {
                        display_options.recurse = true;
                        display_options.grid = false;
                    }
                    b'a' => filtering_options.all = true,
                    b'd' => filtering_options.list_dirs = true,
                    b'D' => {
                        filtering_options.only_dirs = true;
                        filtering_options.only_files = false;
                        filtering_options.all = false;
                    }
                    b'f' => {
                        filtering_options.only_files = true;
                        filtering_options.only_dirs = false;
                        filtering_options.all = false;
                    }
                    invalid_option => {
                        eprintln!(
                            "{}: Invalid option. For help, try running 'minils --help'",
                            invalid_option as char
                        );
                        process::exit(1);
                    }
                }
            }
        } else if i == args.len() - 1 {
            let metadata = fs::metadata(&args[i]).unwrap_or_else(err_handling);
            let entry_name = &args[i];
            let entry_color: &str;
            let reset = "\x1b[0m";

            if metadata.is_dir() && filtering_options.list_dirs {
                entry_color = "\x1b[1;34m"; // bold blue
            } else if metadata.is_dir() {
                return fs::read_dir(&args[i]).unwrap_or_else(err_handling);
            } else if metadata.is_symlink() {
                entry_color = "\x1b[1;96m";
            } else {
                entry_color = "\x1b[1m";
            }

            if display_options.long {
                let permissions_mode = metadata.permissions().mode();
                let read_color = "\x1b[33m"; // yellow
                let write_color = "\x1b[31m"; // red
                let execute_color = "\x1b[32m"; // green
                let is_user = "\x1b[1m"; // bold

                println!("\x1b[4mPermissions\x1b[0m  \x1b[4mSize\x1b[0m  \x1b[4mName\x1b[0m");

                if metadata.is_dir() {
                    print!("{entry_color}d{reset}");
                } else if metadata.is_file() {
                    print!("{entry_color}-{reset}")
                } else {
                    print!("{entry_color}l{reset}")
                }

                // User permissions
                if permissions_mode & 0b100_000_000 == 0b100_000_000 {
                    print!("{is_user}{read_color}r{reset}");
                } else {
                    print!("{is_user}{read_color}-{reset}");
                }
                if permissions_mode & 0b010_000_000 == 0b010_000_000 {
                    print!("{is_user}{write_color}w{reset}");
                } else {
                    print!("{is_user}{write_color}-{reset}");
                }
                if permissions_mode & 0b001_000_000 == 0b001_000_000 {
                    print!("{is_user}{execute_color}x{reset}");
                } else {
                    print!("{is_user}{execute_color}-{reset}");
                }

                // Group permissions
                if permissions_mode & 0b000_100_000 == 0b000_100_000 {
                    print!("{read_color}r{reset}");
                } else {
                    print!("{read_color}-{reset}");
                }
                if permissions_mode & 0b000_010_000 == 0b000_010_000 {
                    print!("{write_color}w{reset}");
                } else {
                    print!("{write_color}-{reset}");
                }
                if permissions_mode & 0b000_001_000 == 0b000_001_000 {
                    print!("{execute_color}x{reset}");
                } else {
                    print!("{execute_color}-{reset}");
                }

                // Other permissions
                if permissions_mode & 0b000_000_100 == 0b000_000_100 {
                    print!("{read_color}r{reset}");
                } else {
                    print!("{read_color}-{reset}");
                }
                if permissions_mode & 0b000_000_010 == 0b000_000_010 {
                    print!("{write_color}w{reset}");
                } else {
                    print!("{write_color}-{reset}");
                }
                if permissions_mode & 0b000_000_001 == 0b000_000_001 {
                    print!("{execute_color}x{reset}");
                } else {
                    print!("{execute_color}-{reset}");
                }

                print!("{padding:<2}", padding = "");

                let size = metadata.size();

                if !metadata.is_dir() {
                    if size < 10u64.pow(3) {
                        print!(" {size:>3}B");
                    } else if size < 10u64.pow(6) {
                        print!("{size:>3}KB", size = size / 10u64.pow(6));
                    } else if size < 10u64.pow(9) {
                        print!("{size:>3}MB", size = size / 10u64.pow(9));
                    } else if size < 10u64.pow(12) {
                        print!("{size:>3}GB", size = size / 10u64.pow(12));
                    } else {
                        print!("{size:>3}TB", size = size / 10u64.pow(15));
                    }
                } else {
                    print!("{:>5}", "-");
                }
                print!("{padding:<2}", padding = "");
            }

            if display_options.long && metadata.is_symlink() {
                let real_path = fs::read_link(&args[i]).unwrap_or_else(err_handling);
                print!(
                    "{entry_color}{entry_name}{reset} -> {real_path_color}{real_path}{reset}",
                    real_path = real_path.display(),
                    real_path_color = "\x1b[0;31m", // regular red
                );
            } else {
                print!("{entry_color}{entry_name}{padding:<5}{reset}", padding = "");
            }

            println!();
            process::exit(0); // hacky solution
        } else {
            eprintln!("Error parsing option. For help, try running 'minils --help'");
            process::exit(1);
        }
    }
    return fs::read_dir(".").unwrap_or_else(err_handling);
}

fn print_entry(
    entry: &fs::DirEntry,
    entry_type: &fs::FileType,
    entry_name: &String,
    display_options: &DisplayOptions,
) {
    let entry_color: &str;
    let reset = "\x1b[0m";

    if entry_type.is_dir() {
        entry_color = "\x1b[1;34m"; // bold blue
    } else if entry_type.is_symlink() {
        entry_color = "\x1b[1;96m"; // bold cyan
    } else {
        entry_color = "\x1b[1m";
    }

    if display_options.long {
        let metadata = entry.metadata().unwrap_or_else(err_handling);
        let permissions_mode = metadata.permissions().mode();
        let read_color = "\x1b[33m"; // yellow
        let write_color = "\x1b[31m"; // red
        let execute_color = "\x1b[32m"; // green
        let is_user = "\x1b[1m"; // bold

        if entry_type.is_dir() {
            print!("{entry_color}d{reset}");
        } else if entry_type.is_file() {
            print!("{entry_color}-{reset}")
        } else {
            print!("{entry_color}l{reset}")
        }

        // User permissions
        if permissions_mode & 0b100_000_000 == 0b100_000_000 {
            print!("{is_user}{read_color}r{reset}");
        } else {
            print!("{is_user}{read_color}-{reset}");
        }
        if permissions_mode & 0b010_000_000 == 0b010_000_000 {
            print!("{is_user}{write_color}w{reset}");
        } else {
            print!("{is_user}{write_color}-{reset}");
        }
        if permissions_mode & 0b001_000_000 == 0b001_000_000 {
            print!("{is_user}{execute_color}x{reset}");
        } else {
            print!("{is_user}{execute_color}-{reset}");
        }

        // Group permissions
        if permissions_mode & 0b000_100_000 == 0b000_100_000 {
            print!("{read_color}r{reset}");
        } else {
            print!("{read_color}-{reset}");
        }
        if permissions_mode & 0b000_010_000 == 0b000_010_000 {
            print!("{write_color}w{reset}");
        } else {
            print!("{write_color}-{reset}");
        }
        if permissions_mode & 0b000_001_000 == 0b000_001_000 {
            print!("{execute_color}x{reset}");
        } else {
            print!("{execute_color}-{reset}");
        }

        // Other permissions
        if permissions_mode & 0b000_000_100 == 0b000_000_100 {
            print!("{read_color}r{reset}");
        } else {
            print!("{read_color}-{reset}");
        }
        if permissions_mode & 0b000_000_010 == 0b000_000_010 {
            print!("{write_color}w{reset}");
        } else {
            print!("{write_color}-{reset}");
        }
        if permissions_mode & 0b000_000_001 == 0b000_000_001 {
            print!("{execute_color}x{reset}");
        } else {
            print!("{execute_color}-{reset}");
        }

        print!("{padding:<2}", padding = "");

        let size = metadata.size();

        if !metadata.is_dir() {
            if size < 10u64.pow(3) {
                print!("{size:>4}B");
            } else if size < 10u64.pow(6) {
                print!("{size:>3}KB", size = size / 10u64.pow(6));
            } else if size < 10u64.pow(9) {
                print!("{size:>3}MB", size = size / 10u64.pow(9));
            } else if size < 10u64.pow(12) {
                print!("{size:>3}GB", size = size / 10u64.pow(12));
            } else {
                print!("{size:>3}TB", size = size / 10u64.pow(15));
            }
        } else {
            print!("{:>5}", "-");
        }
        print!("{padding:<2}", padding = "");
    }

    if display_options.long && entry_type.is_symlink() {
        let real_path = fs::read_link(entry.path()).unwrap_or_else(err_handling);
        print!(
            "{entry_color}{entry_name}{reset} -> {real_path_color}{real_path}{reset}",
            real_path = real_path.display(),
            real_path_color = "\x1b[0;31m", // regular red
        );
    } else {
        print!("{entry_color}{entry_name}{padding:<5}{reset}", padding = "");
    }

    if display_options.long || display_options.oneline {
        println!();
    }
}

pub fn run(
    entries: fs::ReadDir,
    display_options: DisplayOptions,
    filtering_options: FilteringOptions,
) {
    if display_options.long {
        println!("\x1b[4mPermissions\x1b[0m  \x1b[4mSize\x1b[0m  \x1b[4mName\x1b[0m");
    }

    for entry in entries {
        let entry = entry.unwrap_or_else(err_handling);
        let entry_type = entry.file_type().unwrap_or_else(err_handling);

        let entry_name = match entry.file_name().into_string() {
            Ok(entry_name) => entry_name,
            Err(os_entry_name) => {
                eprintln!(
                    "Invalid Unicode in name {}",
                    os_entry_name.to_string_lossy()
                );
                process::exit(1);
            }
        };

        if filtering_options.only_dirs && entry_type.is_dir() {
            if filtering_options.all {
                print_entry(&entry, &entry_type, &entry_name, &display_options);
            } else if !entry_name.starts_with(".") {
                print_entry(&entry, &entry_type, &entry_name, &display_options);
            }
        } else if filtering_options.only_files && entry_type.is_file() {
            if filtering_options.all {
                print_entry(&entry, &entry_type, &entry_name, &display_options);
            } else if !entry_name.starts_with(".") {
                print_entry(&entry, &entry_type, &entry_name, &display_options);
            }
        } else if !filtering_options.all && !entry_name.starts_with(".") {
            print_entry(&entry, &entry_type, &entry_name, &display_options);
        } else {
            print_entry(&entry, &entry_type, &entry_name, &display_options);
        }
    }

    if display_options.grid {
        println!();
    }
}
