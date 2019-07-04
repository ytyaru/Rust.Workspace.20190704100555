use rand::Rng;

pub fn add_one(x: i32) -> i32 { x + 1 }
pub fn get_rand() -> i32 {
    let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します
    let i: i32 = rng.gen();           // genはRng traitに定義されている
    i
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(add_one(1), 2);
    }
}
