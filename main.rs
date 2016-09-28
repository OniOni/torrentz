extern crate bencode;

fn main() {
    println!("Hello, World!");

    let string = "l1:a1:bd3:key5:value2:p1i5e2:p2i3eee";
    let res = bencode::bencode::bdecode(&string);
    println!("{:?}", res);
}
