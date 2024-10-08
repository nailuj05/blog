# Building this blog

As a fitting first real entry to this blog I wanted to write about why (and how) I even created it. I always wanted some kind of place to share my projects and write down my thoughts, not to show off to others but to remember them myself. Just recently however the idea of building a blog formed, I already had the domain so what's there to stop me.

I am not a big fan of JS and its ecosystem of frameworks and whatsoever, so the goal was to make a static website and only using JS where necessary (currently nowhere actually). For the backend I wanted to use Rust, it's been a while since I last used it, and I've gotten rusty (sorry). 

Since school, I've been using Obsidian as my main note-taking tool and I still use it for university and work. My goal was for the blog to be rendered from the markdown pages of my Vault. This way I could just write my entries here in a familiar environment and have it automatically rendered into a nice website for others to view and read.

## Backend

I started by writing a parser to convert the markdown to HTML, I wanted to write this myself as a nice exercise, and so I don't have to rely on other dependencies (another thing I don't really like, especially with projects that are just for fun). The only dependency currently is Rusts Regex module, writing a whole Regex engine myself seemed a bit overkill, but it is definitely something I would like to explore in the future. 

The parser non-recursively parses a file line by line, it supports all common markdown elements such as lists, bold and italics, headers, code and quote blocks, links and images. I also plan to add table support in the future. That should cover everything I currently need, though latex support (like Obsidian) might also be interesting in the future.

For elements that all pages share (such as the header, footer, and recent entries) I created prefab elements to reduce redundancy, they are statically inserted into the final pages when the site is built. 

The rendering of the page works as follows:
1. Clear the current output folder if needed
2. Copy all images/attachments
3. Collect all pages
4. Build the recent entries element from the collected pages
5. Convert all markdown pages and insert them into the entry page HTML template
6. Finally add the template files (index, contact, …)

## Deployment

Originally I planned on deploying on one of my Raspberry Pis and hosting a web server on there. While setting up a GitHub Action to rebuild and run my rust application on the Pi on push, I thought about just hosting the page on GitHub Pages directly, and doing the backend stuff inside an Action. 

Turns out doing all this from an Action is a big hassle, especially if you are as inexperienced with that as I am, but after a lot of trial and error I figured it out. On pushing to the repository (both the backend and the Obsidian vault are contained in the same Repo) an Ubuntu VM is spun up, the project build and run, and the resulting files pushed to a separate branch on the Repo. This branch is deployed to GitHub Pages, my Domain is then pointed to the deployed page using a CNAME.

If I want to add an actual backend instead of just this server-side rendering I might have to move to my Pi again but for now I am happy.

## Future

There are a few things I like to add, firstly I'd like some visitor statistics and the ability to comment, I also want to expand the markdown support to allow for tables and maybe even Latex (for equations, math and special symbols). The CSS needs some work (I really am not a designer and the mobile experience also ins't great right now).
I should also rewrite some Rust stuff, especially fixing a lot of the unwraps to improve safety and the construction and usage of paths.

For now, you can check out everything [here](https://github.com/nailuj05/blog)
