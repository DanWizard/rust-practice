struct Rectangle {
    width: u16,
    height: u16,
}

impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {}

fn give_two() -> i8 {
    return 2;
}

#[cfg(test)]
mod dcode_tests {
    #[test]
    #[should_panic]
    fn test_basic() {
        assert!(1 == 1); // OK
        panic!("Oh no!")
    }

    #[test]
    #[ignore]
    fn test_equals() {
        assert_eq!(super::give_two(), 1 + 1);
        assert_ne!(2, 1 + 2);
    }

    #[test]
    #[should_panic]
    fn test_struct() {
        let r = super::Rectangle {
            width: 50,
            height: 25,
        };

        assert!(r.is_square())
    }
}
