parser grammar GpslParser;
options { tokenVocab = GpslLexer; }

gpslFile: function* EOF ;

function: attribute? FN IDENT LPAREN (IDENT COLON type COMMA?)* RPAREN (ARROW type)? block ;

program: stmt* ;

stmt: let
    | block
    | return
    | if
    | while
    | for
    | expr SEMICOLON
    ;

let: LET IDENT ((COLON type (EQ expr)?) | EQ expr) SEMICOLON ;
block: permission? LCURL stmt* RCURL ;
return: RETURN expr? SEMICOLON ;
if: IF expr stmt (ELSE stmt)? ;
while: WHILE expr stmt ;
for: FOR LPAREN expr? SEMICOLON expr? SEMICOLON expr? RPAREN stmt ;

type: IDENT (LT (type COMMA?)+ BT)? ;
attribute: SHARP LBRACKET IDENT (assign COMMA?)* RBRACKET ; 
mode: SHARP IDENT ;
permission: DOLLER LPAREN ( IDENT LBRACKET ( IDENT COMMA? )* RBRACKET COMMA? )* RPAREN ;

expr: assign ;
assign: equality (EQ assign)? ;
equality: relational (EQEQ relational | NE relational | CONJ)* ;
relational: add (LE add | LT add | BE add | BT add)* ;
add: mul (ADD mul | SUB mul | SUB_ASSIGNMENT mul | ADD_ASSIGNMENT mul)* ;
mul: unary (MUL unary | DIV unary | DIV_ASSIGNMENT unary | MUL_ASSIGNMENT unary)* ;

primary: LPAREN expr RPAREN | function_call | TEXT (LBRACKET stmt RBRACKET)? | NUM ;
function_call: IDENT LPAREN (stmt COMMA?)* RPAREN ;

unary: ADD primary
    | SUB primary
    | primary
    ;
