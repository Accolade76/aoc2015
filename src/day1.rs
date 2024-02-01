mod elevator_code {
    fn elevator(instructions: String) -> i32 {
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
        use crate::input_reader::read_input_file;
        use super::*;
        #[test]
        fn does_lift_reach_correct_floor() {
            assert_eq!(1, elevator("(".to_string()));
            assert_eq!(2, elevator("((".to_string()));
            assert_eq!(1, elevator("(()".to_string()));
            assert_eq!(0, elevator("(())".to_string()));
            assert_eq!(3, elevator("))(((((".to_string()));
            assert_eq!(-3, elevator(")())())".to_string()));
        }

        #[test]
        fn read_instructions_from_file(){
            let instructions = read_input_file("day1_input.txt");
            assert_eq!(74, elevator(instructions));
        }
    }
}