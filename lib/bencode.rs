use std::str::Chars;
use std::string;

fn bdecode_string(it: &mut Chars, char1: char) {
    let mut str_len = "".to_string();
    let mut string = "".to_string();

    str_len.push(char1);

    for car in it.take_while(|a| a.is_digit(10)) {
        str_len.push(car);
    }

    let len = match str_len.parse::<usize>().ok() {
        Some(l) => l,
        None => panic!("Could not find string length...")
    };

    println!("String length {}", len);

    for c in it.take(len) {
        match c {
            ':' => continue,
            c => string.push(c)
        }
    }

    println!("Decoded string: {}", string);
}

fn parse_dict(it: &mut Chars) {
    loop {
        match it.next() {
            Some('e') => {
                break;
            },
            Some(c) => {
                bdecode_string(it, c);
                bdecode_inner(it);
            },
            None => panic!("Reached end of string!")
        };
    }
}

fn bdecode_inner(it: &mut Chars) {
    match it.next() {
        Some('d') => parse_dict(it),
        // 'l' => parse_list(&mut it),
        // 'i' => parse_int(&mut it),
        Some(len) => bdecode_string(it, len),
        None => ()
    };
}

fn bdecode(bencode: &str) {
    let mut it = bencode.chars();
    bdecode_inner(&mut it);
}

fn main() {
    let string = "4:abce".to_string();
    bdecode(&string);

    let string2 = "d5:abcea1:ae";
    bdecode(&string2);
}
