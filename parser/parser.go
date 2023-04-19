package parser

import (
	"fmt"

	"github.com/ahmed-deftoner/monke-lang/ast"
	"github.com/ahmed-deftoner/monke-lang/lexer"
	"github.com/ahmed-deftoner/monke-lang/tokens"
)

type Parser struct {
    l *lexer.Lexer
   
    currToken tokens.Token
    peekToken tokens.Token
    errors []string
}

func New(l *lexer.Lexer) *Parser  {
    p := &Parser{
        l: l,
        errors: []string{},
    }
    p.NextToken()
    p.NextToken()
    return p
}

func (p *Parser) NextToken()  {
    p.currToken = p.peekToken
    p.peekToken = p.l.NextToken()
}

func (p *Parser) Errors() []string {
    return p.errors
}

func (p *Parser) peekError(t tokens.TokenType) {
    msg := fmt.Sprintf("expected next token to be %s, got %s instead",
    t, p.peekToken.Type)
    p.errors = append(p.errors, msg)
}

func (p *Parser) ParseProgram() *ast.Program {
    program := &ast.Program{}
    program.Statements = []ast.Statement{}

    for p.currToken.Type != tokens.EOF {
        stmt := p.ParseStatements()
        if stmt != nil {
            program.Statements = append(program.Statements, stmt)
        }
        p.NextToken()
    }

    return program
}

func (p *Parser) ParseStatements() ast.Statement {
    switch p.currToken.Type {
    case tokens.LET:
        return p.ParseLet()
    default:
        return nil
    }
}

func (p *Parser) ParseLet() *ast.LetStatement {
    stmt := ast.LetStatement{Token: p.currToken}

    if !p.expectPeek(tokens.IDENT) {
        return nil
    }

    stmt.Name = ast.Identifier{Token: p.currToken, Value: p.currToken.Literal}

    if !p.expectPeek(tokens.ASSIGN) {
        return nil
    }

    for !p.curTokenIs(tokens.SEMICOLON) {
        p.NextToken()
    }

    return &stmt
}

func (p *Parser) curTokenIs(t tokens.TokenType) bool {
    return p.currToken.Type == t
}

func (p *Parser) peekTokenIs(t tokens.TokenType) bool {
    return p.peekToken.Type == t
}

func (p *Parser) expectPeek(t tokens.TokenType) bool {
    if p.peekTokenIs(t) {
        p.NextToken()
        return true
    } else {
        return false
    }
}




