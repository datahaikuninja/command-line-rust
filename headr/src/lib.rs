use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{stdin, BufRead, BufReader, Read},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("datahaikuninja")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input files")
                .required(false)
                .default_value("-")
                .multiple(true),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .short("n")
                .long("lines")
                .help("print number of lines")
                .takes_value(true)
                .default_value("10") // 自分の回答では不要
                .required(false),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .conflicts_with("lines")
                .short("c")
                .long("bytes")
                .help("print size of bytes")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    /* 自分の回答
    let mut line_count: usize = 10;
    if let Some(lines) = matches.value_of("lines") {
        match parse_positive_int(lines) {
            Ok(l) => line_count = l,
            Err(e) => return Err(format!("illegal line count -- {}", e).into()),
        }
    }

    let mut bytes_count: Option<usize> = Some(0);
    if let Some(bytes) = matches.value_of("bytes") {
        match parse_positive_int(bytes) {
            Ok(b) => bytes_count = Some(b),
            Err(e) => return Err(format!("illegal byte count -- {}", e).into()),
        }
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: line_count,
        bytes: bytes_count,
    })
    */

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int) // .value_of("lines") の返り値が Some で包まれていたら、map() は &str をアンパックして parse_positive_int に渡す
        .transpose() // parse_positive_int の返り値の型は Result<T, Box<dyn Error>> なので map は Option<Result<T, Box<dyn Error>>> を返す。transpose は Option<Result<T, E>> を Result<Option<T>, E> に変換する
        .map_err(|e| format!("illegal line count -- {}", e))?; // transpose の返り値に Err が含まれていたら関数を実行する。ここではエラーメッセージを作成している。Ok なら何もせず Ok を返す。? を使い Err を main に伝播するか、Ok 値をアンパックする
    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let bytes = config.bytes;
    let lines = config.lines;
    let insert_header_and_newline = config.files.len() >= 2;
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(reader) => {
                if insert_header_and_newline {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }
                print(
                    bytes, lines, //insert_header_and_newline,
                    reader,
                    //&filename
                )?;
            }
        }
    }
    Ok(())
}
fn print(
    bytes: Option<usize>,
    lines: usize,
    //insert_header_and_newline: bool,
    mut reader: Box<dyn BufRead>,
    //filename: &str,
) -> MyResult<()> {
    match bytes {
        // 指定バイト数を標準出力に書き込む処理は、自分の実装で多くのテストに合格しているので悪くない
        Some(n) => {
            let mut buf: Vec<u8> = Vec::new();
            reader.take(n as u64).read_to_end(&mut buf)?;
            let utf8_str = String::from_utf8_lossy(&buf); //.into_owned(); このメソッドチェーンはいらなかった。

            // ヘッダーの挿入はこの関数の外でやったほうが筋がよかった
            //if insert_header_and_newline {
            //    println!("==> {} <==", filename)
            //};
            print!("{}", utf8_str);
            //if insert_header_and_newline {
            //    println!()
            //}
        }
        None => {
            // ヘッダーの挿入はこの関数の外でやったほうが筋がよかった
            //if insert_header_and_newline {
            //    println!("==> {} <==", filename)
            //};
            // 模範解答ここから
            let mut line = String::new();
            for _ in 0..lines {
                let bytes = reader.read_line(&mut line)?; // read_line() を使うと改行(0xA), CRLF(0xD, 0xA) を見つけると、そこまでのバイトと改行を  buf に追加する
                if bytes == 0 {
                    break;
                }
                print!("{}", line);
                line.clear();
            }
            // 模範解答ここまで

            /* 自分の回答ここから
            for line in reader.lines().take(lines) { // read_line() を使わないと改行(0xA), CRLF(0xD, 0xA) を含めることができないのでテストに合格できなかった
                match line {
                    Err(err) => eprintln!("{}", err),
                    Ok(s) => println!("{}", s),
                }
            }
            if insert_header_and_newline {
                println!()
            }
            自分の回答ここまで*/
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse::<usize>() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
