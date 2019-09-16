# Project Manager 
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/cba71a4e61d9462f8716a5ad05414888)](https://www.codacy.com/manual/mckernant1/project_manager?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=mckernant1/project_manager&amp;utm_campaign=Badge_Grade)
This is a git repo manager

### Commands
clone - clones a repo to the default directory
```bash
pm clone git@github.com:mckernant1/project_manager.git
```
pull - pulls all repos, or a specific repo if specified
```bash
pm pull project_manager
```
list - lists all repos listed in the settings file
```bash
pm list
```
status - displays status of all repos or specific repo if specified
```bash
pm status project_manager
```

add - adds an existing repo to the settings file by path
```bash
pm add .
```
rm - removes a directory from the settings file
```bash
pm rm project_manager
pm rm -d project_manager # also deletes the directory
```
cmds - lists commands for a specific project
```bash
pm cmds project_manager
```
cmd - runs a command for a specified project
```bash
pm cmd project_manager build
```
If the command does not exist. It will prompt you for it.
you can then enter commands like `cargo build` or `npm run serve` these commands will be executed in the projects directory. Gradle is weird and doesnt work when running from a remote directory, so you may have to add a shell script in your gradle directory to run with `sh myscript.sh`


### Todo
- add start/stop commands to start/stop specified command in the background
- figure out how to use bottles with custom taps
- add a rename command

### Settings File Structure
##### Location: ($HOME/.pm.json)
```json5
{
  // Defaults to $HOME/Desktop
  defaultDir: "/Users/tmckernan/Desktop",
  repos: [
    {
      name: "", // defaults to folder name
      path: "",
      cmds: {
        serve: "npm run serve"
      } 
    } 
  ]

}
```
