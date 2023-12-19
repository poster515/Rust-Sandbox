pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_one(x: usize) -> usize {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
