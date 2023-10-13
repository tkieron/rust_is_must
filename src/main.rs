#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_assignments)]
#[allow(path_statements)]
#[allow(unused_mut)]
#[allow(unused_macros)]

macro_rules! do_something {
    () => {
        println!("Hello, world macro!");
    };
}

macro_rules! name {
    ($name:expr) => {
        println!("Hello, {}", $name)
    };
}

macro_rules! great {
    ($($name:expr),*) => {
        $(println!("Hi, {0}", $name);)*
    }
}


macro_rules! multi {
    (x => $e: expr) => (println!("X is {}", $e));
    (y => $e: expr) => (println!("Y is {}", $e));
}

macro_rules! build_fn {
    ($fn_name : ident) => {
        fn $fn_name() {
            println!("{:?} was called", stringify!($fn_name));
        }
    }
}

fn main() {

    do_something!();
    name!("Rust");
    great!("Rust", "Python", "C++");
    multi!(y => 1);
    multi!(x => 2 *2);
    build_fn!(hey);
    hey();
}









