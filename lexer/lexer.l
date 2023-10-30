%{
    /* definitions */
%}

/*%option c++*/

/* constants */

ID [a-zA-Z_][a-zA-Z0-9_]* 

/* rules (alphabetical) */
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

{ID} {printf("Identifier\n");}
. {printf("Invalid token\n");}
\n {return 0;}

%%

yywrap() {}

int main() {
    printf("Enter string: ");
    yylex();

    return 0;
}