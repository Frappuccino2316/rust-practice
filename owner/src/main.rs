fn main() {
    let mut s1 = String::from("hello");
    let s2 = s1.clone();
    s1 = String::from("world");
    println!("s1: {}, s2: {}", s1, s2);
}
