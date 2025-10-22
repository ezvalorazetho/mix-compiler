use colored::*;
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
#[allow(unused_imports)]
use std::io::{self, Write};

mod compiler;
use compiler::lexer::Lexer;
use compiler::scanner::Scanner;

static DEBUG_VERBOSE: bool = false;

fn print_help() {
    println!("{}", "┌──────────────────────────────────────────┐".blue());
    println!("{}{}{}", "│".blue(),"            MIX COMMAND HELPER            ".green(),"│".blue());
    println!("{}", "└──────────────────────────────────────────┘".blue());

    println!("{}{}{}", " ◈".blue(), " help       ".green(), ": show command helper".yellow());
    println!("{}{}{}", " ◈".blue(), " create     ".green(), ": create new project".yellow());
    println!("{}{}{}", " ◈".blue(), " run        ".green(), ": run project / program".yellow());
    println!("{}{}{}", " ◈".blue(), " build      ".green(), ": build project / program".yellow());
    println!("{}{}{}", " ◈".blue(), " install    ".green(), ": install dependencies".yellow());
    println!("{}{}{}", " ◈".blue(), " update     ".green(), ": update compiler & package".yellow());
    println!("{}{}{}", " ◈".blue(), " clean      ".green(), ": clean project build".yellow());

    println!("{}", "┌──────────────────────────────────────────┐".blue());
    println!("{}{}{}", "│".blue(), "       READ ALL DOCUMENTATION ON WEB      ".green(), "│".blue());
    println!("{}{}{}", "│".blue(), "      https://mix.org/docs/index.html     ".green(), "│".blue());
    println!("{}", "└──────────────────────────────────────────┘".blue());
}

fn create_project(name: String) -> Option<String> {
    let binding = &name;
    let project_path = Path::new(&binding);
    if project_path.is_dir() && project_path.exists() {
        return Some(format!("{} {}", "error:".red(), "directory is exists"));
    }

    fs::create_dir_all(project_path).ok()?;

    let src_path = project_path.join("src");
    let mix_config = project_path.join("mix.conf");
    let main_path = src_path.clone().join("main.mx");

    let main_content = r#"
func main() {
    std::println("Hello, Mix!");
}
    "#;

    fs::create_dir_all(src_path).ok()?;
    let mut file_config = File::create(mix_config).ok()?;
    writeln!(file_config, "{{").ok()?;
    writeln!(file_config, "\t{}", format!("\"name\": \"{}\",", name)).ok()?;
    writeln!(file_config, "\t\"version\": \"1.0.0\",").ok()?;
    writeln!(file_config, "\t\"author\": \"...\",").ok()?;
    // writeln!(file_config, "\"target\": \"x86_64-gnu-linux\",").ok()?;
    writeln!(file_config, "").ok()?;
    writeln!(file_config, "\t\"packages\": []").ok()?;
    writeln!(file_config, "}}").ok()?;

    let mut file_main = File::create(main_path).ok()?;
    writeln!(file_main, "{}", main_content).ok()?;
    
    None
}

fn compile_program(dir: &str) -> Option<String> {
    let path = Path::new(dir);

    if !path.exists() || !path.is_dir() {
        return Some(format!("{} {}", "error:".red(), "project directory not found"));
    }

    let main_path = path.join("src/main.mx");
    let config_path = path.join("mix.conf");

    if !config_path.exists() || config_path.is_dir() { 
        return Some(format!("{} {}", "error:".red(), "`main.mx` file not found"));
    }

    if !main_path.exists() || main_path.is_dir() { 
        return Some(format!("{} {}", "error:".red(), "`main.mx` file not found"));
    }

    let _config = fs::read_to_string(&config_path).unwrap();
    let content = fs::read_to_string(&main_path).unwrap();

    let full_path = main_path.canonicalize().ok()?;
    let lexer = Lexer::new(content, full_path.display().to_string());
    
    let mut scanner = Scanner::new(lexer, DEBUG_VERBOSE);
    let nodes = scanner.scan();

    println!("{:?}", nodes);

    println!("COMPILE DONE");
    
    None
}

#[allow(dead_code)]
fn execute_program() {

}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        print_help();
        return;
    }

    if args.len() >= 2 {
        if args[1] == "help" {
            print_help();
        } else if args[1] == "run" {
            println!("under development");
            todo!();
        } else if args[1] == "create" && args.len() == 3 {
            let p = create_project(args[2].clone());
            
            if let Some(e) = p {
                println!("{} {}", "error:".red(), "failed to create new project");
                println!("{}", e);
            }

        } else if args[1] == "build" && args.len() >= 2 {
            
            if args.len() == 2 {
                compile_program("./");
            } else {
                compile_program(&args[2]);
            }

        } else if args[1] == "install" && args.len() == 3 {
            println!("under development");
            todo!();
        } else if args[1] == "update" {
            println!("under development");
            todo!();
        } else if args[1] == "clean" {
            println!("under development");
            todo!();
        } else {
            println!("{} {}", "error:".red(), "command failed");
            println!("  {}", "all command available in `help`");
            println!("  {}", "or learn more in https://mix.org/docs/index.html");
            println!("{} {}", "example:".cyan(), "mix help");
        }
    }
}
