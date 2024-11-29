use std::error::Error;

const PROBLEM: &str = include_str!("input/day5.txt");

trait Count {
    fn count(&self, c: char) -> usize;
}

impl Count for &str {
    fn count(&self, c: char) -> usize {
        self.chars()
            .filter(|ch| *ch == c)
            .collect::<Vec<char>>()
            .len()
    }
}

/// A string is nice if all of the following are true:
/// - It contains at least three vowels (aeiou only), like aei, xazegov, or
///   aeiouaeiouaeiou.
/// - It contains at least one letter that appears twice in a row, like xx,
///   abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
/// - It does not contain the strings ab, cd, pq, or xy, even if they are part
///   of one of the other requirements.
fn is_nice_1(string: &str) -> bool {
    let mut twice_in_a_row = false;
    let mut last_char = '\0';

    for ch in string.chars() {
        if last_char == '\0' {
            last_char = ch;
        } else {
            if ch == last_char {
                twice_in_a_row = true;
                break;
            } else {
                last_char = ch;
            }
        }
    }

    twice_in_a_row
        && string.count('a')
            + string.count('e')
            + string.count('i')
            + string.count('o')
            + string.count('u')
            >= 3
        && !(string.contains("ab")
            || string.contains("cd")
            || string.contains("pq")
            || string.contains("xy"))
}

/// A string is nice if all of the following are true:
/// - It contains a pair of any two letters that appears at least twice in the
///   string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not
///   like aaa (aa, but it overlaps).
/// - It contains at least one letter which repeats with exactly one letter
///   between them, like xyx, abcdefeghi (efe), or even aaa.
fn is_nice_2(string: &str) -> bool {
    assert!(string.is_ascii());
    let mut has_repeating_letter = false;
    let chars = string.as_bytes();

    for i in 0..string.len() - 2 {
        let slice = &chars[i..i + 3];

        if slice[0] == slice[2] {
            has_repeating_letter = true;
            break;
        }
    }

    let mut has_repeating_pair = false;

    for i in 0..string.len() - 2 {
        let pair = &string[i..i + 2];

        for j in i + 2..string.len() - 1 {
            if pair == &string[j..j + 2] {
                has_repeating_pair = true;
                break;
            }
        }
    }

    has_repeating_letter && has_repeating_pair
}

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let nice_strings = PROBLEM
        .lines()
        .filter(|line| is_nice_1(line))
        .collect::<Vec<&str>>()
        .len();

    println!("Answer: {}", nice_strings);

    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let nice_strings = PROBLEM
        .lines()
        .filter(|line| is_nice_2(line))
        .collect::<Vec<&str>>()
        .len();

    println!("Answer: {}", nice_strings);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day5::is_nice_2;

    #[test]
    fn is_nice() {
        assert!(is_nice_2("xxxx"));
        assert!(is_nice_2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_2("xxyxx"));
        assert!(!is_nice_2("aaa"));
        assert!(!is_nice_2("ieodomkazucvgmuy"));
        assert!(!is_nice_2("uurcxstgmygtbstg"));
    }
}
