# bfrs
### A command-line [brainf***](https://esolangs.org/wiki/Brainfuck) interpreter and REPL.

## USAGE
### To Interpret a brainf*** File:
```brainfuck
//test.bf
>++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<+
+.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-
]<+.
```
```
$ bfrs test.bf
Hello, World!
```

To pass some input along with it:
```
$ bfrs test.bf INPUT
```
Example of program that intakes two letters and prints them:
```brainfuck
//test.bf
++[>,.<-]
```
```
$ bfrs test.bf HI
HI
```

### Using the REPL
```
$ bfrs
>> bfrs interpreter <<
>>> TYPE COMMANDS HERE
```

Unlike normal brainf***, alphanumerics aren't completely ignored here.
Instead, there's a set of commands for the REPL.
Anything not exactly matching these commands will be ignored like normal.

**OUT:** View the current ASCII output

**VIEW:** View the array you're working on, with the current cell highlighted green

Example (highlight not included here):
```
>>> +>++>+++
>>> view
[1][2][3][0][0][0][0][0][0][0]...
```
**INPUT:** Set the input string for the program

Example:
```
>>> input
ENTER INPUT STRING: HI
>>> ++[>,.<-]
>>> out
HI
```
**NEW:** Discard the current array/input and start fresh

**EXIT:** Exit the REPL
