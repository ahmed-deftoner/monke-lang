package parser

import (
	"github.com/ahmed-deftoner/monke-lang/ast"
	"github.com/ahmed-deftoner/monke-lang/lexer"
	"github.com/ahmed-deftoner/monke-lang/tokens"
)

type Parser struct {
    l *lexer.Lexer
    
    currToken tokens.Token
    peekToken tokens.Token
}

func New(l *lexer.Lexer) *Parser  {
    p := &Parser{l: l}
    p.NextToken()
    p.NextToken()
    return p
}

func (p *Parser) NextToken()  {
    p.currToken = p.peekToken
    p.peekToken = p.l.NextToken()
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
