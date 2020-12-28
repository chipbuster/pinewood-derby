Pinewood
========

This is the documentation for the toy language of this project, Pinewood.

### Core

Pinewood is like a very stripped down version of C99. The only numeric type is
the `int`. There are no floating point numbers and no other integral types.
Structs are supported, as are pointer types and `void`.

All arithmetic, and bitwise operations are supported on integers,
including `+`,`-`,`*`,`/`,`^`, `&`, `|`, and `~`. In addition, the logical
operators `!`, `&&`, and `||` have their usual meaning in C when applied to
integer or pointer types. All standard C comparison operators are supported.

Pinewood does support pointer types, as well as structs and the void type. It
does NOT support enums or unions. It also does NOT support type modifiers
(`const`, `volatile`) or storage class specifiers (`extern`, `static`,
`register`). It does NOT support the `typedef` keyword.

Pinewood supports a subset of C flow-control, including `if`-`else`, `while` and  
`for` loops, and `continue`/`break`/`return`. The constructs `do-while` and `goto`
are NOT supported.

Function declaration and definition in Pinewood works as it does in C 
(with the restrictions on types specified above). In addition, there are three
built-in types that do not exist in C: `lock`, `sema`, and `condvar`. Because
of their role in ensuring synchronization, they have a special role in the
language.

Finally, there is special syntax for defining external events: a named
external event can be triggered by using `<< name >>` syntax, while a named
external event can be waited for by using `[[ ]]`.

### Standard Library Declarations

Since Pinewood does not support 