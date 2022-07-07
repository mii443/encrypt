parser grammar GpslParser;
options { tokenVocab = GpslLexer; }

gpslFile: function* EOF ;

function: FN IDENT LPAREN (IDENT COLON IDENT COMMA?)* RPAREN (ARROW IDENT)? block ;

program: stmt* ;

stmt: let
    | block
    | return
    | if
    | while
    | for
    | expr SEMICOLON
    ;

let: LET IDENT COLON IDENT SEMICOLON ;
block: permission? LCURL stmt* RCURL ;
return: RETURN expr? SEMICOLON ;
if: IF LPAREN expr RPAREN stmt (ELSE stmt)? ;
while: WHILE LPAREN expr RPAREN stmt ;
for: FOR LPAREN expr? SEMICOLON expr? SEMICOLON expr? RPAREN stmt ;

permission: DOLLER LPAREN ( IDENT LBRACKET ( IDENT COMMA? )* RBRACKET COMMA? )* RPAREN ;

expr: assign ;
assign: equality (EQ assign)? ;
equality: relational (EQEQ relational | NE relational | CONJ)* ;
relational: add (LE add | LT add | BE add | BT add)* ;
add: mul (ADD mul | SUB mul | SUB_ASSIGNMENT mul | ADD_ASSIGNMENT mul)* ;
mul: unary (MUL unary | DIV unary | DIV_ASSIGNMENT unary | MUL_ASSIGNMENT unary)* ;

primary: LPAREN expr RPAREN | function_call | TEXT | NUM ;
function_call: IDENT LPAREN (unary COMMA?)* RPAREN ;

unary: ADD primary
    | SUB primary
    | primary
    ;
