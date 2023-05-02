package ast

import (
	"bytes"

	"github.com/ahmed-deftoner/monke-lang/tokens"
)

type Node interface {
    TokenLiteral() string
    String() string
}

type Statement interface {
    Node
    statementNode()
}

type Expression interface {
    Node
    expressionNode()
}

type Program struct {
    Statements []Statement
}

func (p *Program) TokenLiteral() string  {
   if len(p.Statements) > 0 {
    return p.Statements[0].TokenLiteral()
   } 
   return ""
}

func (p *Program) String() string {
    var out bytes.Buffer
    for _, s := range p.Statements {
        out.WriteString(s.String())
    }
    return out.String()
}

type LetStatement struct {
    Token tokens.Token
    Name Identifier
    Value Expression
}

func (ls *LetStatement) TokenLiteral() string {
    return ls.Token.Literal
}

func (ls *LetStatement) String() string {
    var out bytes.Buffer
    out.WriteString(ls.TokenLiteral() + " ")
    out.WriteString(ls.Name.String())
    out.WriteString(" = ")
    if ls.Value != nil {
        out.WriteString(ls.Value.String())
    }
    out.WriteString(";")
    return out.String()
}


func (ls *LetStatement) statementNode() {} 

type Identifier struct {
    Token tokens.Token
    Value string
}

func (i *Identifier) TokenLiteral() string {
    return i.Token.Literal
}

func (i *Identifier) statementNode() {} 

func (i *Identifier) String() string { return i.Value }

type ReturnStatement struct {
    Token tokens.Token
    ReturnVal Expression
}

func (rs *ReturnStatement) statementNode() {}

func (rs *ReturnStatement) TokenLiteral() string {
    return rs.Token.Literal
}

func (rs *ReturnStatement) String() string {
    var out bytes.Buffer
    out.WriteString(rs.TokenLiteral() + " ")
    if rs.ReturnVal != nil {
        out.WriteString(rs.ReturnVal.String())
    }
    out.WriteString(";")
    return out.String()
}

type ExpressionStatement struct {
    Token tokens.Token // the first token of the expression
    Expression Expression
}

func (es *ExpressionStatement) statementNode() {}

func (es *ExpressionStatement) TokenLiteral() string { return es.Token.Literal }

