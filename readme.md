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
- bubble_sort
- ansi_escape_codes
- rule110

## Language syntax
Arithmetic operations
```text
+ - * / %
```

Stack Operations
```text
rot3 dup drop over swap cast print nth
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
func <name> <types>... -> <types>... {

}
```

Variables
```
<type>... assign <name>... {

}
```

Records and Unions
```
record <name> {
    <type> <name>
    ...
}

<value...> <n> union
[<type>...]
```
