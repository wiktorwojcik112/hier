# About Hier Language
Hier is my second attempt at making a programming language (previous was GoldByte, but it had bad architecture, so it was easier to start from the  beginning). This time I decided to do actual research (Thanks for Crafting Interpreters). It runs code by interpreting AST, so it is not very fast, but it currently works and writing a bytecode VM is a possibility. There are issues with it, and it is not perfect, but with time it will improve.

# Usage
Hier is written in Rust, so you will need to install its toolchain, if you don't have it. Go to www.rust-lang.org/learn/get-started for help.
Clone this repo to your machine:
```
git clone https://github.com/wiktorwojcik112/hier.git
```
Enter hier directory and run this to build an executable:
```
cargo build --release
```
The executable will appear in target/release. Go into this directory to run it.
You can run REPL by entering:
```
./hier repl
```
You can run a Hier code from command line and print its result using:
```
./hier run "(print 123)"
```
To run a file, enter:
```
./hier file some_hier_code.hier
```

# Design
Hier uses a Lisp-like syntax of putting everything in brackets - ( and ). Hier contains only 5 constructs: lists (using ( and ) ), blocks (using { and }), numbers (number with optional ., for example: 1.0, 2.5, -1.5, 5, -3), strings (using " and ", they can contain all characters except ", because there is no interpolation and they can be multiline (new lines are included in string)), subscripts (expression with [ ]), properties (expression with . and its property (including functions)), identifiers (any characters, except it can't begin with " and must not contain spaces, :, (, ), ., new lines, [ and ]) and directives (which begin with #).

The language is functional (no classes) and everything in it (except directives) is some kind an expression. Here is an example of a program which adds numbers 1 2 3 and result of subtracting 2 from 1 and prints it:

```
#main
(print (+ 1 2 3 (- 1 2)))
```

Almost all of Hier is value-based. That means that operation creates a copy of a value. For example, by using insert, remove or replace functions on an array, you don't change the original array, but create a new array with specified changes.


# Directives
Directives begin with #. There are 2 types of directives - module name and include. 
- Module name is provided using #some_module_name syntax. Module name is the name of current file (by default, its filename). It is used to distinguish between modules in inclusion.
- Include is provided using #<some_path.hier> syntax. The path can be relative to current directory. When used, Hier will literally include tokens from the provided file in the current file at the tokenization stage. If file was already included, it is omitted. It is determined based on module name of the file. This is an initial way for Hier to support multiple files, which may change in the future.

# Lists
Lists are fundamental element of Hier. They are made of expressions between ( and ). The list may be a function call depending on circumstances. If first expression is an identifier, it will work like a function call, for example (print 1 2 3). If first expression is a property it will work like a syntactic sugar for function call on object, for example, it will convert (array.insert 1) to (insert array 1), allowing clearer syntax. If first expression evaluates to function arguments (for example: (| a b c)), list will evaluate to an anonymous function. Otherwise, it will generate an array (in Hier, arrays are collective name for lists and arrays). Lists (arrays) can also be created using (& value1 value2) function call or (list value1 value2) function call.

An example:
```
(print 1)
(@array (1 2 3))
(print array[0])
(@function ((| a b c) { (print a) }))
(@array)
(print (array.length))
```
# Table
Tables are created using (# value1 value2) or (table value1 value2) function calls. Tables contain many key-values (all arguments to calls must be key-values (look: key-value section)). Fields of table can be accessed using property and specifying key of a key-value pair. Here is an example of the usage:
```
(@some (# key("value")))
(print some.key)
```

At this moment, adding fields is unsupported.

# Blocks
Blocks are made of expressions between curly brackets - { and }. The difference between them and lists is that blocks do not evaluate immediately. This means, that to run a block, you will need to put it as a expression for a function, that executes blocks, for example if, run or while. Blocks also have this property, that they evaluate to the last expression in it, so { (+ 1 2) (+ 5 6) } will evaluate to a value 11. All blocks always create new scopes.

# Properties
Properties can mean either "methods" or a property of a structure. If it appears at the beginning of a list, it will convert to a function with object as the first argument (look: lists). If it appears later, it will convert to a get function call, for example, (print a.b) will convert to (print (get a b)), which will get property b from object a.

# Variables
In Hier, you declare a variable using (@variable_name value_expression) syntax. Two variables with the same name can't exist in the same scope (block). You can get value of the variable by just using its identifier in a list further than first argument, for example, (print variable_name). If variable doesn't exist, Hier returns null. You can use (=name value) to assign a new value to variable.

# Subscripts
Subscript is made by appending [value] to an expression. The value can be an identifier, a number, a block, a list and a string. Subscripts are converted like properties to a get function call, for example, (print array[0]) gets converted to (print (get array 0)).

# Key-value
Key-value is created using identifier(expression) syntax. You can access key using its key property and value using its value property, like this:
(@some key("value"))
(print some.key some.value)

# Identifiers
Some identifiers get converted into values, like true, false and null. All other refer to variables.

# Operators
Operators are just functions called like other functions. There are operators for addition (+; also acts as a string concatenation operation), subtraction (-), multiplication (*), division (/), modulo (%; because all numbers are floats, it rounds all numbers down and then performs modulo), logical negation (!; the only operator that only accepts one argument) logical and (&&),logical or (||), null-coalescing (??; if left is null, returns right, and if left isn't null, return left), (non-)equality (!= and ==) and comparison (<, >, <= and >=). +, -, *, /, && and || accept many arguments. 

# Functions
Functions are declared using (@function_name (| first_argument second_argument) { (print first_argument) }) syntax. Function | returns function arguments - a special value that just contains identifiers that are passed as arguments. The block is the code that will get executed when function is called. You call such function using normal syntax: (function_name 1 2). Hier checks arity (number of arguments) of functions and errors when it doesn't match. 

# Control flow
Hier has 6 control flow functions - try, run, if, while, repeat, break and for.

## Run
(run expression...)
Run evaluates all (run accepts any number of arguments) of its arguments (including execution of passed blocks) and returns value of the last expression, so this code would return 2:
(run { (print "test") } 2)

## If
(if condition block block?)
If executes first block when condition is true and returns the result of its execution (the last expression). If there is a second block, it gets executed when condition is false and its value gets returned.

## While
(while block block)
While first executes first block in the new scope. Then it checks the value of the second block (must evaluate to a bool) and if its true, then it executes third block. Then it repeats check and execution as long as check evaluates to true. Th evalue of while evaluation is null, but it may be changed to the value of the last iteration (same for for and repeat loops).

## Try
(try expression block)
Try checks if expression returns error (you can make an error using error function which accepts string as the only argument) and runs the block if it is an error. It returns value of the expression if it's not an error, and value of the block, if expression evaluates to an error.

## Repeat
(repeat number? block)
Repeat runs the block specified number of times. If there is only a block, it runs the block infinitely. 

## Break
(break)
Break creates a "LoopExit" error which exits currently running loop.

## For
(for array block)
For runs the block for every element of a array (must be a list or a string). In every iteration the current element is passed as a variable named "element".

# Other functions
Hier contains many functions like print (print all values), println (print all values and a new line at the end), cmd (run a shell command), eval (evaluate Hier code string), string and number conversion, operations on arrays (insert, remove, length and replace) and a few more. You can find all of them in native_functions.rs file (they will be split to seperate files in the future). All of the functions will soon be documented.


