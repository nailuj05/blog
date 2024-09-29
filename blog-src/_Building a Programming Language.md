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