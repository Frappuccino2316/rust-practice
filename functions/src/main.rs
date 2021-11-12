fn main() {
    let x = plus_one(123);

    println!("x: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}