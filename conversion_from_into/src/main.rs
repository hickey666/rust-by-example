use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    // From
    let num = Number::from(30);
    println!("The number is {:?}", num);

    // Into
    // Into trait 就是把 From trait 倒过来而已。也就是说，如果你为你的类型实现了 From，那么同时你也就免费获得了 Into。
    let int = 5;
    let num: Number = int.into();
    println!("The number is {:?}", num);
}
