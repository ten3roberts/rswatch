# rswatch
A program for monitoring files and directories and executing command on change

## Use cases
* Print modified, added, or removed files from directory structure
* Execute command for changed files


## Examples

`rswatch src -e cat {}` Checks for changes in `src` directory and run `cat` with the changed files

`rswatch src -p` Print changed files to stdout

`rswatch src -ke compile_and_run.sh` Run `compile_and_run.sh` on any change to `src` directory. If the process is still running, e.g; program is in a loop, kill instead of wait (`-k --kill` flag)

This is useful if you're making an interactive application that runs in a loop or waits for used input. -k flag will kill the process and restart it rather than waiting for it to complete.

Setting up this script in a terminal can be used to automatically rebuild and run tests on save


# Exiting
Using Ctrl-C (SIGINT) in terminal will terminate child process if running, if child process isn't running, it will prompt to confirm exit, simply enter y or `enter` to exit

This is to prevent accidental exit from rswatch when terminating child process

## Usage
### Accessible with `rswatch --help`

    -, --(unnamed) [required]
        Specify which files and/or directories to watch

    -h, --help 
        Show usage

    -p, --print 
        Print modified files and directories to stdout

    -v, --verbose 
        Verbose output

    -i, --interval 
        Time in milliseconds between checks (default 100)

    -k, --kill 
        Specifies to kill the child process `exec` rather than wait for it

    -c, --clear 
        Clears the screen before restarting childprocess `exec`

    -s, --start 
        Start the process `exec` on startup instead of waiting for a file to change before doing so

    -e, --exec 
        Command to execute when files change
        All following arguments are given to the specified command
        {} is to be replaced by the changed files, to run command for each changed file separately
        Watch is not blocked during execution when process is spawned but will wait until previous finished before rerunning next check
        To kill and restart process rather than waiting, use option --kill
