use std::{env::args, fs::File, io::{stdin, stdout, Read, Write}};

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

fn debug(source: Vec<u8>) {
    let mut debugger = Debugger::new(source);
    println!("size: {}",debugger.source.len());

    loop {
        let prompt = input(format!("$0x{:X}> ",debugger.cur).as_str());
        match prompt.parse::<u64>() {
            Ok(x) => {
                for i in debugger.cur..(debugger.cur + x as usize) {
                    print!("{:X} ", debugger.source[i]);
                }
                println!();
                debugger.cur += x as usize;
            },
            Err(_) => {
                match prompt.as_str() {
                    "exit" | "quit" | "close" => {
                        break;
                    },
                    "reset" => {
                        debugger.rest();
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

fn start_debugging(file_path: String) {
    let mut file = File::open(file_path).unwrap();
    let mut source = Vec::new();
    file.read_to_end(&mut source).unwrap();
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
