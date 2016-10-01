use std::fmt::{Formatter, Result};
use std::fmt::Debug;

use std::str::Chars;
use std::collections::BTreeMap;

pub enum Bencoded {
    String(String),
    Object(self::Object),
    Array(self::Array),
    U64(u64),
    Null
}

pub type Array = Vec<Bencoded>;
pub type Object = BTreeMap<String, Bencoded>;

impl Debug for Bencoded {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Bencoded::String(ref string) => String::fmt(string, f),
            Bencoded::Object(ref obj) => BTreeMap::fmt(obj, f),
            Bencoded::U64(ref int) => Debug::fmt(int, f),
            Bencoded::Array(ref ar) => Debug::fmt(ar, f),
            _ => panic!("Noooo!")
        }
    }
}

fn bdecode_string(it: &mut Chars, char1: char) -> String {
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

    for c in it.take(len) {
        match c {
            ':' => continue,
            c => string.push(c)
        }
    }

    return string;
}

fn bdecode_dict(it: &mut Chars) -> Bencoded {
    let mut obj = BTreeMap::new();
    let mut done = false;

    while !done {
        match it.next() {
            Some('e') => {
                done = true;
            },
            Some(c) => {
                let key = bdecode_string(it, c);
                let val = bdecode_value(it);
                obj.insert(key, val);
            },
            None => panic!("Reached end of string!")
        };
    }

    return Bencoded::Object(obj);
}

fn bdecode_list(it: &mut Chars) -> Bencoded {
    let mut ar: Array = vec![bdecode_value(it)];
    let mut done = false;

    while !done {
        match it.next() {
            Some('e') => done = true,
            Some(c) => {
                ar.push(bdecode_match(c, it));
            },
            None => panic!("Reached end of string while parsing list.")
        }
    }

    return Bencoded::Array(ar);
}

fn bdecode_int(it: &mut Chars) -> Bencoded {
    let mut acc = "".to_string();
    let mut done = false;

    while !done {
        match it.next() {
            Some('e') => done = true,
            Some(c) => {
                acc.push(c);
                println!("acc: {}", acc);
            },
            None => panic!("Reached end of string while parsing int.")
        }
    }

    match acc.parse::<u64>().ok() {
        Some(i) => return Bencoded::U64(i),
        None => panic!("Not int to parse.")
    }
}

fn bdecode_match(c: char, it: &mut Chars) -> Bencoded {
    match c {
        'd' => bdecode_dict(it),
        'l' => bdecode_list(it),
        'i' => bdecode_int(it),
        c => Bencoded::String(bdecode_string(it, c)),
    }
}

fn bdecode_value(it: &mut Chars) -> Bencoded {
    match it.next() {
        Some(c) => bdecode_match(c, it),
        None => Bencoded::Null,
    }
}

pub fn bdecode(bencode: &str) -> Bencoded {
    let mut it = bencode.chars();
    return bdecode_value(&mut it);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let string = "4:abce".to_string();
        let res = bdecode(&string);
        println!("{:?}", res);

        let string2 = "d5:abcea1:ae";
        let res2 = bdecode(&string2);
        println!("{:?}", res2);

        let string3 = "d5:abcea1:a3:cde1:ee";
        let res3 = bdecode(&string3);
        println!("{:?}", res3);

        let string4 = "l1:a1:bd3:key5:valueee";
        let res4 = bdecode(&string4);
        println!("{:?}", res4);

        let string5 = "l1:a1:bd3:key5:value2:p1i5e2:p2i3eee";
        let res5 = bdecode(&string5);
        println!("{:?}", res5);
    }
}
