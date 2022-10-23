fn main() {
    let mut optinal = Some(0);

    while let Some(i) = optinal {
        if i > 9 {
            println!("Greater than 9, quit!");
            optinal = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optinal = Some(i + 1);
        }
    }
}
