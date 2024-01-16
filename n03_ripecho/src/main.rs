// Basic echo command implementation in Rust

pub mod rip_echo;
fn main() {
    // Parse args from command line
    let (text, _options) =  parse_args().unwrap_or_else(|e| {
        println!("Error: {}", e);
        std::process::exit(1);
    });

    rip_echo::echo(text)
}

// Options struct
struct Options {
    enable_escapes: bool,
}

impl Options {
    // Create new Options struct
    fn new() -> Options {
        Options {
            enable_escapes: false,
        }
    }
}

fn parse_args () -> Result<(String, Options), &'static str> {
    // Parse args from command line
    let args: Vec<String> = std::env::args().collect();
    // Create new Options struct
    let mut options = Options::new();

    // If no args, return error
    if args.len() < 2 {
        return Err("No arguments found");
    }

    let mut text = String::new();
    // Check for options and build the Options struct
    for arg in args.iter().skip(1) {
        if arg.starts_with("-") {
            // check for options
            match arg.as_str() {
                "-e" => {
                    options.enable_escapes = true;
                },
                "-h" => {
                    println!("Usage: {} [options] [text]", args[0]);
                    println!("Options:");
                    println!("-h\t\tHelp");
                    println!("-v\t\tVersion");
                    println!("-e\t\tEnables the interpretation of backslash escapes");
                    std::process::exit(0);
                },
                "-v" => {
                    println!("{} version {}", args[0], env!("CARGO_PKG_VERSION"));
                    std::process::exit(0);
                },
                _ => {
                    println!("{}: Unknown option {}", args[0], arg);
                    std::process::exit(1);
                }
            }
        }

        // remove escapes from new line markers
        let arg = arg.replace("\\n", "\n");

        // Save the arg that does not start with "-"
        text = if !text.is_empty() {
            format!("{} {}", text, arg)
        } else {
            arg.to_string()
        };
    }
    return Ok((text, options))
}


