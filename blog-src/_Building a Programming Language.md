# Building a Programming Language

It's probably one of the coolest things you can do, building your own programming language. Not only is it a great exercise but also a great way to pick and choose all the great things you like from all your favorite languages and make your own.

I previously build a really simple language (more of a pseudo assembly tbh) called [Klaus](https://github.com/nailuj05/klaus). It was a basic, stack based, language with the compiler written in OCaml. It compiled directly down to x86 Assembly, no intermediate language or much processing in between (why it's more of a pseudo assembly). Nonetheless, it was a great learning experience.

This time around I wanted to work with a proper compiler backend, so I don't have to handle the assembly generation all by myself. This also gives me an easy way to support platforms where I don't know the Assembly well (such as ARM). Basically all modern programming languages use the LLVM backend for this.

A compiler backend (such as LLVM) takes an intermediate representation (IR), which is language similar to an assembly language in its simplicity (usually lacking higher level concepts, and being close to the final assembly) but general enough to target any platform. 

I did not choose LLVM as my backend however, I opted to go with QBE. QBE is comparable to LLVM in its function, but simpler and more hackable. In their own words:

> QBE is a compiler backend that aims to provide 70% of the performance of industrial optimizing compilers in 10% of the code. QBE fosters language innovation by offering a compact user-friendly and performant backend. The size limit constrains QBE to focus on the essential and prevents embarking on a never-ending path of diminishing returns.

QBE is also written completely in C instead of LLVMs C++ codebase, which makes it more approachable and therefore hackable (at least in my opinion).

## Finding out how QBE works - the fun way

So, before we start to work on our big compiler, lets first familiarize ourself with QBE IR. There is a basic addition example on their website, so let's try and compile it to understand how it all works!

//....

## Building a Lexer - defining the grammar

The first step in any compiler is the lexer, its purpose is to convert the string of characters that is the source code into a string of tokens. Tokens on which the parser can then perform syntactical analysis and build the abstract syntax tree.

The actual coding of the Lexer is quite simple, it boils down to a simple finite automaton (which is a fancy way of saying the lexer has multiple different states that it switches between when lexing the text).

More important is the actual definition of what the language should look like. I took heavy inspiration from Haskell and OCaml when writing out example programs in mini to get a feel of how the language's design.

```mini
# Global Function Definition
square :: int -> int
square a = a * a

# Main Entry Point
main :: () -> int
main =
    let sum = 10 + 5 in # Local Function Definition
    square sum
```

While the language definition itself is not set in stone there are a few principles I want to follow:
- I want to keep the syntax as simple and small as possible
- I want to avoid a clumsy "annotated" type syntax such as OCaml or Rust's, instead I want the function declarations to be separate (`square :: int -> int`) which can also be put at the top of a file in a C-like fashion
