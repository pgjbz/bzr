# Bzr lang

A simple interpreted lang based on [Monkey Lang](https://monkeylang.org/) implementation

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