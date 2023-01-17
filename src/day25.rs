use std::{collections::VecDeque, fs};

fn snafu_to_dec(snafu: &str) -> i64 {
    snafu
        .chars()
        .rev()
        .fold((0, 1), |a, c| {
            (
                match c {
                    '2' => a.0 + 2 * a.1,
                    '1' => a.0 + a.1,
                    '0' => a.0,
                    '-' => a.0 - a.1,
                    '=' => a.0 - 2 * a.1,
                    _ => panic!("Invalid SNAFU character."),
                },
                a.1 * 5,
            )
        })
        .0
}

fn dec_to_snafu(v: i64) -> String {
    let trans = ['=', '-', '0', '1', '2'];
    let mut v = v;
    let mut r = VecDeque::new();
    let mut borrow = false;
    while v != 0 {
        let mut x = v % 5 + 2;
        if borrow {
            x += 1;
            borrow = false;
        }
        if x > 4 {
            borrow = true;
        }
        r.push_front(trans[x as usize % 5]);
        v /= 5;
    }
    if borrow {
        r.push_front('1');
    }
    r.into_iter().collect::<String>()
}

fn solution(filename: &str) -> String {
    dec_to_snafu(
        fs::read_to_string(filename)
            .unwrap()
            .lines()
            .map(snafu_to_dec)
            .sum(),
    )
}

pub fn run() -> (String, String) {
    (solution("src/inputs/aoc_25.input"), "".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu_to_dec() {
        assert_eq!(snafu_to_dec("1=-0-2"), 1747);
        assert_eq!(snafu_to_dec("12111"), 906);
        assert_eq!(snafu_to_dec("2=0="), 198);
        assert_eq!(snafu_to_dec("21"), 11);
        assert_eq!(snafu_to_dec("2=01"), 201);
        assert_eq!(snafu_to_dec("111"), 31);
        assert_eq!(snafu_to_dec("20012"), 1257);
        assert_eq!(snafu_to_dec("112"), 32);
        assert_eq!(snafu_to_dec("1=-1="), 353);
        assert_eq!(snafu_to_dec("1-12"), 107);
        assert_eq!(snafu_to_dec("12"), 7);
        assert_eq!(snafu_to_dec("1="), 3);
        assert_eq!(snafu_to_dec("122"), 37);
    }

    #[test]
    fn test_dec_to_snafu() {
        assert_eq!(dec_to_snafu(1), "1");
        assert_eq!(dec_to_snafu(2), "2");
        assert_eq!(dec_to_snafu(3), "1=");
        assert_eq!(dec_to_snafu(4), "1-");
        assert_eq!(dec_to_snafu(5), "10");
        assert_eq!(dec_to_snafu(6), "11");
        assert_eq!(dec_to_snafu(7), "12");
        assert_eq!(dec_to_snafu(8), "2=");
        assert_eq!(dec_to_snafu(9), "2-");
        assert_eq!(dec_to_snafu(10), "20");
        assert_eq!(dec_to_snafu(15), "1=0");
        assert_eq!(dec_to_snafu(20), "1-0");
        assert_eq!(dec_to_snafu(2022), "1=11-2");
        assert_eq!(dec_to_snafu(12345), "1-0---0");
        assert_eq!(dec_to_snafu(314159265), "1121-1110-1=0");
    }

    #[test]
    fn test_run() {
        assert_eq!(solution("src/inputs/aoc_25_sample.input"), "2=-1=0");
        assert_eq!(solution("src/inputs/aoc_25.input"), "2=--=0000-1-0-=1=0=2");
    }
}
