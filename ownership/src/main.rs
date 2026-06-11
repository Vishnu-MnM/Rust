fn main() {
    ownership();
    testing();
}

fn ownership() {
    let s = "hello";
    println!("{}, this is a test", s);
}

fn testing() {
    let mut s = String::from("hello");
    s.push_str(", world"); // appends a literal to a String
    println!("{}", s);
}
