#ifndef TOKEN_H
#define TOKEN_H

enum TokenValue {
    // Single char tokens
    LEFT_PAREN, RIGHT_PAREN, LEFT_CURLY, RIGHT_CURLY,
    DOT, COMMA, MINUS, PLUS, STAR, SEMICOLON, SLASH,

    // Literals
    NUMBER, STRING, IDENTIFIER,

    // Keywords
    AND, OR, IF, ELSE, WHILE, LET, CONST, TRUE, 
    FALSE, RETURN,
};

class Token {
    public:
        Token(TokenValue t, char* l);
    private:
        TokenValue token;
        std::string lexeme;
};

#endif