extern crate bencode;

fn main() {
    let string = "l13:Hello, World!1:a1:bd3:key5:value2:p1i5e2:p2i3eee";
    let res = bencode::decode::parse(&string);
    println!("{:?}", res);
}
