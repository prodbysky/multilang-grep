fn main() {
    let args = std::env::args();
    let config = match Config::new(args) {
        Some(c) => c,
        None => {
            eprintln!("Failed to create config from args!");
            return;
        }
    };

    let file_content = match std::fs::read_to_string(config.file_name) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    for line in file_content.lines() {
        if line.contains(&config.needle) {
            println!("{line}");
        }
    }
}

struct Config {
    file_name: String,
    needle: String,
}

impl Config {
    fn new(mut iter: impl Iterator<Item = String>) -> Option<Self> {
        let program_name = iter.next().unwrap();
        Some(Self {
            file_name: match iter.next() {
                Some(name) => name,
                None => {
                    usage(&program_name);
                    return None;
                }
            },
            needle: match iter.next() {
                Some(needle) => needle,
                None => {
                    usage(&program_name);
                    return None;
                }
            },
        })
    }
}

fn usage(program_name: &str) {
    eprintln!("./{program_name} <file-to-grep> <string-to-search>");
}
