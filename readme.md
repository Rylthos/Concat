# Concat
Stack based programming language

## Goals
- Compile to assembly
- Implement a type system to stack shape and safety
- Bootstrap the compiler

## Language Features
- Basic arithmetic and control flow
- Functions
- Variables
- Simple Defines
- Unions
- Linux syscalls

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

Binary operations
```text
& |
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
if <cond> {} else if <cond> {} else {}
```

Loops
```text
while <cond> { }
```

Functions
```text
func <name> <types>... -> <types>... { }
```

Variables
```text
<type>... assign <name>... { }
```

Records and Unions
```text
record <name> {
    <type> <name>
    ...
}

<value...> <n> union
[<type>...]
```

Defines
```text
<value> <name> define
```

Includes
```text
"<path>" include
"std:<path>" include
```

Syscalls
```text
<args...> <num_args> <syscall> syscall
```
