
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_assignments)]
#[allow(path_statements)]
#[allow(unused_mut)]
fn main() {
    println!("get squares");
    get_squares(3478);
    println!("get cubes");
    get_cubes(4938);
}

fn get_squares(limit : i32) {
    let mut x = 1;
    // this loop will execute as long limit is reached
    while x * x < limit {
        println!("{0} * {0} = {1}", x, x * x);
        x += 1;
    }
}

fn get_cubes(limit: i32) {
    let mut x = 1;
    // this loop will execute infinitely until break is called
    loop {
        println!("{0} * {0} * {0} = {1}", x, x * x * x);
        x += 1;
        if x * x * x > limit {
            break;
        }
    }
}






