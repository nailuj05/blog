I have todo lists everywhere, on my phone, on the web, on my PC, in my note-taking app etc...
So why not build another one? One that I actually see on a daily basis.
I wanted to keep this project small and simple, something that can be done in an hour. In that spirit I wanted to work in C, for persistent storage I wanted to use a database. Having never worked with databases outside of web applications, I was looking forward to getting some experience in that regard. 

I set a goal to keep it minimal, with just the basics: add, list, delete tasks, and a reset option.

The `todo_init` function sets up a database file, creating a `todos` table with `id` and `task` columns if it doesn’t already exist. This ensures data persists between uses, which is critical for a lightweight task manager like this. Adding tasks is done with a parameterized SQL query for security, and each task is assigned a unique ID, making them easy to refer back to.

The program uses simple CLI commands: `add` adds a new task, `ls` lists all tasks, `del` removes a task by ID, and `reset` clears the database. Listing tasks is a quick SQL query, and deletion uses a bound parameter for the ID to prevent mistakes or injections.

In the end, this quick project turned out exactly as I hoped: a straightforward, always-visible todo list that got me comfortable with SQLite in C, all in just a few lines of code.

And that's already it, keeping it really short with this blog post, I am working on some bigger (and cooler) projects that I'll soon be able to share.

In the meantime, check out the repository [here](https://github.com/nailuj05/todo)