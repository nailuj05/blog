# Improving my bootstrapped build system
With some more free time on my hands I finally have time to work on some fun stuff again - and while doing so, I often came back to my build system to change and improve it. 
I previously wrote a blog entry about it, you can find [here](https://blog.julianlimburg.zip/BuildingCusingC.html). The general idea of the build system remains the same, I recommend reading my previous post if you are not familiar.

![[Pasted image 20250314235858.png]]
*Example of noob implementation*

The first big improvement I did was a refactor to namespace all the build system functions. A namespace in C is just a prefix you add to your functions so they don't collide with functions you might want to define in your program. This is also made to include noob in the main thing you are working on, not just the build system, when you want to use its utilities there.

I also added a simple way to run build commands asynchronously, this is helpful for running longer build steps in parallel. The check for whether a file is outdated, which I use in the rebuild process of noob itself, was also exposed to the user. 

For better usability I also created two sh script, which I also added to my shell as functions, that add and update noob.h to your folder. Furthermore, I added utility functions to create a help/usage page and ensure a directory exists. 

Lastly, I made it so that noob will always be executed from the directory the executable lies in, not the directory you call it from (as would be the case usually). This allows relative paths to still function even if you call noob from somewhere else. 
For example, I had the issue where I was editing a file inside a `src/` directory, but noob was implemented in the root directory of the project. Without this addition, calling noob like this `./../noob` wouldn't work, and you'd had to `cd` back up - well not any longer.