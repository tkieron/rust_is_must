
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_assignments)]
#[allow(path_statements)]
#[allow(unused_mut)]
fn main() {
    println!("Hello, world!");
    for i in 0..10 {
        println!("{i}. I have {oranges}", i=i, oranges=get_oranges(i));
    }

    // defined a tupple of two i32
    let point: (i32, i32) = (1, 0);
    // match the tupple
    match point {
        (0,0) => println!("origin"),
        (0,y) => println!("x axis, y = {y}"),
        (x,0) => println!("y axis, x = {x}"),
        (x,y) => println!("({x}, {y})")
    }

}

fn get_oranges(ammount: i32) -> &'static str {
    return match ammount {
        // match exact value
        0 => "no oranges",
        // match to on of the values
        1|2 => "one or two oranges",
        // match to a range
        3..=7 => "a few oranges",
        // default case - confitional
        _ if (ammount % 2 == 0) => "even ammount of oranges",
        // default case - all other not matched previously
        _ => "a lot of oranges"
    }
}




