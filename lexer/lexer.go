package lexer

import "github.com/ahmed-deftoner/monke-lang/tokens"

type Lexer struct {
    input string
    position int // current position in input (points to current char)
    readPosition int // current reading position in input (after current char)
    ch byte // current char under examination
}

func New(input string) *Lexer {
    l := &Lexer{input: input}
    l.readChar()
    return l
}

func (l *Lexer) readChar() {
    if l.readPosition >= len(l.input) {
        l.ch = 0
    } else {
        l.ch = l.input[l.readPosition]
    }
    l.position = l.readPosition
    l.readPosition += 1
}

func (l *Lexer) NextToken() tokens.Token {
    var tok tokens.Token
    l.skipWhitespace()

    switch l.ch {
    case '=':
        if l.peekChar() == '=' {
            ch := l.ch
            l.readChar()
            tok = tokens.Token{Type: tokens.EQ, Literal: string(ch) + string(l.ch)}
        } else {
            tok = newToken(tokens.ASSIGN, l.ch)
        }
    case ';':
        tok = newToken(tokens.SEMICOLON, l.ch)
    case '(':
        tok = newToken(tokens.LPAREN, l.ch)
    case ')':
        tok = newToken(tokens.RPAREN, l.ch)
    case ',':
        tok = newToken(tokens.COMMA, l.ch)
    case '+':
        tok = newToken(tokens.PLUS, l.ch)
    case '{':
        tok = newToken(tokens.LBRACE, l.ch)
    case '}':
        tok = newToken(tokens.RBRACE, l.ch)
    case '-':
        tok = newToken(tokens.MINUS, l.ch)
    case '!':
        if l.peekChar() == '=' {
            ch := l.ch
            l.readChar()
            tok = tokens.Token{Type: tokens.NOT_EQ, Literal: string(ch) + string(l.ch)}
        } else {
            tok = newToken(tokens.BANG, l.ch)
        }
    case '/':
        tok = newToken(tokens.SLASH, l.ch)
    case '*':
        tok = newToken(tokens.ASTERISK, l.ch)
    case '<':
        tok = newToken(tokens.LT, l.ch)
    case '>':
        tok = newToken(tokens.GT, l.ch)
    case 0:
        tok.Literal = ""
        tok.Type = tokens.EOF
    default:
        if isLetter(l.ch) {
            tok.Literal = l.readIdentifier()
            tok.Type = tokens.LookupIdent(tok.Literal)
            return tok
        } else if isDigit(l.ch) {
            tok.Type = tokens.INT
            tok.Literal = l.readNumber()
            return tok
        } else {
            tok = newToken(tokens.ILLEGAL, l.ch)
        }
    }
    l.readChar()
    return tok
}

func (l *Lexer) readNumber() string {
    position := l.position
    for isDigit(l.ch) {
        l.readChar()
    }
    return l.input[position:l.position]
}

func isDigit(ch byte) bool {
    return '0' <= ch && ch <= '9'
}


func newToken(tokenType tokens.TokenType, ch byte) tokens.Token {
    return tokens.Token{Type: tokenType, Literal: string(ch)}
}

func (l *Lexer) readIdentifier() string {
    position := l.position
    for isLetter(l.ch) {
        l.readChar()
    }
    return l.input[position:l.position]
}

func isLetter(ch byte) bool {
    return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

func (l *Lexer) skipWhitespace() {
    for l.ch == ' ' || l.ch == '\t' || l.ch == '\n' || l.ch == '\r' {
        l.readChar()
    }
}

func (l *Lexer) peekChar() byte {
    if l.readPosition >= len(l.input) {
        return 0
    } else {
        return l.input[l.readPosition]
    }
}
