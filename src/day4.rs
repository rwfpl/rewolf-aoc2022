use std::fs;

fn contained(a: &str, b: &str, x: &str, y: &str) -> bool {
    let a = a.parse::<u32>().unwrap();
    let b = b.parse::<u32>().unwrap();
    let x = x.parse::<u32>().unwrap();
    let y = y.parse::<u32>().unwrap();
    (a >= x && b <= y) || (x >= a && y <= b)
}

fn overlapped(a: &str, b: &str, x: &str, y: &str) -> bool {
    let a = a.parse::<u32>().unwrap();
    let b = b.parse::<u32>().unwrap();
    let x = x.parse::<u32>().unwrap();
    let y = y.parse::<u32>().unwrap();
    (a >= x && a <= y) || (b >= x && b <= y) || (x >= a && x <= b) || (y >= a && y <= b)
}

fn solution<F>(condition: F) -> usize
where
    F: Fn(&str, &str, &str, &str) -> bool,
{
    fs::read_to_string("src/inputs/aoc_4.input")
        .unwrap()
        .lines()
        .filter(|line| {
            let p: Vec<&str> = line.split(',').collect();
            let r1: Vec<&str> = p[0].split('-').collect();
            let r2: Vec<&str> = p[1].split('-').collect();
            condition(r1[0], r1[1], r2[0], r2[1])
        })
        .count()
}

pub fn run() -> (String, String) {
    (
        solution(contained).to_string(),
        solution(overlapped).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(solution(contained), 487);
        assert_eq!(solution(overlapped), 849);
    }
}
