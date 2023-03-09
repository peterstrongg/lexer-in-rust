#include <iostream>
#include <vector>
#include <fstream>
#include <string>
#include "Scanner.h"

using namespace std;

ifstream open_file(string);

int main(int argc, char *argv[]) {
    if(argc > 2) {
        printf("Usage: %s <file>\n", argv[0]);
        exit(-1);
    } else if (argc < 2) {
        // Enter repl
        exit(1);    
    }

    ifstream file = open_file(argv[1]);

    Scanner a;
    return 0;
}

ifstream open_file(string file_name) {
    ifstream f;
    f.open(file_name);

    if(f.fail()) {
        printf("Error opening file\n");
        exit(-1);
    }

    return f;
}