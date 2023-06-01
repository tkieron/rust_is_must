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
    let mut nums = [1, 2, 3, 4, 5];
    println!("First assigment {:?}", nums);
    // change the value of first element is possible while it's declared as mutable
    nums[0] = 0;
    println!("After zero element is changed {:?}", nums);
    // assigned first element reference to mutable variable
    let mut number = &mut nums[0];
    // passing the variable as mutable to function
    by_reference(&mut number);
    println!("After function was called with mutable variable {:?}", nums);
    by_value(nums[3]);
    println!("After function was called with mutable variable {:?}", nums);

    println!("for loop");
    for i in 0..10 {
        println!("{}",i);
    }
    println!("while loop");
    let mut j =  0;
    while j < 10 {
        println!("{}", j);
        j = j + 1;
    }
    let mut k = 0;
    println!("loop ");
    loop {
        println!("{}", k);
        k = k + 1;
        if k == 10 {
            break;
        }
    }
}

/*
 Passing by reference.
 This method takes mutable i32 as a paremeter,
 prints it into console log and then sets it's value by changing memory addres to value 2
 then one more time prints the value to console log
 As the result not only value within the function has changed but also the value that was passed a function
 */
fn by_reference(number: &mut i32) {
    println!("Method by_reference. Argument is {}", number);
    // change of the value stored in-memory - we do it with pointer (*)
    *number = 2;
    println!("Method by_reference...and now Argument is {}", number);
}

/*
    Passing by value. Value cannot be changed
 */
fn by_value(number: i32) {
    println!("Method by_value. Argument is {}", number);
    // cannot reasigne the value becouse the number is immutable
    // number = 10;
    // println!("Method by_value...and now Argument is {}", number);
}



