/// Adds one to a number.
pub fn plus_one(x: f64) -> f64 {
    x + 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(plus_one(3.0), 4.0);
    }
}
