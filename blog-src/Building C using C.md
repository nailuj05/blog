# Building C using C

I hate build systems, I think they are a necessary evil at best - and a design flaw at worst. In almost all cases, you'll need to learn another language just to build the language you actually want to use. And for anything beyond simple applications, it is tedious up to impossible to not use a build system. 

For building C I usually tried to use just a shell script, this is something you can usually read and execute on your system (assuming you are on Linux). It is also simple and transparent. For anything more complex, it becomes just as unusable and clunky as the rest.

You can imagine my excitement when I came across this philosophy:

> The idea is that you should not need anything but a C compiler to build a C project. No make, no cmake, no shell, no cmd, no PowerShell etc. Only C compiler. So with the C compiler you bootstrap your build system and then you use the build system to build everything else.

*From tsoding's [nobuild](https://github.com/tsoding/nobuild)*

Inspired by this, I got to implementing my own: a single header library that allows you to quickly construct a build system, as complex as you want to be. 
I called it noob, which stand for no (original) build, as a reference to tsodings original nob.

```C
#include <noob.h>

int main(int argc, const char *argv[]) {
  RebuildYourself(argc, argv);

  if (HasFlag(argc, argv, "web")) {
    BuildAndRunCommand("rm -rf builds/web/* && emcc -o builds/web/yingyang.html src/ying-yang.c -Iraylib/src -Lraylib/src -lraylib "
                       "-s USE_GLFW=3 -s ASYNCIFY -s TOTAL_MEMORY=16777216 -DPLATFORM_WEB -DSUPPORT_TRACELOG=0 --shell-file src/shell.html");

    if (HasFlag(argc, argv, "serve"))
      BuildAndRunCommand("emrun builds/web/yingyang.html");
  } else {
    BuildAndRunCommand("rm -rf builds/ying-yang && gcc src/ying-yang.c -Iinclude -lraylib "
                       "-lGL -lm -lpthread "
                       "-ldl -lrt -lX11 "
                       "-o builds/ying-yang");
    
    if (HasFlag(argc, argv, "run"))
	    BuildAndRunCommand("./builds/ying-yang");
  }

  return 0;
}
```

This is the build script for my [[Raylib, C, Physics and WebAssembly|yin-yang]] project.

Once the build system is bootstrapped (compiled using just `gcc -o noob noob.c`) you are good to go. You can now run the build by calling noob. Passing `run` will not only compile the raylib project (with all the flags you really don't want to type out manually) and then run the binary.

If you want to compile the WebAssembly version, you can pass in `web` (requires you have emscripten set up and enabled, but that is besides the point). You can also pass in `serve` to run the web server to test it as well.

I am actively expanding noobs functionality with all the helper functions I find useful in my projects, such as easily checking argv for flags, async exectution for bigger builds and of course checking whether a binary file is outdated and recompiling if necessary (for incremental builds). 

The really cool part is the bootstrapping, though. In any project, you will only have to compile noob once. After that (and assuming you called `RebuildYourself()` at the top) noob will detect whether it is out of date (the binary is older than the last modified of the source), recompile and the run itself.

```C
void RebuildYourself(int argc, const char **argv) {
  int source = nb_GetLastModified("noob.c");
  int exec = nb_GetLastModified("noob");

  if (source > exec) {
    printf("Rebuilding\n");
    if (nb_Recompile() == 0) {
      BuildCommand *bc = CreateBuildCommand(128);

      for (int i = 0; i < argc; i++)
        AddCommand(bc, argv[i]);

      RunCommand(bc);

      FreeCommand(bc);
      exit(0);
    }
    exit(1);
  }
}
```

Regardless whether you changed your build system and/or your project itself, just calling noob will handle it. RebuildYourself() also pass the arguments through so besides the "Rebuilding" printed to your stdout, it will be seamless.

C can call any command and, due to being Turing complete, can model any logic. Annoying and repetitive tasks, such as checking for flags in `argv`, can be put into the library to keep the actual build script as concise as possible. 

Another cool perk of including the entire build system in the header file within the project is that it will never cause issues with newer versions (a common issue with GNU autotools). For example I recently refactored noob to use a more concise syntax and a prefix to avoid namespace collisions in bigger projects. This doesn't force me to rewrite any of the noob.c build files in my current projects however, they are all self contained and will continue to work. 

Of course, C can not just build C (and I use it for other projects too) but the philosophy itself is also portable to most languages. Why not compile Ocaml in Ocaml?

As always you can check out the entire source [here](https://github.com/nailuj05/noob)
