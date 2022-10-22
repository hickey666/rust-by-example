use std::vec;

fn main() {
    // iter 在每次迭代中借用集合中的一个元素，这样集合本身不会被改变，循环之后仍可以使用
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);

    // into_iter 会消耗集合。在每次迭代中，集合中的数据本身会被提供。
    // 一旦集合被消耗了，之后就无法再使用了，因为它已经在循环中被“移除”（move）了。
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("names: {:?}", names);

    // iter_mut 可变的（mutably）借用集合中的每个元素，从而允许集合就地被修改。
    let mut names_mut = vec!["Bob", "Frank", "Ferris"];
    for name in names_mut.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names_mut);
}
