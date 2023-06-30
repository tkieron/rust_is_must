
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_assignments)]
#[allow(path_statements)]
#[allow(unused_mut)]
fn main() {
    // for loop of the range
    for i in 1..11 {
        // read it as element {0} and element {1} - so element 0 multiplied by element 0 = element 1
        println!("{0} * {0} = {1}", i, i * i);
    }
    // for loop of the collection elements
    let pets = ["cat", "dog", "rabbit", "hamster", "bear"];
    for pet in pets.iter() {
        // compare to the bear using the reference &"bear"
        if pet == &"bear" {
            println!("I don't like {}", pet);
            continue;
        }
        println!("I love my {}", pet);
    }

    // a for loop with enumerate to get pos and element
    for (pos, i) in (1..11).enumerate() {
        let square = i * i;
        let nb = pos;
        println!("L.p. {0} -> {1} * {1} = {2}",nb, i, square);
    }
}





