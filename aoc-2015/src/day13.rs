use std::collections::HashMap;
use std::error::Error;

const PROBLEM: &str = include_str!("input/day13.txt");

fn make_seating_dict(lines: &str) -> HashMap<&str, HashMap<&str, i32>> {
    let mut seating = HashMap::<&str, HashMap<&str, i32>>::new();

    for line in lines.lines() {
        let words: Vec<&str> = line.split(" ").collect();

        let name = words[0];
        let gain_lose = words[2];
        let happiness: i32 = words[3].parse().expect("not a number");
        let other = words[10].strip_suffix(".").expect("not end of line");

        if !seating.contains_key(name) {
            seating.insert(name, HashMap::<&str, i32>::new());
        }

        if let Some(n) = seating.get_mut(name) {
            n.insert(
                other,
                match gain_lose {
                    "gain" => happiness,
                    "lose" => -happiness,
                    _ => panic!("must be gain or lose"),
                },
            );
        }
    }

    seating
}

fn calculate_happiness(seating: &HashMap<&str, HashMap<&str, i32>>, table: &Vec<&str>) -> i32 {
    let mut happiness = 0;

    for i in 0..table.len() {
        happiness += seating
            .get(table[i])
            .expect("missing person")
            .get(table[if i == 0 { table.len() - 1 } else { i - 1 }])
            .expect("missing person");
        happiness += seating
            .get(table[i])
            .expect("missing person")
            .get(table[if i == table.len() - 1 { 0 } else { i + 1 }])
            .expect("missing person");
    }

    // two people leads to double counting
    if table.len() == 2 {
        happiness / 2
    } else {
        happiness
    }
}

fn find_best_seating_happiness(seating: &HashMap<&str, HashMap<&str, i32>>) -> i32 {
    let mut best_score = 0;

    fn backtrack<'a>(
        seating: &HashMap<&'a str, HashMap<&'a str, i32>>,
        table: &'a Vec<&'a str>,
        best_score: &mut i32,
    ) {
        if table.len() == seating.keys().len() {
            let happiness = calculate_happiness(seating, table);

            if happiness > *best_score {
                *best_score = happiness;
            }
        } else {
            for &person in seating.keys() {
                if !table.contains(&person) {
                    let mut next_table = table.to_vec();
                    next_table.push(person);
                    backtrack(seating, &next_table, best_score);
                }
            }
        }
    }

    backtrack(&seating, &vec![], &mut best_score);

    best_score
}

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let seating = make_seating_dict(PROBLEM);
    let happiness = find_best_seating_happiness(&seating);

    println!("{}", happiness);
    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let mut seating = make_seating_dict(PROBLEM);
    let mut my_preference = HashMap::<&str, i32>::new();

    for &person in seating.keys() {
        my_preference.insert(person, 0);
    }

    seating.iter_mut().for_each(|(&_k, v)| {
        v.insert("I", 0);
    });

    seating.insert("I", my_preference);

    let happiness = find_best_seating_happiness(&seating);

    println!("{}", happiness);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_input_correctly() {
        let seating = make_seating_dict(PROBLEM);

        assert_eq!(
            *seating
                .get("Alice")
                .expect("missing Alice")
                .get("Bob")
                .expect("missing Bob"),
            2
        );
        assert_eq!(
            *seating
                .get("Alice")
                .expect("missing Alice")
                .get("Carol")
                .expect("missing Carol"),
            26
        );
        assert_eq!(
            *seating
                .get("Alice")
                .expect("missing Alice")
                .get("David")
                .expect("missing David"),
            -82
        );
        assert_eq!(
            *seating
                .get("Alice")
                .expect("missing Alice")
                .get("Eric")
                .expect("missing Eric"),
            -75
        );
    }

    #[test]
    fn calculate_happiness_correctly() {
        let seating = make_seating_dict(PROBLEM);

        assert_eq!(
            calculate_happiness(&seating, &vec!["Alice", "Bob"]),
            42 as i32
        );
        assert_eq!(
            calculate_happiness(&seating, &vec!["Alice", "Bob", "Carol"]),
            -127 as i32
        );
    }
}
