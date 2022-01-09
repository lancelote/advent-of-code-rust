fn parse_measurements(input: &str) -> Vec<i64> {
    input.trim().lines().map(|x| x.parse().unwrap()).collect()
}

fn part_a(input: &str) -> i64 {
    let measurements = parse_measurements(input);
    let mut count_increases = 0;

    for i in 1..measurements.len() {
        if measurements[i] > measurements[i - 1] {
            count_increases += 1;
        }
    }

    count_increases
}

fn main() {
    println!("part a: {:?}", part_a(include_str!("day1_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::parse_measurements;
    use super::part_a;

    #[test]
    fn test_parse_measurements() {
        assert_eq!(parse_measurements("1\n42\n0\n"), vec![1, 42, 0]);
    }

    #[test]
    fn test_part_a() {
        assert_eq!(
            part_a("199\n200\n208\n210\n200\n207\n240\n269\n260\n263"),
            7
        );
    }
}
