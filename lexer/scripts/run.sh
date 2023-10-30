#!/bin/bash

flex ../lexer.lex

gcc lex.yy.c -o output

./output