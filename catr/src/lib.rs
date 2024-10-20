use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonbrank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(reader) => {
                let mut prev_num = 0;
                for (line_num, line) in reader.lines().enumerate() {
                    // line をそのまま扱ってはいけない。所有権絡み。
                    let contents = line?;
                    // for loop の line をシャドーイングするほうが Rust らしさはあるらしい
                    // Rust におけるシャドーイングは、変数の名前を再利用しつつ新しい値を設定する
                    // let line = line?;
                    if config.number_lines {
                        println!("{:>6}\t{}", line_num + 1, contents);
                    } else if config.number_nonbrank_lines {
                        if contents.is_empty() {
                            println!("{}", contents);
                        } else {
                            // 空行の行数をスキップさせないようにするために必要
                            prev_num += 1;
                            println!("{prev_num:6}\t{}", contents);
                        }
                    } else {
                        println!("{}", contents);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("datahaikuninja")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input files")
                .required(false)
                .default_value("-")
                .multiple(true),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("print number line")
                .conflicts_with("number_nonbrank_lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonbrank")
                .short("b")
                .long("number-nonblank")
                .help("print number non brank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonbrank_lines: matches.is_present("number_nonbrank"),
    })
}
