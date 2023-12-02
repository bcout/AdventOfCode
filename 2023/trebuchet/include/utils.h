
/*
    Reads the input file from the provided path and returns an array of strings for the lines in that file
*/
char** read_input_file(char* file_path);

/*
    Generic function for printing errors and exiting the program.
    If you're passing an error_message that was made using str_concat(), that value
    needs to be freed, so make sure to supply 1 for the free flag, or else
    there will be a memory leak.

    For example:
    print_error_and_exit("main()", str_concat("Error occured ", strerror(errno)), 1);
    vs
    print_error_and_exit("main()", "Error occured", 0);
*/
void print_error_and_exit(char* function_name, char* error_message, int free_flag);

char* str_concat(char* s1, char* s2);