use std::ops::{Add, Sub, Mul, Div};

fn do_operation<F, V>(f: F, v1: V, v2: V) -> V
    where 
        F: Fn(V, V) -> V,
        V: Add + Sub + Mul + Div {
    f(v1, v2)
}

pub fn rpn(raw: &str) -> f64 {
    let mut stack = Vec::new();

    for c in raw.split(' ') {
        if let Some(i) = c.parse::<f64>().ok() {
            stack.push(i);
            continue;
        }

        let r = stack.pop().expect("Invalid equation. No numbers left in stack for the right side of the operation");
        let l = stack.pop().expect("Invalid equation. No numbers left in stack for the left side of the operation");

        let result = match c {
            "+" => do_operation(|l, r| l + r, l, r),
            "-" => do_operation(|l, r| l - r, l, r),
            "*" => do_operation(|l, r| l * r, l, r),
            "/" => 
            {
                if r == 0.0
                {
                    panic!("Division by zero not allowed");
                }

                do_operation(|l, r| l / r, l, r) 
            },
            _ => panic!("Unknown character {:?}", c),
        };

        stack.push(result);
    }

    if stack.len() != 1
    {
        panic!("Invalid equation. Wrong number of elements left in stack. Expected left: 1, actual: {:?}", stack.len());
    }

    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_integer() {
        assert_eq!(0.25, rpn("14 4 6 8 + * /"));
        assert_eq!(14.0, rpn("5 1 2 + 4 * + 3 -"));
        assert_eq!(0.5, rpn("5 4 6 + /"));
        assert_eq!(2.0, rpn("2 5 * 4 + 3 2 * 1 + /"));
    }

    #[test]
    fn basic_floating_point() {
        assert_eq!(20.04, rpn("5.5 1.3 2.3 + 4.9 * + 3.1 -"));
        assert_eq!(11.25, rpn("1.5 3.0 4.5 + *"));
    }

    #[test]
    fn negative() {
        assert_eq!(-2503.0, rpn("-4 -9 -33 -76 * + -"));
        assert_eq!(2653660.0, rpn("-56 -34 + -54 * 43 23 54 + * -800 * -"));
    }

    #[test]
    #[should_panic]
    fn divide_by_zero() {
        assert_eq!(-2503.0, rpn("2 0 /"));
    }

    #[test]
    #[should_panic]
    fn invalid_input_1() {
        rpn("");
    }

    #[test]
    #[should_panic]
    fn invalid_input_2() {
        rpn("14 4 6 8 + . /");
    }

    #[test]
    #[should_panic]
    fn invalid_input_3() {
        rpn("POTATO");
    }

    #[test]
    #[should_panic]
    fn invalid_input_4() {
        rpn("54 4 6 O + \\ /");
    }

    #[test]
    fn long_equation() {
        let mut eq = "2 ".to_string();
        for _ in 0..2000000 {
            eq.push_str("2 + ");
        }

        eq.push_str("1 +");

        assert_eq!(4000003.0, rpn(&eq));
    }
}