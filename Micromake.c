#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/**
 * Prints a message in red text.
 * 
 * @param message The error message to print.
 */
void printRed(char *message) {
    printf("\033[1;31m");
    printf(message);
    printf("\033[0m");
}

/**
 * Prints an error message in red text.
 * 
 * @param message The error message to print.
 */
void error(char *message) {
    printRed("Micromake Error:\n");
    printRed(message);
    exit(1);
}

/**
 * C program which makes command line parameters to run a Micron program.
 *  This is accomplished by making system calls.
 * Since Micron is a Rust program, we need to navigate to either the debug or release folder and run the program.
 *  This program will assume that the Micron program is being run from the /target/debug/ folder.
 *  If the -r flag is provided, the program will run the Micron program from the /target/release/ folder.
 * 
 * Example Usages
 * Run Debug: micron <program>
 * Run Release: micron -r <program>
 */
int main(int argc, char *argv[]) {
    // Check if the user has provided a Micron program to run
    if (argc < 2) {
        error("Usage: micron <program>\nUsage: micron -r <program>\n");
        return 1;
    }

    // Get the absolute path of the file to run
    char abs_path[300];
    _fullpath(abs_path, argv[argc - 1], sizeof(abs_path));

    // Check if the user provided the -r flag
    if (argc == 3 && strcmp(argv[1], "-r") == 0) {
        // Run the Micron program from the /target/release/ folder
        char command[400];
        system("cargo build --release"); // Build the Micron program in release mode
        snprintf(command, sizeof(command), "cd target/release && micron.exe \"%s\"", abs_path);
        system(command);
    } else {
        // Run the Micron program from the /target/debug/ folder
        char command[400];
        system("cargo build"); // Build the Micron program in debug mode
        snprintf(command, sizeof(command), "cd target/debug && micron.exe \"%s\"", abs_path);
        system(command);
    }
    return 0;
}