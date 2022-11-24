/// 第一行是对函数的简短描述
///
/// 接下来数行是详细文档。代码块用三个反引号开启，Rust会隐式地在其中添加
/// `fn main()` 和 `extern crate <createname>`。比如测试 `doccoments` crate:
///
/// ```
/// let result = doccoments::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 文档注释通常可能带有 "Examples" 、 "Panics" 和 "Failure" 这些部分。
///
/// 下面的函数将两数相除。
///
/// # Examples
///
/// ```
/// let result = doccoments::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// 如果第二个参数是 0，函数将会panic。
///
/// ```rust,should_panic
/// // panics on divsion by zero
/// doccoments::div(10,0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("div-by-zero error");
    }

    a / b
}
fn main() {
    println!("Hello, world!");
}
