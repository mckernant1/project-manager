# Project Manager 
This is a git repo manager

### Install
Install via homebrew
```bash
brew tap mckernant1/tools
brew install project-manager
```

### Commands
clone - clones a repo to the default directory
```bash
pm clone git@github.com:mckernant1/project-manager.git
```
pull - pulls all repos, or a specific repo if specified
```bash
pm pull project-manager
```
list - lists all repos listed in the settings file
```bash
pm list
```
status - displays status of all repos or specific repo if specified
```bash
pm status project-manager
```

add - adds an existing repo to the settings file by path
```bash
pm add .
```
rm - removes a directory from the settings file
```bash
pm rm project-manager
pm rm -d project_manager # also deletes the directory
```

### Todo
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
      path: ""
    } 
  ]

}
```
