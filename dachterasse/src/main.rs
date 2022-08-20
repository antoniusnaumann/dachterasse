use std::{env, process};
use std::error::Error;

type Execute = fn(&[String]) -> Result<(), Box<dyn Error>>;

pub struct Command {
    name: String,
    help: String,
    arguments: Vec<(String, String)>,
    method: Execute
}

impl Command {
    fn new(name: &str, help: &str, method: Execute) -> Self {
        Command {
            name: String::from(name),
            help: String::from(help),
            arguments: Vec::new(),
            method }
    }

    fn new_with_args(name: &str, help: &str, arguments: &[(&str, &str)], method: Execute) -> Self {
        Command {
            name: String::from(name),
            help: String::from(help),
            arguments: arguments.iter().map(|(arg, desc)| (String::from(*arg), String::from(*desc))).collect(),
            method }
    }

    fn execute(&self, args: &[String]) -> Result<(), Box<dyn Error>> {
        (self.method)(args)
    }
}

fn commands() -> Vec<Command> {
    let config_arg = ("--config", "Custom path to config. Default is cache/lecture_cache.json");

    vec![
        Command::new("help", "Prints out this help page",print_help),
        Command::new_with_args("overview", "Displays titles for all cached lectures. Please call dachterasse init before.", &[config_arg],show_overview),
        Command::new_with_args("all", "Shows details for all cached lectures. Please call dachterasse init before.", &[config_arg],show_details),
        // TODO: Add command for filtering by name, module, category
    ]
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmds = commands();

    if let Some(argument) = args.get(1) {
        if let Some(cmd) = cmds.iter().find(|cmd| cmd.name.eq_ignore_ascii_case(argument)) {
            cmd.execute(&args[1..]).unwrap_or_else(|error| {
                println!("An error occured while executing 'dachterasse {}': {}", cmd.name, error);
                process::exit(1);
            });
            return;
        }
    }

    let _ = print_help(&args[1..]);
}

use commands::*;
mod commands {
    use crate::*;

    pub fn print_help(_: &[String]) -> Result<(), Box<dyn Error>> {
        for cmd in commands() {
            print_command_header(&cmd);
            print_command_args(&cmd);
        }

        Ok(())
    }

    pub fn show_overview(args: &[String]) -> Result<(), Box<dyn Error>> {
        let degree = prompt_degree();
        print_lectures(client_with_config_args(args).lectures(degree));

        Ok(())
    }

    pub fn show_details(args: &[String]) -> Result<(), Box<dyn Error>> {
        let degree = prompt_degree();
        print_lectures_detailed(client_with_config_args(args).lectures(degree));

        Ok(())
    }
}

use helpers::*;
mod helpers {
    use std::io;
    use std::ops::{Add, AddAssign};
    use dachterasse::{Config, StaticDegree, Degrees, Lecture, LectureClient};
    use crate::*;

    pub fn client_with_config_args(args: &[String]) -> LectureClient {
        if args.len() >= 3 && args[1] == "--config" {
            LectureClient::from_config(Config::new().cache_path(args[1].clone())).initialized()
        } else {
            LectureClient::from_config(Config::with_cache()).initialized()
        }
    }

    pub fn print_lectures(lectures: &[Lecture]) {
        if lectures.is_empty() {
            println!("\nNo lectures found. Try to run 'dachterasse init'.")
        }

        for lecture in lectures {
            println!("{}", lecture.title);
        }
    }

    pub fn print_lectures_detailed(lectures: &[Lecture]) {
        if lectures.is_empty() {
            println!("\nNo lectures found. Try to run 'dachterasse init'.")
        }

        for lecture in lectures {
            println!("{}", lecture.title);
            println!("{}", lecture.url);
            if let Some(c) = &lecture.categories {
                for (module, categories) in c {
                    println!("> {}", module);
                    for category in categories {
                        println!("\t {}", category);
                    }
                }
            }
            println!("--------------");
        }
    }

    pub fn prompt_degree() -> &'static StaticDegree {
        println!("Degrees\n");

        let degrees = Degrees::all();
        for (index, degree) in degrees.iter().enumerate() {
            println!("{} ({})", degree.name, index);
        }

        println!("\nPlease choose your degree (0 - {}):", degrees.len() - 1);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        let input: usize = input.trim()
            .parse()
            .expect("Please enter a valid index");

        &degrees[input]
    }

    const NAME_LENGTH: usize = 12;

    pub fn print_command_header(command: &Command) {
        let mut name = command.name.to_owned().add("                    ");
        name.truncate(NAME_LENGTH);
        println!("{} {}", name, command.help)
    }

    pub fn print_command_args(command: &Command) {
        let mut spacing = String::from(" ");
        for _ in 0..NAME_LENGTH {
            spacing.add_assign(" ");
        }

        if let Some(arg) = command.arguments.first() {
            print!("{}ARGS:", spacing);
            println!(" {}\t{}", arg.0, arg.1);
        }
        if command.arguments.len() > 1 {
            for arg in &command.arguments[1..] {
                println!("      {}{}\t{}", spacing, arg.0, arg.1);
            }
        }
        println!();
    }
}