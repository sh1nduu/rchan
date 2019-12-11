#!/bin/bash

try() {
  expected="$1"
  input="$2"

  ./target/debug/rchan "$input" > tmp.s
  gcc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    cat tmp.s
    exit 1
  fi
}

cargo build
if [ "$?" = "101" ]; then
  exit 1
fi

try 0 '0;'
try 42 '42;'
try 21 '5+20-4;'
try 156 '123-56+89;'
try 45 '1+2+3+4+5+6+7+8+9;'
try 15 '1+2-3+4-5+6-7+8+9;'
try 189 '123 - 23 + 89;'
try 2 '1 - 2 + 3;'
try 47 '5+6*7;'
try 15 '5*(9-6);'
try 4 '(3+5)/2;'
try 10 '-10+20;'
try 1 '2*-2+5;'
try 10 '+5++5;'
try 1 '1 == 1;'
try 0 '1 == 2;'
try 1 '1 != 2;'
try 0 '1 != 1;'
try 1 '2 >= 1;'
try 1 '1 >= 1;'
try 0 '1 >= 2;'
try 1 '1 > 0;'
try 0 '1 > 2;'
try 1 '1 < 2;'
try 0 '1 < 1;'
try 0 '2 <= 1;'
try 1 '1 <= 1;'
try 0 '1 <= 0;'
try 1 '12 + 13 <= 10 * 5;'
try 1 'a=1; a;'
try 1 'a=1; b=2; b-a;'
try 16 'a=8; b=2; a*b;'
try 65 'a=8; a=a*a; a+1;'
try 4 'abc=8; def=abc*2; def/4;'
try 14 'a = 3;
b = 5 * 6 - 8;
a + b / 2;'
echo OK