fn main( )  {
    generate_fizzbuzz(100);
}

fn is_valid(num: u32, a: u32) -> bool {
    if a == 0 {
        return false
    }

    num % a == 0
}

fn generate_fizzbuzz(value: u32) {
    for i in 1..=value{
        if is_valid(i, 3) && is_valid(i, 5) {
            println!("Fizz Buzz")
        }else if is_valid(i, 3) {
            println!("Fizz")
        }else if is_valid(i, 5) {
            println!("Buzz")
        }else {
            println!("{}", i)
        }
    }
}