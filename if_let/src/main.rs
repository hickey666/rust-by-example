enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let` 结构读作：若 `let` 将 `number` 解构成 `Some(i)` ，则执行语句块（`{}`）
    if let Some(i) = number{
        print!("Matched {:?}", i);
    }

    if let Some(i) = letter{
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letter = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if i_like_letter  {
        println!("Didn match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is footbar");
    }
    if let Foo::Bar = b {
        println!("b is footbar");
    }
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
}
