use std::time::SystemTime;
use std::io;

fn main() {
    part1();
    part2();
    part3();
}

fn part1() {
    println!("Recursive");

    let before = SystemTime::now();
    let k = 5;
    for i in 0..k {
        println!("Fibonacci({}) = {}", i, recursive_fib(i));
    }
    let after = SystemTime::now();
    let difference = after.duration_since(before);
    let difference = difference.expect("Did the clock go back?");
    println!("Time it took: {:?}", difference);
}
fn recursive_fib(k: u32) -> u128 {
    return if k == 0 {
        0
    } else if k == 1 {
        1
    } else {
        recursive_fib(k - 1) + recursive_fib(k - 2)
    }
}
fn part2() {
    println!("Iterative");

    let mut a: [u8; 101] = [0; 101];
    a[0] = 0;
    a[1] = 1;

    for i in 2..101 {
        a[i] = a[i - 2] + a[i - 1];
    }
    for i in 0..101 {
        println!("Fibonacci({}) = {}", i, a[i]);
    }
}

fn part3(){
    println!("Enter a non-negative integer:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    let k: u32 = input.parse().expect("Not a good number!");
    let k_u128: u128 = k as u128;

    let mut sum:u128= 0;
    for i in 1..k_u128+1{
        sum += i * i;
    }
    println!("The sum of squares from 1 to {} is: {}", k, sum)
}



