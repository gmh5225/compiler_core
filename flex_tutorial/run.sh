#!/bin/bash

flex flex_tutorial.l

gcc lex.yy.c -o output

./output