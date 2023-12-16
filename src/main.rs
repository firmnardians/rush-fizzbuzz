fn main( )  {
    generate_fizzbuzz(10);
}


fn generate_fizzbuzz(value: i32)  {
    for i in 1..=value{

        if i % 3 == 0 && i % 5 == 0 {
            println!("Fizz Buzz")
        }else if i % 3 == 0 {
            println!("Fizz")
        }else if i % 5 == 0 {
            println!("Buzz")
        }else {
            println!("{}", i)
        }
    }
}