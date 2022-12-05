pub fn case1() {
    // 使用尽可能多的方法来通过编译
    let x = String::from("hello, world");
    let y = &x;
    println!("{},{}", x, y);
}

// 不要修改 case2 中的代码
pub fn case2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// 只能修改下面的代码!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        case1();
        case2();
    }
}