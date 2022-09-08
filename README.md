# Rustack
a stack based programming language written in rust

---

## Introduction

### Numbers

`1 2 3` will push the integer values 1, 2 and than 3 on top of the stack.

### Bodies

`(1 2) 3` will do the same as before, but noww `(1 2)` is seen as one operation.
This is useful for flow control and macros.

### Operations
```
+ :  addition
- :  subtraction
* :  multiplication
/ :  division (floored)
```

### Flow Control

```
if (...)                    If the top stack value isn't 0, the following operation will be executed.
                            Otherwise it'll be skipped.
if (...) else (...)         If the top stack value isn't 0, the following operation will be executed.
                            Otherwise the operation after 'else' will be executed.

while (...)                 While the top stack value isn't 0, the following operation will be
                            executed.

repeat (...)                Pops the top stack value and repeats following operation that many times.

```

### Variables and Macros
```
@(id)                       Pops the top value of the stack and registers it to the id's name.

{ (id) ... }                Maps the id's to the top of the stack, poping the values and registering
                            in the same order as the maps layout.

macro (id) (...)            Defines a macro with the id's name with the following operation.

macro [(int)] (id) (...)    Defines a macro with the id's name with the following operation,
                            and expecting the stack size to be at least int's size.


(id)                        If the id's name is a variable then it'll push its value to the stack,
                            if it's a macro then it'll execute that macro.

```

### Special Keywords

`print` takes the top value and prints it to the console (no new line).

`STACK` prints the current stack as a string (with new line).

`LENGTH` pushes the stacks size as an integer on top of the stack.
