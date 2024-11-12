use clap::{Arg, Command};
use css_parser_project::*;
use std::fs;
use std::process::{exit, Command as ProcessCommand};

fn parse_file(file_path: &str) -> Result<String, String> {
    println!("Парсинг файлу: {}", file_path);

    match fs::read_to_string(file_path) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(format!("Не вдалося прочитати файл: {}", e)),
    }
}

fn parse_css_code(css_code: &str) -> Result<(), String> {
    match parse_css_file(css_code) {
        Ok(parsed) => {
            println!("Розпарсений CSS код:\n{}", parsed);
            Ok(())
        }
        Err(e) => {
            eprintln!("Помилка парсингу CSS коду: {}", e);
            Err("Помилка при парсингу CSS коду.".to_string())
        }
    }
}

fn display_credits() {
    println!("Цей CSS парсер зробила Прокопчук Олександра, студентка ІПЗ-3.");
}

fn run_tests() {
    println!("Запуск тестів...");

    let output = ProcessCommand::new("cargo")
        .arg("test")
        .arg("--test")
        .arg("css_grammar_tests")
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                eprintln!(
                    "Тести не пройшли з помилками: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                exit(1);
            } else {
                println!("Тести пройшли успішно!");
            }
        }
        Err(e) => {
            eprintln!("Помилка при запуску тестів: {}", e);
            exit(1);
        }
    }
}

fn main() {
    let matches = Command::new("CSS Parser")
        .version("1.0")
        .author("Олександра <o.prokopchuk@ukma.edu.ua>")
        .about("CLI для парсингу CSS файлів та запуску тестів")
        .subcommand(
            Command::new("parse").about("Парсити CSS файл").arg(
                Arg::new("file")
                    .help("Шлях до CSS файлу")
                    .default_value("css_code.txt")
                    .index(1),
            ),
        )
        .subcommand(Command::new("tests").about("Запустити тести"))
        .subcommand(Command::new("credits").about("Показати інформацію про авторку"))
        .get_matches();

    match matches.subcommand() {
        Some(("parse", sub_m)) => {
            if let Some(file_path) = sub_m.get_one::<String>("file") {
                let css_code = match parse_file(file_path) {
                    Ok(contents) => contents,
                    Err(e) => {
                        eprintln!("Помилка: {}", e);
                        exit(1);
                    }
                };

                if let Err(e) = parse_css_code(&css_code) {
                    eprintln!("Помилка: {}", e);
                    exit(1);
                }
            }
        }
        Some(("tests", _)) => run_tests(),
        Some(("credits", _)) => display_credits(),
        None => {
            println!("Використовуйте 'cargo run help' для відображення доступних команд.");
        }
        _ => {
            eprintln!("Невідома команда. Спробуйте 'cargo run help' для допомоги.");
            exit(1);
        }
    }
}
