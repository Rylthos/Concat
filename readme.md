# Concat
Stack based programming language

## Goals
- Compile to assembly
- Implement a type system to stack shape and safety
- Bootstrap the compiler

## Language Features
- Basic arithmetic and control flow

## Examples
- fibonacci
- fizzbuzz
- fizzbuzzfuzz

## Language syntax
Arithmetic operations
```text
+ - * / %
```

Stack Operations
```text
rot3 dup drop over swap cast print
```

Boolean Operations
```text
< > <= >= = != && || !
```

Conditionals
```text
if else
```

Loops
```text
while
```

Functions
```
func <name> [<types>] -> [<types>] {

}
```
