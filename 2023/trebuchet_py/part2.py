

def add_numbers_to_words(line):
    # Wherever there is a word like "eight", add the number to the middle of the word
    # eight becomes ei8ght
    # eightwone becomes ei8htw2o1ne, so the first and last numbers are 8 and 1
    new_line = line.replace("one", "o1ne")
    new_line = new_line.replace("two", "t2wo")
    new_line = new_line.replace("three", "th3ree")
    new_line = new_line.replace("four", "fo4ur")
    new_line = new_line.replace("five", "fi5ve")
    new_line = new_line.replace("six", "si6x")
    new_line = new_line.replace("seven", "sev7en")
    new_line = new_line.replace("eight", "ei8ght")
    new_line = new_line.replace("nine", "ni9ne")
    return new_line


def find_first_digit(line):
    for char in line:
        if char.isdigit():
            return char


def find_last_digit(line):
    new_line = line[::-1]
    for char in new_line:
        if char.isdigit():
            return char


def main():
    input_path = "/home/brennan/Documents/AdventOfCode/2023/trebuchet_py/files/part2.txt"
    test_input_path = "/home/brennan/Documents/AdventOfCode/2023/trebuchet_py/files/part2test.txt"
    input = input_path

    sum = 0
    f = open(input, "r")
    for line in f:
        new_line = add_numbers_to_words(line)
        first = find_first_digit(new_line)
        last = find_last_digit(new_line)
        combined = int(str(first) + str(last))
        sum += combined
        
    print(sum)
    

if __name__ == "__main__":
    main()