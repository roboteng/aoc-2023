advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let answer = input
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .find_map(|char| char.to_digit(10))
                .unwrap_or_default();
            let last = line
                .chars()
                .rev()
                .find_map(|char| char.to_digit(10))
                .unwrap_or_default();
            first * 10 + last
        })
        .sum();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let answer = input
        .lines()
        .map(|line| {
            let mut starts = Vec::new();

            for pair in numbers {
                if let Some(n) = line.find(pair.0) {
                    starts.push((pair.1, n as u32));
                }
                if let Some(n) = line.find(&pair.1.to_string()) {
                    starts.push((pair.1, n as u32));
                }
            }
            starts.sort_by(|a, b| a.1.cmp(&b.1));
            let first = starts.get(0).map(|a| a.0).unwrap_or_default();

            let mut lasts = Vec::new();

            for pair in numbers {
                if let Some(n) = line.rfind(pair.0) {
                    lasts.push((pair.1, (n + pair.0.len()) as u32));
                }
                if let Some(n) = line.rfind(&pair.1.to_string()) {
                    lasts.push((pair.1, n as u32 + 1));
                }
            }
            lasts.sort_by(|a, b| a.1.cmp(&b.1).reverse());
            let last = lasts.get(0).map(|a| a.0).unwrap_or_default();

            first * 10 + last
        })
        .sum();

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let result = part_two(input);

        assert_eq!(result, Some(281));
    }
}
