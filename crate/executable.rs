// rustc executable.rs --extern rary=library.rlib --edition=2018 && ./executable
fn main() {
    rary::public_function();

    // 报错！ `private_function` 是私有的
    // rary::private_function();

    rary::indirect_access();
}
