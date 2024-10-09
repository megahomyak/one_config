# one.config - one config for everything

"one.config" is a simple utility to manage complex environments. You don't need to copy and paste configurations and scripts anymore: one.config can store them in one global place or in your project's root directory and then run at your command.

## How to set this up

Next sections describe the steps you need to take to set up this amazing program.

### Executable installation

`cargo install one_config`

or

`git clone` this repository and then `cargo install --path /path/to/cloned/repository`

The former way is recommended since the version of the tool will certainly be stable. Otherwise you'd need to `git switch` to the desired version tag to get to a stable version of the repository.

Rename the installed binary to "one.config" (optional, but recommended).

### Understanding the order of precedence

* There are two kinds of config files: the ones that are global to your user (located in your OS's config directory for the current user) and the ones that are local to your projects (located at the project root).
* Command executables are also considered "config files".
* The local configs take precedence over global configs.
* The local configs are picked up either from the current directory or one of its ancestors. The search stops when either `one.config` or `one.config.commands` are found, and then the `PROJECT_ROOT` environment variable is set to the directory where the search stopped.

### Setting up command executables

* Command executable files are just some executable files that are named with the name you want the command to have.
* Command executables should be stored directly in a `one.config.commands` directory.

### Setting up environment variables

* Environment variables are located in a `one.config` file.
* Lines that are empty or begin with "#" are not considered.
* Lines that *are* considered should have at least one "=" in them. What's before the first "=" is considered to be the name of the environment variable and what's after it is considered to be the value.

## How to run one.config

Just go into your project's directory and run `one.config {command} {any arguments here...}`, where "command" is the name of the executable you want to run! Here's what happens:

* one.config collects the configuration information: the executable files and the environment variable files
* After setting these up (in order, of course), it tries to find the executable with the specified name
* If the executable is found, it is run with the provided arguments (that is, with the arguments after the command name)
