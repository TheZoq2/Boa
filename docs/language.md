Goals
=====

 * Type safety
 * Ease of use


Syntax
======
##Basics
Comments are started using the `#` sign. Block comments can be done using
`#>` and `#<`

Lines (or sentences) end with `;` unless they end with a block like `{}` or `[]`

Variables are defined using `let <Type> <name> = <value>`. If the type of the value can be 
guessed, the type can be omitted

A value can be assigned to a variable using `name = value` 

##Operators
Basic operators include:
 * Arithmetic operators `+` `-` `*` `/` `^`
 * Comparison operators `<` `<=` `>=` `>` `==` `!=`
 * Logic operators `&` `|` `!`

Operators are overloadable

##Functions
Functions are defined as `def [return-type] <name>(<type1> <var1>...) {function code}`
Values are returned using the `return keyword`. Types must be specified but the return type
can be omitted if the function does not return

##Conditionals and loops
A condition is a boolean value. The condition operator can also be overloaded for types to make things like
```
List<sometype> a;
if a{} #if a is not empty
```

If statements are started using the `if` keyword. Brackets are optional. The chain can continue using
`else` or `elseif`.

While loops are defined using the `while` keyword.

For loops can either be simple fori loops or foreach loops. Fori loops look like this
```
for <var> is <start> to <end> step [amount to step] {}
```
while foreach loops look like this:
```
foreach <var> in <container> {}
```
The type of var can be omitted because it is known from the container
