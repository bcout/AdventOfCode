from dataclasses import dataclass
import re

@dataclass
class Set:
    red: int
    blue: int
    green: int


def part1():
    input_path = "/home/brennan/Documents/AdventOfCode/2023/cube_conundrum/files/part1.txt"
    test_input_path = "/home/brennan/Documents/AdventOfCode/2023/cube_conundrum/files/part1test.txt"
    input = input_path

    max_reds = 12
    max_blues = 14
    max_greens = 13
    sum = 0

    f = open(input, "r")
    for line in f:
        green_total = 0
        blue_total = 0
        red_total = 0

        split_line = line.split(":", 1)
        game_num = split_line[0].split(" ", 1)[1]
        sets = split_line[1]
        split_sets = sets.split(";", -1)
        for set in split_sets:
            numbers = re.findall(r'\d+ \w', set)
            for subset in numbers:
                split_numbers = subset.split(" ", -1)
                key = split_numbers[1]
                value = split_numbers[0]
                if key == 'r':
                    red_total += int(value)
                if key == 'g':
                    green_total += int(value)
                if key == 'b':
                    blue_total += int(value)

        if red_total <= max_reds and green_total <= max_greens and blue_total <= max_blues:
            sum += int(game_num)

    print(sum)

if __name__ == "__main__":
    part1()