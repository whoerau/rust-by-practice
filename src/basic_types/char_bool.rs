pub fn case1() {
    use std::mem::size_of_val;
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4);

    println!("Success!")
}

pub fn case2() {
    fn print_char(c : char) {
        println!("{}", c);
    }
    let c1 = '中';
    print_char(c1);
}

pub fn case3() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("Success!")
    }
}

pub fn case4() {
    let f = true;
    let t = true && false;
    assert_ne!(t, f);

    println!("Success!")
}

pub fn case5() {
    fn implicitly_ret_unit() {
        println!("I will return a ()")
    }

    // 不要使用下面的函数，它只用于演示！
    fn explicitly_ret_unit() -> () {
        println!("I will return a ()")
    }

    let _v: () = ();

    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!")
}


pub fn case6() {
    use std::mem::size_of_val;
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!")
}



fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        case1();
        case2();
        case3();
        case4();
        case5();
        case6();
    }
}