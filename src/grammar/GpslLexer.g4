lexer grammar GpslLexer;

WS
    : [ \t\r\n]
      -> skip
    ;

ADD: '+' ;
SUB: '-' ;
MUL: '*' ;
DIV: '/' ;
CONJ: '&&' ;
AND: '&' ;
EQ: '=' ;
EQEQ: '==' ;
NE: '!=' ;
BE: '>=' ;
LE: '<=' ;
BT: '>' ;
LT: '<' ;
SEMICOLON: ';' ;
COLON: ':' ;
COMMA: ',' ;
DOT: '.' ;
QUOTE: '"' ;
ADD_ASSIGNMENT: '+=' ;
SUB_ASSIGNMENT: '-=' ;
MUL_ASSIGNMENT: '*=' ;
DIV_ASSIGNMENT: '/=' ;
LPAREN: '(' ;
RPAREN: ')' ;
LCURL: '{' ;
RCURL: '}' ;
ARROW: '->' ;

FN: 'fn' ;
FOR: 'for' ;
WHILE: 'while' ;
IF: 'if' ;
ELSE: 'else' ;
LET: 'let' ;
RETURN: 'return' ;

NUM: [1-9] [0-9]* ;

TEXT: QUOTE [a-zA-Z0-9_-]* QUOTE ;
IDENT: [a-zA-Z_]+ ;