# jj-starship

starship plugin for jj

## Features

- [x] show bookmarks in you current commits history and how many commits you are ahead of them.
  - [x] filter bookmarks by name.
  - [x] filter bookmarks by distance to current commit.
  - [x] limit number of bookmarks that will be printed.
  - [ ] overwrite bookmark filter per workspace.
- [x] show current commit text.
- [x] show current commit state (Conflict, Divergent, Hidden).
- [x] show current commit metrics (changed files, insertions, deletions).
  - [x] define a custom template for how these changes should be presented.
- [x] print in colors.
- [ ] customize settings via config file.
- [ ] print a default config file.
- [ ] print the path to the default config file path.

## Installation

### From Source
```bash
  cargo install starship-jj --locked
```

## Usage

1. Enable the plugin in you starship.toml

```toml
format="""
...
${custom.jj}\
...
"""

#...

[custom.jj]
command='''starship-jj --ignore-working-copy starship prompt'''
format = "[$symbol](blue bold) $output "
symbol = "󱗆 "
detect_folders=[".jj"]
```
