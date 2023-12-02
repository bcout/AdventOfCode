#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define INITIAL_BUFFER_SIZE 255


void print_error_and_exit(char* function_name, char* error_message, int free_flag){
    fprintf(stderr, "ERROR in %s\n\t%s\n", function_name, error_message);
    if (free_flag){
        free(error_message);
    }
    exit(EXIT_FAILURE);
}

char* str_concat(char* s1, char* s2){
    char* result = malloc(strlen(s1) + strlen(s2) + 1);
    if (result == NULL){
        print_error_and_exit("str_concat()", "malloc() failed", 0);
        exit(EXIT_FAILURE); 
    }
    strcpy(result, s1);
    strcat(result, s2);
    return result;
}

FILE* open_file(char* file_path){
    FILE* fp = fopen(file_path, "r");
    if (fp == NULL){
        print_error_and_exit("open_file()", strcat("Could not open ", file_path), 0);
    }
    return fp;
}

long get_filesize(FILE* fp){
    if (fseek(fp, 0, SEEK_END) != 0) {
        print_error_and_exit("get_filesize()", "fseek() failed", 0);
    } 
    return 0;
}

char** read_input_file(char* file_path){
    printf("File path: %s\n", file_path);

    //FILE* fp = open_file(file_path);
    //long filesize = get_filesize(fp);

    void* line_v = malloc(INITIAL_BUFFER_SIZE * sizeof(char));
    if (line_v == NULL){
        print_error_and_exit("read_input_file()", "malloc() failed to allocate space for input line", 0);
    }
    char* line = (char*)line_v;

    free(line);
}