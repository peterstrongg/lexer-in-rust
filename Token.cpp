#include <iostream>
#include "Token.h"

using namespace std;

Token::Token(TokenValue t, char* l) {
    token = t;
    lexeme = l;
}