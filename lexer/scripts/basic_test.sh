#!/bin/bash

# Step 1: Compile the lexer
flex ../lexer.l
gcc lex.yy.c -o output

# Step 2: A function to test each keyword
PROMPT="Enter string: "
test_keyword() {
    keyword=$1
    expected_output="${PROMPT}$2"
    actual_output=$(echo $keyword | ./output)
    if [[ "$actual_output" == "$expected_output" ]]; then
        echo "Test for '$keyword' PASSED"
    else
        echo "Test for '$keyword' FAILED. Expected: '$expected_output', Got: '$actual_output'"
    fi
}

# Step 3: Call the test function for each keyword
test_keyword "case" "case keyword"
test_keyword "char" "char keyword"
test_keyword "const" "const keyword"
test_keyword "continue" "continue keyword"
test_keyword "double" "double keyword"
test_keyword "else" "else keyword"
test_keyword "else if" "else if keyword"
test_keyword "enum" "enum keyword"
test_keyword "extern" "extern keyword"
test_keyword "float" "float keyword"
test_keyword "for" "for keyword"
test_keyword "if" "if keyword" 
test_keyword "int" "int keyword"
test_keyword "long" "long keyword"
test_keyword "return" "return keyword"
test_keyword "short" "short keyword"
test_keyword "signed" "signed keyword"
test_keyword "sizeof" "sizeof keyword"
test_keyword "static" "static keyword"
test_keyword "struct" "struct keyword"
test_keyword "switch" "switch keyword"
test_keyword "void" "void keyword"
test_keyword "while" "while keyword"

rm lex.yy.c output
