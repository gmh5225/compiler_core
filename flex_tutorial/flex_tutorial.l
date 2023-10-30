%{
    /* definitions */
%}

/*%option c++*/

/* constants */

DIGIT [0-9]
ID [a-z][a-z0-9]* 


/* rules */
%%
if|then|else {printf("Keyword\n");}
[a-z0-9.]{3,}@[a-z0-9.-]+ {printf("Email Address\n");}
[6-9][0-9]{9} {printf("Mobile number\n");}
[a-zA-Z_][a-zA-Z0-9_]* {printf("Valid identifier\n");}
. {printf("Invalid identifier\n");}
\n {return 0;}

%%

yywrap() {}

int main() {
    printf("Enter string: ");
    yylex();

    return 0;
}