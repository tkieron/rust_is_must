
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_assignments)]
#[allow(path_statements)]
#[allow(unused_mut)]
fn main() {
    let squere = |a : i32| -> i32 { a * a };
    apply(squere, 10);

    // Calculate the sum of all the squares less then 500
    // only for even numbers
    let mut sum = 0;
    for i in 0.. {
        let res  = squere(i);
        if res > 500 {
            break;
        }
        if !is_even(res) {
            continue;
        }
        sum += res;
    }
    println!("Sum is {0}", sum);

    // Calculate the sum of all the squares less then 500
    // only for even numbers
    // using Higher Order Functions
    let sum2 = (0..)
        .map(|x| squere(x))
        .take_while(|&x| x < 500)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("Sum2 is {0}", sum2);

    println!("factorial result of {0} is {1}", 5, factorial(5));
    let sum3 = (0..)
        .map(|x| factorial(x))
        .take_while(|&x| x < 500)
        .filter(|x| is_even(*x))
        .fold(0, |sum,x| sum + x);
    println!("Sum3 is {0}", sum3);

}

// calculate factorial of given number
fn factorial(n: i32) -> i32 {
    (1..n+1).fold(1, |a, b| a * b)
}

fn is_even(res: i32) -> bool {
    res % 2 == 0
}

fn apply(f: fn(i32) -> i32, a: i32) {
    println!("Result is {0}", f(a));
}







