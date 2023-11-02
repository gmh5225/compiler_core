%{
    /* definitions */
    #include <stdio.h>
%}

%union {
    int num;
    char sym;
}

%token EOL 
%token<num> INTEGER
%token<num> exp;

/* rules */

%%

input: 
    exp EOL {printf("%d\n", $1);}
    | EOL
    ;

exp: 
    INTEGER { $$ = $1;} 
    | exp PLUS exp { $$ = $1 + $3; }
    ;

%%

int main() {
    yyparse();

    return 0;
}