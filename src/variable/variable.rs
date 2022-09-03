/// 🌟 变量只有在初始化后才能被使用
pub fn case1() {
    let x: i32 = 0; // 未初始化，但被使用
    let y: i32 = 0; // 未初始化，也未被使用
    println!("x,y  is equal to {} {}", x, y);
}

/// 🌟🌟 可以使用 mut 将变量标记为可变
pub fn case2() {
    let mut x = 1;
    x += 2;

    println!("x = {}", x);
}

/// 🌟 作用域是一个变量在程序中能够保持合法的范围
pub fn case3() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }
    println!("x 的值是 {}", x);
}

///
pub fn case4() {
    let x = define_x();
    fn define_x() -> String {
        let x = "hello".to_string();
        x
    }
    println!("{}, world", x);
}

/// 🌟🌟 若后面的变量声明的名称和之前的变量相同，则我们说：第一个变量被第二个同名变量遮蔽了( shadowing )
pub fn case5() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12)
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // 输出 "42".
}


/// 🌟🌟 删除一行代码以通过编译
#[allow(unused_variables)]
pub fn case6() {
    let mut x: i32 = 1;
    x = 7;
    // 遮蔽且再次绑定
    let x = x;
    // x += 3;


    let y = 4;
    // 遮蔽
    let y = "I can also be bound to text!";
}

/// 使用以下方法来修复编译器输出的 warning :
#[allow(unused_variables)]
pub fn case7() {
    let _x = 1;
}

/// 🌟🌟 我们可以将 let 跟一个模式一起使用来解构一个元组，最终将它解构为多个独立的变量
pub fn case8() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
}

/// 你可以在赋值语句的左式中使用元组、切片或结构体进行匹配赋值
pub fn case9() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // 填空，让代码工作
    assert_eq!([x, y], [3, 2]);
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
    fn test_case3() {
        case3();
    }

    #[test]
    fn test_case4() {
        case4();
    }

    #[test]
    fn test_case5() {
        case5();
    }

    #[test]
    fn test_case6() {
        case6();
    }

    #[test]
    fn test_case7() {
        case7();
    }

    #[test]
    fn test_case8() {
        case8();
    }

    #[test]
    fn test_cas9() {
        case9();
    }
}
