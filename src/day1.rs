use std::error::Error;

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let problem = include_str!("day1.txt");

    let max = problem.split("\n\n").map(|list| -> u32 {
        list.split("\n")
            .map(|string| string.parse::<u32>().unwrap())
            .sum()
    }).max().expect("Failed to run on input");

    println!("Answer: {}", max);

    Ok(())
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let problem = include_str!("day1.txt");
    let mut top_3: [u32; 3] = [0; 3];

    let calories = problem.split("\n\n").map(|list| -> u32 {
        list.split("\n")
            .map(|string| string.parse::<u32>().unwrap())
            .sum()
    });

    for n in calories {
        if n > top_3[0] {
            let mut i = 1;
            top_3[0] = n;

            while i < 3 && n > top_3[i] {
                top_3[i-1] = top_3[i];
                top_3[i] = n;
                i += 1;
            }
        }
    }

    println!("Answer: {}", top_3.iter().sum::<u32>());

    Ok(())
}

#[cfg(test)]
mod tests {

}
