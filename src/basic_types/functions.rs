use std::time::Duration;

pub fn case1() {
    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }

    // 不要修改下面两行代码!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);
}

pub fn case2() {
    // 使用另一个类型来替代 i32
    fn print() -> () {
        println!("hello,world");
    }
    print();
}

pub fn case3() {
    use ::std::thread;
    use ::std::time;

    fn never_return() -> ! {
        // 实现这个函数，不要修改函数签名!
        loop {
            thread::sleep(time::Duration::from_secs(10));
        }
    }
    never_return();
}

/// 🌟🌟 发散函数( Diverging function )不会返回任何值，因此它们可以用于替代需要返回任何值的地方
pub fn case4() {
    fn get_option(tp: u8) -> Option<i32> {
        match tp {
            1 => {
                // TODO
            }
            _ => {
                // TODO
            }
        };

        // 这里与其返回一个 None，不如使用发散函数替代
        never_return_fn()
    }

    // 使用三种方法实现以下发散函数
    fn never_return_fn() -> ! {
        unimplemented!();
        // panic!();
        // todo!();
    }
    println!("Success!");
}

pub fn case5() {
    // 填空
    let b = true;

    let _v = match b {
        true => 1,
        // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };

    println!("Exercise Failed if printing out this line!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_cases() {
        case1();
        case2();
        // case3();
        case4();
        case5();
    }
}