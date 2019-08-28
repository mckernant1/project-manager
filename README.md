# Project Manager 
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
add - adds an existing repo to the settings file by path
```bash
pm add .
```
rm - removes a directory from the settings file
```bash
pm rm project_manager
pm rm -d project_manager # also deletes the directory
```
cmd - runs a command for a specified project
```bash
pm cmd project_manager build
```
If the command does not exist. It will prompt you for it.
you can then enter commands like `cargo build` or `npm run serve` these commands will be executed in the projects directory. Gradle is weird and doesnt work when running from a remote directory, so you may have to add a shell script in your gradle directory to run with `sh myscript.sh`

### Settings File Structure
##### Location: ($HOME/.gg.json)
```json5
{
  // Defaults to $HOME/Desktop
  defaultDir: "/Users/tmckernan/Desktop",
  repos: [
    {
      name: "", // defaults to folder name
      path: "", 
      
    } 
  ]

}
```
