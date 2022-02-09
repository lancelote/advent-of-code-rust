struct Command {
    direction: String,
    units: i64,
}

impl Command {
    fn from_line(line: &str) -> Command {
        let mut parts = line.split_whitespace();
        let direction: String = parts.next().unwrap().to_string();
        let units: i64 = parts.next().unwrap().parse().unwrap();

        Command { direction, units }
    }
}

fn parse_commands(input: &str) -> Vec<Command> {
    input.trim().lines().map(Command::from_line).collect()
}

fn part_a(input: &str) -> i64 {
    let mut horizontal = 0;
    let mut depth = 0;

    for command in parse_commands(input) {
        match command.direction.as_str() {
            "forward" => horizontal += command.units,
            "down" => depth += command.units,
            "up" => depth -= command.units,
            _ => panic!("unknown command"),
        };
    }

    horizontal * depth
}

fn part_b(input: &str) -> i64 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in parse_commands(input) {
        match command.direction.as_str() {
            "forward" => {
                horizontal += command.units;
                depth += aim * command.units;
            }
            "down" => aim += command.units,
            "up" => aim -= command.units,
            _ => panic!("unknown command"),
        };
    }

    horizontal * depth
}

fn main() {
    println!("part a: {:?}", part_a(include_str!("day2_input.txt")));
    println!("part b: {:?}", part_b(include_str!("day2_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::part_a;
    use super::part_b;

    #[test]
    fn test_part_a() {
        assert_eq!(
            part_a("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"),
            150
        );
    }

    #[test]
    fn test_part_b() {
        assert_eq!(
            part_b("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"),
            900
        );
    }
}
