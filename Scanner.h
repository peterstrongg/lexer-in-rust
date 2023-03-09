#include <string>
#include <vector>
#include "Token.h"

#ifndef SCANNER_H
#define SCANNER_H

class Scanner {
    public:
        Scanner(std::string src);
        void scan_for_tokens();
    
    private:
        std::string source;
        std::vector<Token> tokens;
};

#endif