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
    exit 1
  fi
}

cargo build

try 0 0
try 42 42
try 21 '5+20-4'
try 156 '123-56+89'
try 45 '1+2+3+4+5+6+7+8+9'
try 15 '1+2-3+4-5+6-7+8+9'
try 189 '123 - 23 + 89'
try 2 '1 - 2 + 3'
echo OK