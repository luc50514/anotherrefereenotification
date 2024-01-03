pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}
pub fn recursive_solution(n: i32) -> i32 {
    match n {
        0 => 0,
        _ => n + recursive_solution(n - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }

    #[test]
    fn recursive_solution_should_return_21_when_given_6() {
        assert_eq!(21, recursive_solution(6))
    }

    #[test]
    fn recursive_solution_should_return_28_when_given_7() {
        assert_eq!(28, recursive_solution(7))
    }
}
