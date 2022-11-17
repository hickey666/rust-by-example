fn main() {
    iter_resutl_1();
    iter_resutl_2();
    iter_resutl_3();
    iter_resutl_4();
    iter_resutl_5();
}

fn iter_resutl_1() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("iter_resutl_1 Results: {:?}", numbers);
}

// 使用filter_map()忽略失败项
fn iter_resutl_2() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("iter_resutl_2 Results: {:?}", numbers);
}

// 使用collect()使整个操作失败
fn iter_resutl_3() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("iter_resutl_3 Results: {:?}", numbers);
}

// 使用Partition()收集所有合法的值和错误
fn iter_resutl_4() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("iter_resutl_4 Number: {:?} Errors: {:?}", numbers, errors);
}

// 使用Partition()收集所有合法的值和错误
fn iter_resutl_5() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("iter_resutl_5 Number: {:?} Errors: {:?}", numbers, errors);
}
