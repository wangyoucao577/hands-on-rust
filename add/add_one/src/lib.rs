
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_one(x: i32) -> i32 {
    x+1
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
    fn add_one_works() {
        assert_eq!(add_one(1), 2);
    }

    use rand::Rng;

    #[test]
    fn add_one_rand_works(){
        let rand_num = rand::thread_rng().gen_range(1..=100);
        assert_eq!(add_one(rand_num), rand_num+1);
    }
}
