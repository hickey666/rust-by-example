fn main() {
    println!("Tell me what type of person you are");
    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // 将匹配的值绑定到 n 上，现在年龄就可以读取了
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm not a old person of age {:?}", n),
    }

    match some_number() {
        Some(n @ 42) => println!("The Answer: {}", n),
        Some(n) => println!("The Interesing... {}", n),
        _ => (),
    }
}

fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}
