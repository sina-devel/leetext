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

fn main() {
    let res = decode(&r"|-|3|_|_0 |/\|0|2|_|)".to_string());
    println!("{}", res.unwrap());
}

#[test]
fn decode_test() {
    let input: String = LEET_ALPHABET.iter().map(|c| c.to_string()).collect();
    let output: String = ALPHABET.iter().map(|c| c.to_string()).collect();
    assert_eq!(decode(&input).unwrap(), output);
}
