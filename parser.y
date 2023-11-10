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
%type<num> exp
%token PLUS

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

int yyerror(char* s) {
    printf("ERROR: %s\n", s);
    return 0;
}