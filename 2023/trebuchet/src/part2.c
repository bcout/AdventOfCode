#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <ctype.h>

int find_first_digit(char* buffer){
    
}

int find_last_digit(char* buffer){
    
}

int concatenate_numbers(int x, int y){
    int pow = 10;
    while (y >= pow) {
        pow *= 10;
    }
    return x * pow + y;
}

void run_part2(){
    char* test_input = "/home/brennan/Documents/AdventOfCode/2023/trebuchet/data/part2test.txt";
    char* input = "/home/brennan/Documents/AdventOfCode/2023/trebuchet/data/part2.txt";
    char* file = input;
    int buf_size = 255;
    char buffer[buf_size];

    FILE* fp = fopen(file, "r");
    if (fp == NULL){
        fprintf(stderr, "Error in run_part1()\n\tCould not open: %s\n\t%s\n", file, strerror(errno));
        exit(EXIT_FAILURE);
    }   

    int sum = 0; 

    while (fgets(buffer, buf_size, fp) != NULL) {
        int first_digit = find_first_digit(buffer);
        int last_digit = find_last_digit(buffer);
        int num = concatenate_numbers(first_digit, last_digit);
        printf("%d\n", num);

        sum += num;
    }
    printf("\n");

    printf("%d\n", sum);
    fclose(fp);
}