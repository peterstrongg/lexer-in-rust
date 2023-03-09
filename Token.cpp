#include <string>
#include "Token.h"

Token::Token(TokenValue t, std::string l, int ln) {
    token = t;
    lexeme = l;
    line = ln;
}