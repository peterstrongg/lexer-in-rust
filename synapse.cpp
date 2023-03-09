#include <iostream>
#include <vector>
#include <fstream>
#include <string>
#include "Scanner.h"

std::string read_file(std::string file_name);

int main(int argc, char *argv[]) {
    if(argc > 2) {
        printf("Usage: %s <file>\n", argv[0]);
        exit(-1);
    } else if (argc < 2) {
        // Enter repl
        exit(1);    
    }
    
    Scanner scanner(read_file(argv[1]));

    return 0;
}

std::string read_file(std::string file_name) {
    char ch; std::string src;
    
    std::ifstream file(file_name, std::ios::in);

    if(file.fail()) {
        printf("Error opening file\n");
        exit(-1);
    }

    while(file) {
        if(file.get(ch)) {
            src.push_back(ch);
        }
    }

    file.close();

    return src;
}