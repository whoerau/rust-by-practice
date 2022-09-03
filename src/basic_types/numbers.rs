pub fn case1() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;

    let _z = 10; // 这里 z 的类型是?
}

pub fn case2() {
    let v: u16 = 38_u8 as u16;
}

pub fn case3() {
    // 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }

    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));
}

pub fn case4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
}

pub fn case5() {
    let v1 = 247_u8 + 8;
    let v2 = i8::checked_add(119, 8).unwrap();
    println!("{},{}", v1, v2);
}

pub fn case6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v != 1579);
}

pub fn case7() {
    let x = 1_000.000_1; // f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64
}

pub fn case8() {
    assert!(0.1 + 0.2 != 0.3);
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);  // ???
    assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.001);
}

pub fn case9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c as u8);
    }
}

pub fn case10() {
    use std::ops::{Range, RangeInclusive};
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
}

pub fn case11() {
    // 整数加法
    assert!(1u32 + 2 == 3_u32);

    // 整数减法
    assert!(1i32 - 2 == -1i32);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150i32);

    assert!(9.6f32 / 3.2f32 == 3.0f32); // error ! 修改它让代码工作

    assert!(24 % 5 == 4 );

    // 逻辑与或非操作
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // 位操作
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        case1();
    }

    #[test]
    fn test_case2() {
        case2();
    }

    #[test]
    fn test_case3() { case3(); }

    #[test]
    fn test_case4() { case4(); }

    #[test]
    fn test_case5() { case5(); }

    #[test]
    fn test_case6() { case6(); }

    #[test]
    fn test_case7() { case7(); }

    #[test]
    fn test_case8() { case8(); }

    #[test]
    fn test_case9() { case9(); }

    #[test]
    fn test_case10() { case10(); }

    #[test]
    fn test_case11() { case11(); }
}