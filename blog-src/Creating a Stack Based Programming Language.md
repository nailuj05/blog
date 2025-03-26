# Creating a Stack Based Programming Language

I always wanted to get into compiler development and building/designing my own programming language. But before diving head first into a new project, that I probably lack the skills to finish in a state where I would be happy with it, I wanted to exercise in a simpler project first. 

This project originally started in August 2024, half a year later I revisited and completely reworked the language, the entire history will be explained below.

Initially, I had the following goals:
- Build a Turing complete language
- provide both input and output
- do everything myself, from lexing, parsing to code generation
- Target x86-64 assembly (this is the architecture I am on, and I don't want to use an emulator to run it)

The language itself should be stack based, this had two reasons, firstly I thought this to be simpler to implement (especially on an assembly level) and secondly I had basically no idea how these languages worked and wanted to learn about it. 

![[Pasted image 20250325192943.png]]
*Example of a push and multiply operation on a stack*

> Stack based programming is a programming paradigm that relies on stacks to manipulate data. They usually utilize [reverse polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) as the basis of their syntax. They aren't that popular due to their rather obscure syntax and how much they differ from more popular languages. 

![[Pasted image 20250325194134.png]]

A compiler is usually split into 3 stages, a frontend which transforms the input source code into some sort of intermediate representation (IR), a middle end that performs optimizations and lastly a backend that generates the actual assembly/binary for your target architecture and operating system. In my case, I did not implement any optimizations.

## Developing the initial version

I chose to develop the compiler in OCaml, a functional language suits itself quite well to the recursive nature of a compiler (at least for a small compiler like mine).

The initial version of the compiler transformed the source code into an IR which then gets generated into x86-64 assembly, assembled and linked with `libc` (for I/O) turning it into an executable binary. The syntax consisted of labels, operands and possibly some arguments (like a value to push, or a label to jump to).

```klaus
# Test program
Read
Push 10
Sub
Cmp > :bigger
Pop
Puts
End

:bigger
Puts
```
*example code of the first version of klaus*

Even though this was more of a pseudo assembly than a real programming language, it was already function and could be used to develop algorithms and programs. When you check out the repo later, you can find some examples I wrote in the older commits :)

## Rewriting the language

This is where I left the project back in August to work on new things. But in the recent weeks, the thought of making a "real" programming language out of the pseudo assembly that klaus was, kept getting back to me, so I reopened the project and got to work again. 

Firstly, I updated the build system to my reworked version of [[Improving my bootstrapped build system|noob]], my own bootstrapped build system. Next, I threw out the old parser and rebuild it to use the reverse polish notation I envisioned. The first iteration had just the input/output and arithmetic operations. Without the control flow, loops and ifs, this was more of a fancy calculator, but it already looked promising.

A simple program to add 10 to your input and then divide it by 2 went from this:

```klaus (old)
Read 
Push 10
Add
Push 2
Div
Puts
```

to this:

```klaus
read 10 + 
2 / puts
```

So far so good, this was easily done, as it was basically just a change in how the language gets parsed. Next up were variables, in the old version klaus provided a `Get [<imm>]` operand which would get you any value from the stack, addressed by either an immediate value in the code, or the value at the top of the stack. I replaced this with a simple, yet effective variable implementation, with which you can store the top of the stack using `:<name>` and retrieve with `<name>`

```klaus
42 :answer
5 6 7 + + 
answer +
puts
```
*Example usage of both storing and retrieving a variable*

## Conditions and Loops

Previously, ifs/conditionals were done via `Cmp` which would jump to a label defined as `:<name>`, I didn't want to leave it that way for 2 reasons:
- the syntax was already used for variables
- it's basically assembly, so why should I use klaus over asm

```klaus
3 4 < if 
	1337 puts
end
```

So instead I went with this approach, an `if` statement enters its inner block of code, when the top of the stack is not 0, otherwise it jumps to the corresponding `end` immediately.
Any comparison (like the `<` here) will put either a 0 or a 1 onto the stack, depending on whether it evaluates to true or false. 

With the previous compare-and-jump-to-label implementation, you could model both ifs and loops, now we are missing loops.

Getting a loop syntax that felt good and also worked how you would expect was quite a challenge, I experimented a lot with getting some kind of for or while loop with an expression inside going, but in the end decided against it. The current loop just loops between `loop` and `end`.

```klaus
loop
	# decrement n and exit if 0
	( n 1 - :n 0 n >= ) if break end
	:prev +
	prev swap
end
```
*Excerpt of the Fibonacci example's loop*

This example shows how you can use and if combined with `break` to build your own loop condition.

## Scopes and Spaces
The last "big" feature of klaus are scopes and spaces. 

Scopes are denoted by `{` and `}`. Scopes in klaus work similar to how you would expect them from C, meaning the stack gets reset at the end of the scope.  This is handy for doing calculations that you just want to store in a variable or print without putting a lot of unused values on the stack or needing to pop them all off manually.

Spaces work similarly to scopes, however here the top of the stack is kept. 
This is handy for if statements where you might need some calculations and comparisons but only care about the 0/1 result. This is shown in the example above.

*However,* scopes and spaces do not have a separate stack frame, if you pop more values from the stack than you pushed onto it, you will access the values outside the scope/space.
Furthermore, at the end of the scope, the top of the stack will be restored. Any values you might have popped off will be restored. 
In general it is advised to avoid doing this as it might lead to unpredictable behavior.



And that is it! You can find a more detailed manual of the language as well as the source code for the compiler and some example programs [here](https://github.com/nailuj05/klaus)
