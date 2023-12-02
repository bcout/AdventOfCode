
#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>

#include "../include/part1.h"
#include "../include/part2.h"
#include "../include/utils.h"

/*
    Run using make run part=x, where x is either 1 or 2
*/
int main(int argc, char** argv){

    if (argc < 2){
        print_error_and_exit("main()", "No argument provided. Use make run part=x", 0);
    }

    char* nptr = argv[1];
    char* endptr;
    errno = 0;
    long part_num = strtol(nptr, &endptr, 10);

    if (nptr == endptr) {
        print_error_and_exit("main()", str_concat("Argument is not an integer: ", nptr), 1);
    }
    if (errno != 0 && part_num == 0) {
        print_error_and_exit("main()", str_concat("Error occured ", strerror(errno)), 1);
    }

    switch (part_num){
        case 1:
            run_part1();
            break;
        case 2:
            run_part2();
            break;
        default:
            print_error_and_exit("main()", str_concat("Invalid part number: ", nptr), 1);
            break;
    }
}