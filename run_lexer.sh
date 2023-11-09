#!/bin/bash

flex lexer.l

gcc lex.yy.c -o output

./output