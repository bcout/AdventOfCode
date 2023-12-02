
#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>

#include "../include/part1.h"
#include "../include/part2.h"

/*
    Run using make run part=x, where x is either 1 or 2
*/
int main(int argc, char** argv){

    if (argc < 2){
        printf("No argument provided. Use make run part=x\n");
        return -1;
    }

    const char* nptr = argv[1];
    char* endptr;
    errno = 0;
    long part_num = strtol(nptr, &endptr, 10);

    if (nptr == endptr) {
        printf("Argument is not an integer: %s\n", nptr);
        return EXIT_FAILURE;
    }
    if (errno != 0 && part_num == 0) {
        fprintf(stderr, "Error occured %s", strerror(errno));
        return EXIT_FAILURE;
    }

    switch (part_num){
        case 1:
            run_part1();
            break;
        case 2:
            run_part2();
            break;
        default:
            printf("%lu is not a valid part number\n", part_num);
    }
        
}