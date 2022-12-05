pub fn case1() {
    let x = 5;
    // 填写空白处
    let p = &x;

    println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
}

pub fn case2() {
    let x = 5;
    let y = &x;

    // 只能修改以下行
    assert_eq!(5, *y);
}

pub fn case3() {
    let mut s = String::from("hello, ");

    borrow_object(&s)
}

fn borrow_object(s: &String) {}

pub fn case4() {
    let mut s = &mut String::from("hello, ");

    push_str(s)
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

pub fn case5() {
    let mut s = String::from("hello, ");

    // 填写空白处，让代码工作
    let p = &mut s;

    p.push_str("world");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        case1();
        case2();
        case3();
        case4();
        case5();
    }
}
