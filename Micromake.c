#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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
        printf("Usage: micron <program>\n");
        return 1;
    }

    // Check if the user provided the -r flag
    if (argc == 3 && strcmp(argv[1], "-r") == 0) {
        // Run the Micron program from the /target/release/ folder
        char command[100] = "cd target/release && ./";
        system("cargo build --release"); // Build the Micron program in release mode
        strcat(command, argv[2]);
        system(command);
    } else {
        // Run the Micron program from the /target/debug/ folder
        char command[100] = "cd target/debug && ./";
        system("cargo build"); // Build the Micron program in debug mode
        strcat(command, argv[1]);
        system(command);
    }
}
