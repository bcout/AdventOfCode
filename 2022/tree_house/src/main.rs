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
*/


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

fn main() {
    part_1_main();
}
