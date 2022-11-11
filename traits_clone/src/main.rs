#[derive(Debug, Clone, Copy)]
struct Unit;

#[derive(Debug, Clone)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    let unit = Unit;
    let copied_unt = unit;

    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unt);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    let move_pair = pair;
    println!("moved: {:?}", move_pair);

    // println!("original: {:?}", pair);

    let cloned_pair = move_pair.clone();
    drop(move_pair);

    // println!("copy: {:?}", move_pair);

    println!("clone: {:?}", cloned_pair);
}
