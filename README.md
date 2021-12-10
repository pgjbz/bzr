# Bzr lang

A simple interpreted lang based on [Monkey Lang](https://monkeylang.org/) implementation, this lang is make only for learn more about Rust and interpreters and improve my english.

### Rust version to write this code:

Required Rust Nightly minimal 1.58.0

<hr>

### Types

For now only have 3 types:

- Boolean => `bool`
- Integer 64bit => `int`
- String => `str`

<hr>

### Examples

## Hello world

```bzr
puts("Hello World!");
```

## If

```bzr
if 34 + 35 == 68 {
    puts("Ok");
} else {
    puts("Not ok");
}
```

## If else if

```bzr
if 34 + 35 == 68 {
    puts("Ok");
} else if 34 + 35 == 69{
    puts("very ok ok");
} else {
    puts("not ok");
}
```

## Function

```bzr
fn add(a int, b int) int {
    ret a + b;
}
```

## let

```bzr
let a int = 10;
puts("a = ", a);
```

But for store function return cannot inform type

```bzr
fn factorial(x int) int {
    if x <= 1 {
        ret 1;
    } else {
        ret x * factorial(x - 1);
    }
}

let fact = factorial(4);
puts("Fact = ", fact);
```

## while

```bzr
let i int = 0;
while i < 10 {
    putsln("i = ", i);
    i = i + 1;
}
```

## Replace

For now only suports array

replace(array, pos, new_value);

## Len

Return the length of array or string

len("banana") returns 6

## Slice

Create a slice of array or string

slice(array/string, start, end);

## read

Read input for user

## Append

Append new item in final array or final string

```bzr
let arr array = [1, 2, 3];
append(arr, 20);
putsln(arr);
```

Display: [1, 2, 3, 20];
## is_erro

Check if is error

```bzr
let a = to_int("aaa");
if is_error(a) {
    putsln("Error");
}
```

## to_int and to_str

Parse to int or str

## trim

Remove spaces in string

```bzr
let a = "  bzr   ";
let b = trim(a);
putsln(b);
```
Display: bzr
## print

To print have `puts`, `putsln`, `eputs` and `eputsln`

- puts = print in same line
- putsln = print and put new line
- eputs = print in stardard error
- eputsln = print in stardard error and put new line

## Operators:

Name | Symbol | Example | Return |
|----|--------|---------|--------|
| plus |   +   |  1 + 1  |    2   |
| minus |   -   |  1 - 1  |    0   |
| multiply |   *   |  2 * 2  |    4   |
| divide |   /   |  2 / 2  |    1   |
| and |   &   |  7 & 5  |    5   |
| or |   \|   |  7 | 5  |    7   |
| xor |   ^   |  7 ^ 5  |    2   |
| shift left |  <<   |  1 << 2  |    4   |
| shift rigt |  >>   |  4 >> 1  |    2   |
| equals |   ==   |  5 == 5  |    true   |
| not equals |   =!   |  5 =! 5  |    false   |
| greater than |   >   |  5 > 2  |    true   |
| less than |   <   |  5 < 2  |    false   |
| greater than equals |   >=   |  5 >= 2  |    true   |
| less than equals |   <   |  2 <= 2  |    true   |
| and |   &&   |  5 > 2 && true  |    true   |
| or |   \|\|   |  5 < 2 \|\| true  |    true   |


# Build

To build this lang clone this repo with

```git clone url.git``` 

And enter the directory with `cd bzr` and enter command ```cargo build --release`

## Use

Execute the bin with

./target/release/bzr filename.bzr