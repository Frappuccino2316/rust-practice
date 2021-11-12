fn main() {
    let x = 5;
    let y = {
        let x = x-2;
        x + 1
    };

    another_function(y);
}

fn another_function(a: i32) {
    println!("The a value is: {}", a);
}