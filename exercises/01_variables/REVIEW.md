# REVIEW

## are variables are inmutable by default, excepting when...
- we use mut keyword at declaration.
- CONSTANTS can not use this.

## variables must be initialized !
When you declare a variable in Rust, the compiler expects you to assign a value to it. If you don't assign a value to a variable, the compiler will raise an error because it cannot determine the initial value of the variable.

Rust is a statically-typed language, which means that every variable must have a known type at compile time. When you declare a variable without assigning a value, the compiler cannot infer the type of the variable. This can lead to potential issues, such as uninitialized memory being used or incorrect assumptions about the variable's type.

## Typing variables
- there are 2 ways of typing


i32 = (4 x 8) = 32. This equals 4 bytes.