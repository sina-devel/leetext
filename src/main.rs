use std::collections::HashMap;
use std::env;
use std::io;
use std::io::Read;
use std::process::exit;

const LEET_ALPHABET: [&str; 26] = [
    "4", "8", "(", "|)", "3", "|=", "6", "|-|", "1", "_|", "|<", "|_", r"|\/|", r"|\|", "0", "|°",
    "9", "|2", "5", "7", r"\_/", r"\/", r"|/\|", ")(", "/", "2",
];
const ALPHABET: [&str; 26] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z",
];

fn decode(input: &String) -> Result<String, String> {
    let mut res = String::new();
    let mut temp = String::new();
    for i in input.chars() {
        if i == ' ' || i == '\n' || i == '\t' {
            res.push(i);
            continue;
        }
        temp.push(i);
        if LEET_ALPHABET
            .iter()
            .map(|c| c.to_string())
            .collect::<String>()
            .matches(|x| x == i)
            .count()
            == 0
        {
            return Err(format!("{} is invalid", i));
        }
        let x = LEET_ALPHABET.iter().position(|&x| x == temp);
        if x.is_some() {
            res.push_str(ALPHABET[x.unwrap()]);
            temp.clear();
        } else {
            continue;
        }
    }
    Ok(res)
}

fn help() {
    println!(
        "
USAGE:
    leetext word
    leetext ARGS

ARGS:
    -h   print help
    -p   print ALPHABET
    -    read at stdin

EXAMPLES:
    leetext hi
    echo hi | leetext -

    "
    );
}

fn pipe() {
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap_or_else(|e| {
        eprintln!("error: {}", e);
        exit(1);
    });
    match decode(&data) {
        Ok(res) => println!("{}", res),
        Err(err) => eprintln!("error: {}", err),
    }
}

fn print_alphabet() {
    let alphabet_to_leet: HashMap<&&str, &&str> =
        ALPHABET.iter().zip(LEET_ALPHABET.iter()).collect();
    println!("{:#?}", alphabet_to_leet);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        help();
        exit(1);
    }
    match args[0].as_str() {
        "-" => pipe(),
        "-p" => print_alphabet(),
        "-h" => help(),
        _ => match decode(&args[0]) {
            Ok(res) => println!("{}", res),
            Err(err) => eprintln!("error: {}", err),
        },
    }
}

#[test]
fn decode_test() {
    let input: String = LEET_ALPHABET.iter().map(|c| c.to_string()).collect();
    let output: String = ALPHABET.iter().map(|c| c.to_string()).collect();
    assert_eq!(decode(&input).unwrap(), output);
}
