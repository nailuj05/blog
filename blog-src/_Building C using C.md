I hate build systems, I think they are a necessary evil at best - and a design flaw at worst. In almost all cases, you'll need to learn another language just to build the language you actually want to use. And for anything beyond simple applications, it is tedious up to impossible to not use a build system. 

For building C I usually tried to use just a shell script, this is something you can usually read and execute on your system (assuming you are on Linux). It is also simple and transparent. For anything more complex, it becomes just as unusable and clunky as the rest.

You can imagine my excitement when I came across this philosophy:

> The idea is that you should not need anything but a C compiler to build a C project. No make, no cmake, no shell, no cmd, no PowerShell etc. Only C compiler. So with the C compiler you bootstrap your build system and then you use the build system to build everything else.

*From tsoding's [nobuild](https://github.com/tsoding/nobuild)*

Inspired by this, I got to implementing my own: a single header library that allows you to quickly construct a build system, as complex as you want to be. 

```C
#include <noob.h>

int main(int argc, const char *argv[]) {
  RebuildYourself(argc, argv);

  int run = HasFlag(argc, argv, "run");

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
  }

  if (run)
    BuildAndRunCommand("./builds/ying-yang");

  return 0;
}
```

This is the build script for my [yin-yang]() project.

There are also functions to create, extend (for example for adding options or debug flags to a compile command) and freeing a more complex command.
Working with this system in future projects, I will probably expand it further depending on my needs. 