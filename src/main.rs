use std::{env::args, fs::File, io::{stdin, stdout, Read, Write}, process::Command};

struct Debugger {
    source: Vec<u8>,
    cur: usize,
}
impl Debugger {

    pub fn new(source: Vec<u8>) -> Self {
        Self {
            source,
            cur: 0
        }
    }

    pub fn rest(&mut self) {
        self.cur = 0;
    }
}


fn input(prompt: &str) -> String {
    print!("{prompt}");
    stdout().flush().unwrap();
    let mut inp = String::new();
    stdin().read_line(&mut inp).unwrap();
    inp.trim().to_string()
}

fn parse_number(num_str: &str) -> Option<usize> {
    if num_str.starts_with("0x") {
        Some(usize::from_str_radix(num_str.trim_start_matches("0x"), 16).unwrap())
    } else if num_str.starts_with("0b") {
        Some(usize::from_str_radix(num_str.trim_start_matches("0b"), 2).unwrap())
    } else {
        if let Ok(num) = num_str.parse::<usize>() {
            return Some(num);
        } else {
            return None;
        }
    }
}


fn debug(source: Vec<u8>) {
    let mut debugger = Debugger::new(source);

    loop {
        let prompt = input(format!("${:#04X}> ",debugger.cur).as_str());
        match parse_number(&prompt) {
            Some(x) => {
                for i in debugger.cur..(debugger.cur + x as usize) {
                    print!("{:02X} ", debugger.source[i]);
                }
                println!();
                debugger.cur += x as usize;
            },
            None => {
                let prompt_list = prompt.split(" ").collect::<Vec<&str>>();
                match prompt_list[0].trim() {
                    "exit" | "quit" | "close" => {
                        break;
                    },
                    "reset" => {
                        debugger.rest();
                    },
                    "offset" => {
                        let value = parse_number(prompt_list[1]).unwrap_or(debugger.cur);
                        debugger.cur = value;
                    },
                    _ => {
                        println!("unknown command!");
                        continue;
                    }
                }
            }
        }
    }

}

fn print_cmd_help(cmd: String, desc: String) {
    print!("    ");
    let cmd_size = cmd.len();
    let spaces = 15 - cmd_size;
    print!("{cmd}{}", " ".repeat(spaces));
    println!("{desc}");
}

fn help(program_path: String) {
    println!("{program_path} <file path>");
    print_cmd_help("--help, -h".to_string(), "Shows help for the program".to_string());
}

fn extract_file_derails(file_path: String) {
    match Command::new("file").arg("-b").arg(file_path).output() {
        Ok(out) => {
            if !out.status.success() {
                println!("filetype: unknown");
                return;
            }
            let out_str = String::from_utf8_lossy(&out.stdout);
            let params = out_str.split(",");
            for param in params {
                println!("{}",param.trim());
            }
        },
        Err(_) => {
            println!("filetype: unknown");
        }
    }
}

fn show_file_details(file_path: String, size: usize) {
    println!("\n{:=^1$}", file_path, 50);
    println!("size: {}",size);
    extract_file_derails(file_path);
    println!("{}", "=".repeat(50));
}

fn start_debugging(file_path: String) {
    let mut file = File::open(&file_path).unwrap();
    let mut source = Vec::new();
    file.read_to_end(&mut source).unwrap();
    show_file_details(file_path, source.len());
    debug(source);
}

fn main() {
    let mut args = args();
    let program_path = args.next().unwrap();
    let cmd = args.next().expect("to little arguments!");
    match cmd.as_str() {
        "--help" | "-h" => help(program_path),
        _ => {
            start_debugging(cmd);
        }
    }
}
