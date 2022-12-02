/*
The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking inventory of their supplies. One important consideration is food - in particular, the number of Calories each Elf is carrying (your puzzle input).

The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.

For example, suppose the Elves finish writing their items' Calories and end up with the following list:

1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

This list represents the Calories of the food carried by five Elves:

    The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
    The second Elf is carrying one food item with 4000 Calories.
    The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
    The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
    The fifth Elf is carrying one food item with 10000 Calories.

In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is 24000 (carried by the fourth Elf).

Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?

--------------------------------------------------------------------------------------------------------
--------------------------------------------------------------------------------------------------------
--------------------------------------------------------------------------------------------------------

By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories of food might 
    eventually run out of snacks.

To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the top three Elves carrying the most 
    Calories. That way, even if one of those Elves runs out of snacks, they still have two backups.

In the example above, the top three Elves are the fourth Elf (with 24000 Calories), then the third Elf (with 11000 Calories), then the fifth 
    Elf (with 10000 Calories). The sum of the Calories carried by these three elves is 45000.

Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
*/

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Part 1 of the challenge
    get_max_calorie_count();
    // Part 2 (keeping them separate even though they're basically the same)
    get_top_three_calorie_counts();
}

// Stolen from the rust by example page https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/*
This function reads every line of the file and sums up the integers in groups separated by empty lines
Once all the group sums are calculated, it prints out the highest value.
*/
fn get_max_calorie_count() {
    let mut calorie_count: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;

    let lines = read_lines("./input.txt")
        .expect("Failed to read lines");
    for line in lines {
        // I guess Ok() just ignores the potential error that could come from assigning "line" a value
        if let Ok(line_contents) = line {
            if line_contents.is_empty() {
                // Store the current total in the calorie_count vector
                calorie_count.push(sum);
                sum = 0;
            }
            else {
                let number: i32 = line_contents.parse::<i32>()
                    .expect("Failed to convert line to integer");
                sum = sum + number;
            }
        }
    }
    calorie_count.push(sum);
    println!("{:?}", calorie_count);
    let max_value = calorie_count.iter().max()
        .expect("Failed to iterate and find max");
    println!("Max calorie count {}", max_value);
}


/*
This function does the same thing as get_max_calorie_count() except at the end it takes the top three highest counts,
adds them together and prints the value out.
 */
fn get_top_three_calorie_counts(){
    let mut calorie_count: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;

    let lines = read_lines("./input.txt")
        .expect("Failed to read lines");
    for line in lines {
        // I guess Ok() just ignores the potential error that could come from assigning "line" a value
        if let Ok(line_contents) = line {
            if line_contents.is_empty() {
                calorie_count.push(sum);
                sum = 0;
            }
            else {
                let number: i32 = line_contents.parse::<i32>()
                    .expect("Failed to convert line to integer");
                sum = sum + number;
            }
        }
    }
    calorie_count.push(sum);
    calorie_count.sort();
    println!("{:?}", calorie_count);
    // Reverses the previously sorted array, takes the first 3 elements and sums them up.
    // We reverse it so that the highest value items are first
    let last_three_sum: i32 = calorie_count.iter().rev().take(3).sum();
    println!("{}", last_three_sum);
}