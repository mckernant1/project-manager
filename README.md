# Global Git
This is a git repo manager

### Commands
clone - clones a repo to the default directory
```bash
gblg clone git@github.com:mckernant1/global_git.git
```
pull - pulls all repos, or a specific repo if specified
```bash
gblg pull global_git
```
list - lists all repos listed in the settings file
```bash
gblg list
```
add - adds an existing repo to the settings file by path
```bash
gblg add .
```
rm - removes a directory from the settings file
```bash
gblg rm global_git
gblg rm -d global_git # also deletes the directory
```


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
