fn main() {
    let mut s = String::from("jfeirsfeijrs");
    let s2 = change(&mut s);
}

fn change(s3: &mut String) {
    s3.push_str(", www");
}
