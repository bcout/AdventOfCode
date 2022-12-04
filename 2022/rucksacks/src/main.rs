/*
One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey. Unfortunately, that Elf didn't quite 
    follow the packing instructions, and so a few items now need to be rearranged.

Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments. The Elf that 
    did the packing failed to follow this rule for exactly one item type per rucksack.

The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors. 
    Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).

The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in 
    each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the 
    characters represent items in the second compartment.

For example, suppose you have the following list of contents from six rucksacks:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

    The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr, while 
        the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
    The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both compartments is 
        uppercase L.
    The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
    The fourth rucksack's compartments only share item type v.
    The fifth rucksack's compartments only share item type t.
    The sixth rucksack's compartments only share item type s.

To help prioritize item rearrangement, every item type can be converted to a priority:

    Lowercase item types a through z have priorities 1 through 26.
    Uppercase item types A through Z have priorities 27 through 52.

In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 
    20 (t), and 19 (s); the sum of these is 157.

Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?

--------------------------------------------------------------------------------------------------------
--------------------------------------------------------------------------------------------------------
--------------------------------------------------------------------------------------------------------

As you finish identifying the misplaced items, the Elves come to you with another issue.

For safety, the Elves are divided into groups of three. Every Elf carries a badge that identifies their group. For efficiency, within each 
    group of three Elves, the badge is the only item type carried by all three Elves. That is, if a group's badge is item type B, then all 
    three Elves will have item type B somewhere in their rucksack, and at most two of the Elves will be carrying any other item type.

The problem is that someone forgot to put this year's updated authenticity sticker on the badges. All of the badges need to be pulled out of the 
    rucksacks so the new authenticity stickers can be attached.

Additionally, nobody wrote down which item type corresponds to each group's badges. The only way to tell which item type is the right one is by 
    finding the one item type that is common between all three Elves in each group.

Every set of three lines in your list corresponds to a single group, but each group can have a different badge item type. So, in the above example, 
    the first group's rucksacks are the first three lines:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg

And the second group's rucksacks are the next three lines:

wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

In the first group, the only item type that appears in all three rucksacks is lowercase r; this must be their badges. In the second group, their 
    badge item type must be Z.

Priorities for these items must still be found to organize the sticker attachment efforts: here, they are 18 (r) for the first group and 52 (Z) 
    for the second group. The sum of these is 70.

Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?
*/

// TWIST: Need to use Waldo Search (for part 1)

#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashSet;

use rand::Rng;

/*
    We know the string is only ASCII letters, no unicode or digits, so there are no checks for that
    a-z: 1-26
    A-Z: 27-52
*/
fn get_letter_priority(letter: u8) -> i32{
    let mut priority = 0;
    let letter_char = letter as char;
    if letter_char.is_lowercase() {
        priority = letter - 96;
    }
    else {
        priority = letter - 38;
    }
    return priority as i32;
}

/*
    Uses Waldo Search to find the lone letter that occurs in both strings

    "Inspired by the common behaviour people exhibit when looking for Waldo, the Waldo Search starts off by wildly glancing around the array, 
        hoping to spot Waldo by chance. The number of times the search does this is proportional to the size of the array (the bigger the Waldo 
        map the longer people spend glancing around wildly). The search may also check the same places multiple times, something people do a lot.

    Once the random guesswork has been abandoned a linear search is performed, not skipping over previously-checked indices (similar to how people 
        will eventually just start at the top of the map and go inch by inch down the page looking for Waldo).

    It's bound to find Waldo eventually, you might even be lucky enough to find it in your first attempt"
    - Brennan, circa June 2020
*/
fn waldo_search(a: &str, b: &str) -> u8{
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    
    // Determine how many random guesses we get
    // This is an arbitrary formula
    let num_searches = (a_bytes.len() + b_bytes.len()) / 3;

    let mut index = 0;
    for i in 0..num_searches {
        // Pick a random index to search
        index = rand::thread_rng().gen_range(0..a_bytes.len());
        if a_bytes[index] == b_bytes[index] {
            return a_bytes[index];
        }
    }
    
    // Didn't find it by randomly searching
    for i in 0..a_bytes.len() {
        for j in 0..b_bytes.len() {
            if a_bytes[i] == b_bytes[j] {
                return a_bytes[i];
            }
        }
    }

    return 0;
}


fn part_1_main() {
    let input: &str = include_str!("../input.txt");
    let mut sum = 0;
    for line in input.split("\n"){
        if line.is_empty() {
            continue;
        }
        
        let first_half = &line[0..line.len()/2];
        let second_half = &line[line.len()/2..line.len()];
        let duplicate_letter = waldo_search(first_half, second_half);
        if duplicate_letter == 0 {
            eprintln!("Did not find a duplicate letter. The duplicate exists, so this is a bug to be fixed.");
            return;
        }
        sum = sum + get_letter_priority(duplicate_letter);
    }
    println!("{sum}");
}

fn find_common_letter(a: &str, b: &str, c: &str) -> char{
    for i in a.chars() {
        for j in b.chars() {
            if i == j {
                for k in c.chars() {
                    if i == k {
                        return k;
                    }
                }
            }
        } 
    }
    return '\0';
}

fn input_to_vec<'a>(input: &'a str, mut vector: &mut Vec<&'a str>) {
    for line in input.lines() {
        vector.push(line);
    }
}

fn part_2_main() {
    let mut strings: Vec<&str> = Vec::new();
    let mut sum = 0;

    let input: &str = include_str!("../input.txt");
    input_to_vec(input, &mut strings);
    
    let mut i = 0;
    let max_lines = strings.len();
    while i < max_lines {
        let a = strings[i];
        let b = strings[i + 1];
        let c = strings[i + 2];

        let common_letter = find_common_letter(a, b, c);
        if common_letter == '\0' {
            eprintln!("Could not find a common letter. This should not happen.");
            return;
        }
        sum = sum + get_letter_priority(common_letter as u8);
        i = i + 3;
    }

    println!("{sum}");
   
}

fn main() {
    //part_1_main();
    part_2_main();
}

