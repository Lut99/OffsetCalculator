# OffsetCalculator
A small tool that is meant to be the easiest way to calculate with (unsigned) hexadecimal values. Mostly used when, idk, trying to figure out what it says in a raw binary file, and you have these offsets in hexadecimal and want to compute between them.

## Compilation
The project is managed using Rust's Cargo, so download that first (the easiest way is with [rustup](https://rustup.io)).

Once you have that, navigate to the project's source directory and build the package and dependencies with:
```
cargo build --release
```

## Running
Once compiled, a binary can be found under `target/release/offsetcalculator`. Its main purpose is to provide a place to perform calculations by making use of a _read-eval-print loop_ (REPL). This is what is entered when the binary is run normally, and where expressions can be entered line-by-line to be computed.

### Expressions
Expressions in the OffsetCalculator are build up as a combination of _operators_ and _values_. When entered and a newline is given, the calculator parses the expression and tries to evaluate it. When succesful, the result is printed to stdout.

For example, the most simple expression in the calculator (just a decimal value) would evaluate to:
```
 > 42
 = 42
```

### Values
The calculator supports multiple value 'types'; all of them evaluate to an unsigned 64-bit integer, but each of them has a different representation. This matters because the calculator tries to return the same representation to the user as is given, and knowledge of the representations can thus be used to convert between them.

The three representations supported are:
 - Decimal numbers, noted either as a number (e.g., `42`) or a number prefixed with `0d` (e.g., `0d42`).
 - Hexadecimal numbers, noted down as a hexadecimal number prefixed with `0x` (e.g., `0x2A`).
 - Binary numbers, noted down as a binary string prefixed with `0b` (e.g., `0b101010`).

### Operations
There are several types of operations that are supported by the calculator. Note that every operation works on expressions, and can thus be combined to create complex expressions.

---------
#### _Arithmetic operations_
The first type are arithmetic operations. These are:
 - Addition, noted with `+`.
 - Subtraction, noted with `-`.
 - Multiplication, noted with `*`.
 - (Integer) Division, noted with `/`.

All of them are left-associative, and are infix between two expressions. For example, the following happens when we input `42 - 42 + 42` in the calculator:
```
 > 42 - 42 + 42
 = 42
```
Meaning it first computed `42 - 42`, and then `0 + 42`.

Also note that multiplication and division have a higher precedence than addition and subtraction; consider the following evaluation:
```
 > 42 + 42 * 42
 = 1806
```
Note, however, that both associativity and precedence can be overwritten by using brackets (`()`):
```
 > (42 + 42) * 42
 = 3528
```

Since the calculator tries to deduce the output format from the given formats, its important to note that for all arithmetic expressions, the _leftmost_ format is taken when they differ (unless a conversion operation is used; see the [next](#Conversion-operations) section).

---------
#### _Conversion operations_
The second type of operations are format conversion operators. These can convert one representation into another. There are three of them:
 - `dec`: Converts the given expression to a decimal representation.
 - `hex`: Converts the given expression to a hexadecimal representation.
 - `bin`: Converts the given expression to a binary representation.

Note that each of them takes special precedence when an artihmetic operation has to decide which representation to use; if one side has a conversion operation along the way, that side is always preferred during representation evaluation.
For example:
```
 > 42 + 0x2A
 = 84

 > 42 + hex 42
 = 0x54
```

### Variables
To make usage of the calculator a lot easier, it also supports the use of variables.

To declare a variable, use the special _assign-operator_, after which it can be used in place of a normal value:
```
 > ex = 42
 = 42

 > ex
 = 42
```
Note that assignments are considered expressions too, and thus return the value that they assigned. This allows for a few more interesting expressions:
```
 > ex2 = (42 + ex1 = 42)
 = 84

 > ex1
 = 42

 > ex2
 = 84
```
Finally, also note that variables take on the representation used by their value:
```
 > ex = 0x2A
 = 0x2A

 > ex + 42
 = 0x54
```

---------
#### _ans_
From the start, a special variable `ans` is defined:
```
 > ans
1: Identifier 'ans' is defined, but not initialized yet.
```
It begins as unitialized, but is automatically updated with the result of the last expression every time one is run:
```
 > 42
 = 42

 > ans
 = 42
```
This allows for workflows where a value has to be updated all the time and the intermediate results printed:
```
 > 42
 = 42

 > ans + 42
 = 84

 > ans + 42
 = 126
```

### Precedence
With all the operators and values explained, we can now put their precedence in a table:
| Precedence level | Operator | Description                                          | Associativity |
|------------------|----------|------------------------------------------------------|---------------|
|1                 | =        | Assignment of a variable.                            | Right-to-left |
|2                 | dec      | Converts the given expression to decimal format.     | Right-to-left |
|2                 | hex      | Converts the given expression to hexadecimal format. | Right-to-left |
|2                 | bin      | Converts the given expression to binary format.      | Right-to-left |
|3                 | *        | Multiplication.                                      | Left-to-right |
|3                 | /        | Division.                                            | Left-to-right |
|4                 | +        | Addition.                                            | Left-to-right |
|4                 | -        | Subtraction.                                         | Left-to-right |

### Commands
Finally, instead of giving an expression, a few special commands can be given as well:
 - `del <id>`: Deletes the variable with the given identifier.
 - `delall`: Deletes all variables, even 'ans' (resetting it to undefined).
 - `show_vars`: Shows a list of currently loaded variables and their values.
 - `help`: Shows an in-calculator help menu for expressions and commands.
 - `exit`: Exits the REPL.
Note that for obvious reasons, expressions and commands cannot be mixed.

## Command line arguments
This binary takes a few command line arguments:
 * `-e,--execute <expression>`: If given, executes the given expression and then quits. Note that this returns its value as simple a number, hex or binary without any formatting to aid calling it from scripts or other executables.
 * `-s,--session <path>`: If given, stores this session in the given so you can resume later on. If it already exists, loads that session and continues from there. Note that, if present, the OffsetCalculator always tries to load './offsetcalculator.session' if it exists.
 * `-S,--no-session`: If given, does not the './offsetcalculator.session' file in the current directory if it exists.
 * `-h,--help`: Shows this list of arguments and then quits.

## Issues
If you have suggestions, want to see something changed or encounter a bug, feel free to make a new issue on our [issues](https://github.com/Lut99/OffsetCalculator/issues) page. Try to give it the appropriate tags.

Also note this project is a hobby project of mine, so response times to issues may vary.

## License
This project is licenced under the GNU General Public License v3.
