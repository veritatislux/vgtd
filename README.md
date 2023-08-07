<div align="center">
<img src="docs/images/vgtd.svg" width=256px>
</div>

<div align="center">
<img src="https://img.shields.io/github/license/veritatislux/vgtd">
<img src="https://img.shields.io/github/commit-activity/t/veritatislux/vgtd">
<img src="https://img.shields.io/github/issues/veritatislux/vgtd">
<img src="https://img.shields.io/github/v/tag/veritatislux/vgtd">
</div>

<div align="center">
<img src="https://img.shields.io/badge/gitflow-blue">
<img src="https://img.shields.io/badge/conventional_commits-blue">
</div>

---

**vGTD** is the Volt Package's task management tool. It is based on David
Allen's Getting Things Done® system.

Currently, vGTD only serves as a <abbr title="Command Line Interface">CLI</abbr>
tool, usable in the terminal in command form. In the future, however, we plan
to add <abbr title="Terminal User Interface">TUI</abbr> and
<abbr title="Graphical User Interface">GUI</abbr> modes for the tool.

vGTD works within **workspaces**, which are files that hold arrays of lists and
other settings. **Lists** are containers that hold projects and tasks. A
**project** is a container that holds tasks. A **task** has a *name*,
*description*, and *status* and is used to describe a single actionable task.

### Creating a workspace

To start with, you need a workspace. A workspace is nothing but a file in the
local directory called `.gtd.toml`. In order to create it, use the `init`
command:

```bash
vgtd init # Create the workspace file with default contents
```

If your workspace is corrupted or you tried to create the file manually but
failed, use the `reset` command to reset a workspace:

```bash
vgtd reset # Reset the workspace file to the default contents
```

### Dealing with lists

Next, you can take a look at the currently available lists using the `lists`
command:

```bash
vgtd lists # Show the lists in the current workspace
```

This will show you the lists in the workspace, and the amount of tasks and
projects contained within them after their name.

You can create a list using `list create` and remove a list using `list
remove`:

```bash
vgtd list create "example" # Creates the list "example"
vgtd list remove "example" # Removes the list "example"
```

To see the contents of a list, use `list show`:

```bash
vgtd list show "example" # Show the contents of the list "example"
```

This will show the tasks and projects within the list. The projects will be
shown in a single line, with the amount of tasks held in them to the right of
their name. To see each project's tasks along with them, append the `--all`
option or its shortform `-a` to the end of `list show`:

```bash
vgtd lists show "example" --all # Show the projects' tasks as well
```

Right now, your lists are probably empty, so you'll see a "list is empty"
message instead.

### Item and container paths

vGTD wants its users to think of the location of projects and tasks like they
think of folders and files in their system. The names of projects and tasks are
often long and descriptive, so picking tasks and projects by their name in
order to use them in commands would prove to be a tiresome task. In order to
prevent this, indexes are used to refer to projects and tasks instead (hence
why you see indexes before project and task names when using `list show` and
`project show`). Indexes are currently one-based increasing numbers but will
work with an index-to-letter system that we plan to implement in the near
future.

All of that means that the path to the first task of the "inbox" list is
`inbox/1`, the second `inbox/2` and so on. Interestingly, the path to the first
*project* of the "inbox" list is *also* `inbox/1`. vGTD knows how to
differentiate one from the other by context, as it is often intuitive whether a
command wants a path to a task or to a task *container* (which can be a list or
a project). An example of a path to a task contained within a project is
`inbox/1/3`, that is, the task with index `3`, inside the project with index
`1`, inside the list `inbox`.

### Dealing with projects

It is often useful to organize yours tasks not only within lists, but also
within projects inside those lists. Not only that, but according to Mr. David
Allen, projects are often the *source* of many tasks. To create a project, use
`project create`, providing it with the name of the list to create the project
at and the name of the project:

```bash
vgtd project create "inbox" "test" # Create project "test" within the list "inbox"
```

You can see the contents of it using `project show`:

```bash
vgtd project show "inbox/1" # Show the contents of the first project within "inbox"
```

And if you ever want to get rid of a project, use `project remove`:

```bash
vgtd project remove "inbox/1" # Remove the first project within the "inbox" list
```

### Dealing with tasks

You now know how to deal with lists and projects and item paths, which is
great, but knowing how to manipulate task containers is useless if you don't
know how to deal with tasks, so here we go.

To create a task, use `task create`, providing the path to the container that
will hold the task and the name of the task to be created:

```bash
vgtd task create "inbox" "Implement the global workspace" # Create a task inside the "inbox" list
```

To remove a task you added by mistake or want to get rid of, use `task remove`
and provide the task's path:

```bash
vgtd task remove "inbox/1" # Remove the first task of the "inbox" list
```

To change the status of a task, use `task mark` and provide it with the task's
path. You can also provide it with the status to change the task to; by
default, vGTD assumes you want to mark it as "Done". Currently, tasks can only
be "Done" or "Todo", but in the future you'll be able to add your own statuses
or change the default ones.

```bash
vgtd task mark "inbox/1" # Mark the first task of the "inbox" list as done
```

### Moving things around

Now you can create, remove, and manipulate tasks, projects, and lists. Awesome!
But what if you create a task inside the wrong list? Do you have to remove it
and then create it again inside the right list? No! All you need to do is move
things around. To move a task from a task container to another, use `task move`
and provide it with the task's item path and the target task container's path
(yes, you can move tasks from projects to lists or from projects to projects in
other lists).

```bash
vgtd task move "inbox/1" "next" # Move the first task of the "inbox" list to the "next" list
```

You can move not only tasks, but entire projects as well! Using `project move`
you can move a project from a list to another:

```bash
vgtd project move "inbox/1" "next" # Move the first project of the "inbox" list to the "next" list
```

### Getting help

If you forget the syntax of a command, want to know the meaning of an argument,
or discover which options are available to change the behavior of the command,
you can always use the `help` subcommand.

- `vgtd help` will summarize the major commands.
- `vgtd list help` will summarize the commands used to deal with lists.
- `vgtd task help` will summarize the commands used to deal with tasks.
- And so on...
