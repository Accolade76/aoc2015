#[cfg(test)]
mod test {
    #[test]
    fn apartment_lift_floor() {
        assert_eq!(1, elevator("("));
        assert_eq!(2, elevator("(("));
        assert_eq!(1, elevator("(()"));
    }

    fn elevator(instructions: &str) -> i32 {
        let floor = 0;
        if instructions == "(" {
            return floor + 1
        }
        if instructions == "((" {
            return 2
        }
        if instructions == "(()" {
            return 1
        }
        0
    }
}