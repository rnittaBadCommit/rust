pub trait Animal {
    fn speak(&self) -> &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dog;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cat;

impl Animal for Dog {
    fn speak(&self) -> &'static str {
        "wan"
    }
}

impl Animal for Cat {
    fn speak(&self) -> &'static str {
        "nyan"
    }
}

pub fn speak_once<T: Animal>(animal: &T) -> &'static str {
    animal.speak()
}

#[cfg(test)]
mod tests {
    use super::{speak_once, Animal, Cat, Dog};

    #[test]
    fn dog_implements_animal() {
        let dog = Dog;
        assert_eq!(dog.speak(), "wan");
    }

    #[test]
    fn cat_implements_animal() {
        let cat = Cat;
        assert_eq!(cat.speak(), "nyan");
    }

    #[test]
    fn generic_function_accepts_any_animal() {
        let dog = Dog;
        let cat = Cat;

        assert_eq!(speak_once(&dog), "wan");
        assert_eq!(speak_once(&cat), "nyan");
    }
}
