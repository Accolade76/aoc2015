mod elevator_code {
    fn elevator(instructions: &str) -> i32 {
        let mut floor = 0;
        for i in instructions.chars() {
            if i == '(' {
                floor = floor + 1;
            }
            if i == ')' {
                floor = floor - 1;
            }
        }
        floor
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn apartment_lift_floor() {
            assert_eq!(1, elevator("("));
            assert_eq!(2, elevator("(("));
            assert_eq!(1, elevator("(()"));
        }
    }
}