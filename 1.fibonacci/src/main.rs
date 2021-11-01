fn main() {
    for i in 1..20 {
        println!("{}", fibonacci(i))
    }
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
