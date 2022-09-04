pub fn case() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 下面表达式的值将被赋给 `y`
        x_cube + x_squared + x
    };

    let z = {
        // 分号让表达式变成了语句，因此返回的不再是表达式 `2 * x` 的值，而是语句的值 `()`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

pub fn case1() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);
}

pub fn case2() {
    let v = {let x = 3; x};

    assert!(v == 3);
}

pub fn case3() {
    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
    let s = sum(1 , 2);
    assert_eq!(s, 3);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        case();
        case1();
        case2();
        case3();
    }
}