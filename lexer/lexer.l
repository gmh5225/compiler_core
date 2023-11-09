%{
    /* definitions, header files, macros etc. */
    #include <stdio.h>
    #include <stdlib.h>
    int yylex (void);
    void yyerror (char const *);
    #define SAVE_TOKEN yylval.string = new std::string(yytext, yyleng)
    #define TOKEN(t) (yylval.token = t)
%}

%option c++

/* constants */

ID [a-zA-Z_][a-zA-Z0-9_]* 
DOUBLE [0-9]+.[0-9]*
INTEGER [0-9]+

/* rules */
%%
case {printf("case keyword");}
char {printf("char keyword");}
const {printf("const keyword");}
continue {printf("continue keyword");}
double {printf("double keyword");}
else {printf("else keyword");}
else\ if {printf("else if keyword");}
enum {printf("enum keyword");}
extern {printf("extern keyword");}
float {printf("float keyword");}
for {printf("for keyword");}
if {printf("if keyword");}
int {printf("int keyword");}
long {printf("long keyword");}
return {printf("return keyword");}
short {printf("short keyword");}
signed {printf("signed keyword");}
sizeof {printf("sizeof keyword");}
static {printf("static keyword");}
struct {printf("struct keyword");}
switch {printf("switch keyword");}
void {printf("void keyword");}
while {printf("while keyword");}

"=" {printf("equal");}
"==" {printf("double equal");}
"!=" {printf("does not equal");}
"<" {printf("less than");}
"<=" {printf("less than or equal to");}
">" {printf("greater than");}
">=" {printf("greater than or equal to");}
"(" {printf("left paren");}
")" {printf("right paren");}
"{" {printf("left bracket");}
"}" {printf("right bracket");}
"." {printf("dot");}
"," {printf("comma");}
"+" {printf("plus");}
"-" {printf("minus");}
"*" {printf("mult");}
"/" {printf("divisor");}

{ID} {printf("identifier");}
{DOUBLE} {printf("double");}
{INTEGER} {yylval.num = atoi(yytext); return INTEGER;}

\n {return EOL;}
. {printf("Invalid token"); yyterminate();}

%%

yywrap() {}