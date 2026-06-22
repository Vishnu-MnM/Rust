fn main() {
    ownership();
    testing();
    cloning();
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

fn cloning() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}