#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <ctype.h>

int find_first_digit(char* buffer){
    int index = 0;
    while (index < strlen(buffer)){
        char c = buffer[index];
        if (isdigit(c)) {
            return c - '0';
        }
        index++;
    }
    return -1;
}

int find_last_digit(char* buffer){
    int index = strlen(buffer) - 2;
    while (index > -1){
        char c = buffer[index];
        if (isdigit(c)) {
            return c - '0';
        }
        index--;
    }
    return -1;
}

int concatenate_numbers(int x, int y){
    int pow = 10;
    while (y >= pow) {
        pow *= 10;
    }
    return x * pow + y;
}

void run_part1(){
    char* test_input = "/home/brennan/Documents/AdventOfCode/2023/trebuchet/data/part1test.txt";
    char* input = "/home/brennan/Documents/AdventOfCode/2023/trebuchet/data/part1.txt";
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