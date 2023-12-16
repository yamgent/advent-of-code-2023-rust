const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/15/input.txt");

fn hash(text: &str) -> u8 {
    text.chars()
        .fold(0u8, |acc, ch| acc.wrapping_add(ch as u8).wrapping_mul(17))
}

fn p1(input: &str) -> String {
    input
        .trim()
        .split(',')
        .map(hash)
        .map(|val| val as u64)
        .sum::<u64>()
        .to_string()
}

fn p2(input: &str) -> String {
    let _input = input.trim();
    "".to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1("HASH"), "52");
        assert_eq!(p1(SAMPLE_INPUT), "1320");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "507291");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
