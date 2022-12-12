/*
The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition 
    planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a tree house.

First, determine whether there is enough tree cover here to keep a tree house hidden. To do this, you need to count the number of trees that 
    are visible from outside the grid when looking directly along a row or column.

The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). For example:

30373
25512
65332
33549
35390

Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.

A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; 
    that is, only look up, down, left, or right from any given tree.

All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view. In this example, 
    that only leaves the interior nine trees to consider:

    The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
    The top-middle 5 is visible from the top and right.
    The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
    The left-middle 5 is visible, but only from the right.
    The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
    The right-middle 3 is visible from the right.
    In the bottom row, the middle 5 is visible, but the 3 and 4 are not.

With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.

Consider your map; how many trees are visible from outside the grid?

--------------------------------------------------------------------------------------------------------
--------------------------------------------------------------------------------------------------------
--------------------------------------------------------------------------------------------------------

Content with the amount of tree cover available, the Elves just need to know the best spot to build their tree house: they would like to be able to 
    see a lot of trees.

To measure the viewing distance from a given tree, look up, down, left, and right from that tree; stop if you reach an edge or at the first tree that 
    is the same height or taller than the tree under consideration. (If a tree is right on the edge, at least one of its viewing distances will be zero.)

The Elves don't care about distant trees taller than those found by the rules above; the proposed tree house has large eaves to keep it dry, so they 
    wouldn't be able to see higher than the tree house anyway.

In the example above, consider the middle 5 in the second row:

30373
25512
65332
33549
35390

    Looking up, its view is not blocked; it can see 1 tree (of height 3).
    Looking left, its view is blocked immediately; it can see only 1 tree (of height 5, right next to it).
    Looking right, its view is not blocked; it can see 2 trees.
    Looking down, its view is blocked eventually; it can see 2 trees (one of height 3, then the tree of height 5 that blocks its view).

A tree's scenic score is found by multiplying together its viewing distance in each of the four directions. For this tree, this is 4 (found by 
    multiplying 1 * 1 * 2 * 2).

However, you can do even better: consider the tree of height 5 in the middle of the fourth row:

30373
25512
65332
33549
35390

    Looking up, its view is blocked at 2 trees (by another tree with a height of 5).
    Looking left, its view is not blocked; it can see 2 trees.
    Looking down, its view is also not blocked; it can see 1 tree.
    Looking right, its view is blocked at 2 trees (by a massive tree of height 9).

This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the tree house.

Consider each tree on your map. What is the highest scenic score possible for any tree?
*/

use std::cmp::max;


fn print_matrix(matrix: &Vec<Vec<u32>>, row_count: usize, col_count: usize) {
    for row in 0..row_count {
        for col in 0..col_count {
            print!("{}", matrix[row][col]);
        }
        println!();
    }
}

fn get_number_of_lines(input: &str) -> usize{
    let mut lines: usize = 0;
    for _ in input.lines() {
        lines = lines + 1;
    }
    return lines;
}

fn check_tree_visibility(matrix: &Vec<Vec<u32>>, row: usize, col: usize, num_rows: usize, num_cols: usize) -> usize {

    let current_value = matrix[row][col];
    let mut north = 1;
    let mut south = 1;
    let mut east = 1;
    let mut west = 1;

    // Check north
    for i in (0..row).rev() {
        if matrix[i][col] >= current_value {
            north = 0;
        }
    }
    
    // Check west
    for i in (0..col).rev() {
        if matrix[row][i] >= current_value {
            west = 0;
        }
    }
    
    // Check east
    for i in (col+1)..num_cols {
        if matrix[row][i] >= current_value {
            east = 0;
        }
    }

    // Check south
    for i in (row+1)..num_rows {
        if matrix[i][col] >= current_value {
            south = 0;
        }
    }
    
    if (north == 1) || (west == 1) | (east == 1) | (south == 1) {
        return 1;
    }
    else {
        return 0;
    }
}

fn part_1_main() {

    let input: &str = include_str!("../input.txt");
    let num_lines: usize = get_number_of_lines(input);
    let line_length: usize = input.lines().next().unwrap().len();
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; line_length]; num_lines];

    for (row, line) in input.lines().enumerate() {
        for (col, character) in line.chars().enumerate(){
            matrix[row][col] = character.to_digit(10).unwrap();
        }
    }

    //print_matrix(&matrix, num_lines, line_length);

    let mut num_visible_trees: usize = 0;
    for row in 1..num_lines - 1 {
        for col in 1..line_length - 1 {
            num_visible_trees = num_visible_trees + check_tree_visibility(&matrix, row, col, num_lines, line_length);
        }
    }

    let num_perimeter_trees = (num_lines * 2) + ((line_length * 2) - 4);

    println!("Number of visible trees: {}", num_visible_trees + num_perimeter_trees);

    
}

fn get_tree_score(matrix: &Vec<Vec<u32>>, row: usize, col: usize, num_rows: usize, num_cols: usize) -> usize {
    let current_value = matrix[row][col];
    // The number of trees visible in each direction
    let mut north = 0;
    let mut south = 0;
    let mut east = 0;
    let mut west = 0;

    // Check north
    for i in (0..row).rev() {
        north = north + 1;
        if matrix[i][col] >= current_value {
            break;
        }
    }
    
    // Check west
    for i in (0..col).rev() {
        west = west + 1;
        if matrix[row][i] >= current_value {
            break;
        }
    }
    
    // Check east
    for i in (col+1)..num_cols {
        east = east + 1;
        if matrix[row][i] >= current_value {
            break;
        }
    }

    // Check south
    for i in (row+1)..num_rows {
        south = south + 1;
        if matrix[i][col] >= current_value {
            break;
        }
    }

    return north * south * east * west;
}

fn part_2_main() {
    let input: &str = include_str!("../input.txt");
    let num_lines: usize = get_number_of_lines(input);
    let line_length: usize = input.lines().next().unwrap().len();
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; line_length]; num_lines];

    for (row, line) in input.lines().enumerate() {
        for (col, character) in line.chars().enumerate(){
            matrix[row][col] = character.to_digit(10).unwrap();
        }
    }

    let mut high_score: usize = 0;
    for row in 1..num_lines - 1 {
        for col in 1..line_length - 1 {
            high_score = max(high_score, get_tree_score(&matrix, row, col, num_lines, line_length));
        }
    }

    println!("{}", high_score);
}

fn main() {
    //part_1_main();
    part_2_main();
}
