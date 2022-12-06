fn all_different<const N: usize>(arr: &[char; N]) -> bool {
    for i in 0..(N - 1) {
        for j in (i + 1)..N {
            if arr[i] == arr[j] {
                return false;
            }
        }
    }
    true
}

fn solve<const N: usize>(file_content: &str) -> usize {
    let mut enumerate_chars = file_content.chars().enumerate();
    let mut last: [char; N] = [0 as char; N];
    for i in 0..N {
        last[i] = enumerate_chars.next().unwrap().1;
    }
    if all_different(&last) {
        return N;
    }
    for (i, c) in enumerate_chars {
        for j in 0..(N - 1) {
            last[j] = last[j + 1];
        }
        last[N - 1] = c;
        if all_different(&last) {
            return i + 1;
        }
    }
    0
}
pub fn solve_task1(file_content: &str) -> usize {
    solve::<4>(file_content)
}
pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
    solve::<14>(file_content)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task1() {
        assert_eq!(
            format!("{}", solve_task1("bvwbjplbgvbhsrlpgdmjqwftvncz")),
            "5"
        );
        assert_eq!(
            format!("{}", solve_task1("nppdvjthqldpwncqszvftbrmjlhg")),
            "6"
        );
        assert_eq!(
            format!("{}", solve_task1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
            "10"
        );
        assert_eq!(
            format!("{}", solve_task1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")),
            "11"
        );
    }

    #[test]
    fn test_task2() {
        assert_eq!(
            format!("{}", solve_task2("mjqjpqmgbljsphdztnvjfqwrcgsmlb")),
            "19"
        );
        assert_eq!(
            format!("{}", solve_task2("bvwbjplbgvbhsrlpgdmjqwftvncz")),
            "23"
        );
        assert_eq!(
            format!("{}", solve_task2("nppdvjthqldpwncqszvftbrmjlhg")),
            "23"
        );
        assert_eq!(
            format!("{}", solve_task2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
            "29"
        );
        assert_eq!(
            format!("{}", solve_task2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")),
            "26"
        );
    }
}
