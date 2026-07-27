pub trait TargetTrait {}

impl TargetTrait for usize {}

pub struct TargetType;

impl TargetType {
    pub fn test_fn(arg: String) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
