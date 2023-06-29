use rand::Rng;

#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_assignments)]
#[allow(path_statements)]
#[allow(unused_mut)]
fn main() {
    let mut rng = rand::thread_rng();
    let mut x = rng.gen_range(-100..100);
    if x >= 0 {
        println!("{} is positive", x);
    } else if x == 0 {
        println!("{} is zero", x);
    } else if x == 1 {
        println!("{} is one", x);
    } else {
        println!("{} is negative", x);
    }
}




