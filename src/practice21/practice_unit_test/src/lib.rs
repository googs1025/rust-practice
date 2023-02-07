pub fn add(left: usize, right: usize) -> usize {
    left + right
}




#[cfg(test)]
mod tests {
    use super::*;

    // 使用 assert_eq!宏
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // 使用 Result<(), String> 返回值
    #[test]
    fn it_works1() -> Result<(), String> {
        let result = add(5, 5);

        // if result == 10 {
        //     Ok(())
        // } else {
        //     Err(String::from("相加有错。"))
        // }

        match result {
            10 => Ok(()),

            _ => Err(String::from("相加有错")),
        }
    }

}



