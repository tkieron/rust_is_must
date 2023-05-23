/*
    Constants live for the entire lifetime of a program. More specifically,
    constants in Rust have no fixed address in memory.
    This is because they’re effectively inlined to each place that they’re used.
    References to the same constant are not necessarily guaranteed to refer to the same memory address for this reason.
 */
const MAX_POINTS: u32 = 100_000;

/*
Rust provides a ‘global variable’ sort of facility in static items.
They’re similar to constants, but static items aren’t inlined upon use.
This means that there is only one instance for each value, and it’s at a fixed location in memory.

Statics live for the entire lifetime of a program, and therefore any reference stored in a static has a 'static lifetime:

static NAME: &'static str = "Steve";

 */
static PI: f32 = 3.14;

/*
We can define a static mut variable, but we must do so within an unsafe block
 */
static mut COUNTER: u32 = 0;

/*
Almost always, if you can choose between the two, choose const.
It’s pretty rare that you actually want a memory location associated with your constant,
and using a const allows for optimizations like constant propagation not only in your crate but downstream crates.
 */


/*
 Variaables are immutable by default
 in order to declare a mutable variable a mut keyword must be used
 but varialbles can be shadowed - resued with the same name in the same scope
 */
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_assignments)]
#[allow(path_statements)]
#[allow(unused_mut)]
fn main() {
    // declared c1 as immutable integer size 8bits
    let c1: i8 = 1;
    // declared c2 as mutable integer size 16bits
    let c2: i16 = 2;
    // declared c3 as immutable integer size 128bits - max size of integer in rust
    let c3: i128 = 3;
    // declared c4 as immutable unsigned integer size 8bits
    let u1: u8 = 1;
    // by default type of a variable can be infered by compiler
    let inf = 10;
    // but it can be also declared explicitly
    let inf_2 = 34.20;
    // to improve readabilty number can be written with underscore separator
    let readable_number = 1_000_000; // one milion

    // example of boolean variable
    let is_real = true;
    // string can be represented as a slice of bytes
    let name = "John";
    // or as a String object
    let last_name = String::from("Doe");
    // string can be concatenated with + operator
    let full_name = name.to_owned() + " " + &last_name;
    // or with format! macro
    let full_name_2 = format!("{} {}", name, last_name);
    // or with push_str method
    let mut full_name_3 = String::from("John");
    full_name_3.push_str(" Doe");
    // or with push method - it takes only one character
    let mut full_name_4 = String::from("John");
    full_name_4.push(' ');

    // tuple is a collection of values of different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tuple can be destructured
    let (x, y, z) = tup;
    // or accessed by index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // array is a collection of values of the same type
    let a = [1, 2, 3, 4, 5];
    // array can be destructured
    let [a1, b2, c3, d4, e5] = a;
    // or accessed by index
    let one = a[0];
    let two = a[1];
    let three = a[2];
    let four = a[3];
    let five = a[4];

    // function can be called with or without parentheses
    hello_no_args();
    hello_no_args;
    // function can be called with or without arguments
    hello("John");
    hello;
    // function can be called with or without return value
    let result = hello("John");
    let result = hello;

    // mutable variable can be reassigned
    let mut some_mutable: u8 = 1;
    some_mutable = 2;
    println!("some_mutable is {}", some_mutable);

    // shadowing - reusing the same name for variable
    let shadowed = 1;
    let shadowed = 2;
    println!("shadowed is {}", shadowed);

    // frozen - immutable variable cannot be reassigned
    let mut frozen = 1;
    {
        // variable frozen is shadowed in this scope
        let frozen = 2;
        // now frozen is immutable
        // frozen = 3; // this will not compile
        println!("frozen is {}", frozen);
    }
    println!("frozen is {}", frozen);

    // constants are immutable by default
    // constants cannot be shadowed
    // constants must be declared with type
    // constants can be declared in any scope
    // constants can be declared only with const keyword
    // constants can be declared only with uppercase letters
    // constants can be declared only with underscore separator for readability
    // constants can be declared only with explicit type
    // constants can be declared only with value assigned
    // PI = 3.15; // this will not compile
    // MAX_POINTS = 10;
    println!("PI is {}", PI);
    println!("MAX_POINTS is {}", MAX_POINTS);
    // static global variables are immutable by default
    // static global variables cannot be shadowed
    // let MAX_POINTS: int32 = 100_000_000;

    // use static mutable requires code block with unsafe
    unsafe {
        COUNTER += 1;
    }
}

fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn hello_no_args() -> String {
    format!("Hello, world!")
}



