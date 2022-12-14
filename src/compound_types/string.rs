// 修复错误，不要新增代码行
// 🌟 正常情况下我们无法使用 str 类型，但是可以使用 &str 来替代
pub fn case1() {
    let s: &str = "hello, world";
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_case1() {
        case1();
    }
}