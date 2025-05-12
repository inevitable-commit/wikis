# wikis
A CLI tool to fetch a summary on a topic from Wikipedia.

Similar to [wikit](https://github.com/KorySchneider/wikit), except with lots of limitations.

# Usage
**Syntax**: 

``wikis [OPTIONS] [TOPIC]...``

**Arguments**:

`[TOPIC]...`    Topic to search on the Wikipedia.

**Options**:

| Flag                    | Description                                     |
| ----------------------- | ----------------------------------------------- |
| `--no-link`             | Don't provide the link
| `--no-summary`          | Don't provide the summary
| `-c, --choice <CHOICE>` | Index of the topic to choose without prompting |
| `-h, --help`            | Print help                                      |
| `-V, --version`         | Print version                                   |

By default, the program will ask to choose a topic if multiple topic related to the query is found. Also, the link to the Wikipedia page for the chosen topic will be given.
# Examples
1. Get summary with default options. (Link is included)
```
$ wikis arch linux
Topics:
1: Arch Linux
2: Arch Linux ARM
Select a topic (Default: "Arch Linux"): 1
Arch Linux
https://en.wikipedia.org/wiki/Arch_Linux
Arch Linux is an open source, rolling release Linux distribution. Arch Linux is kept up-to-date by regularly updating the individual pieces of software that it comprises. Arch Linux is intentionally minimal, and is meant to be configured by the user during installation so they may add only what they require.
```

2. Get summary only.
```
$ wikis --no-link arch linux
Topics:
1: Arch Linux
2: Arch Linux ARM
Select a topic (Default: "Arch Linux"): 2
Arch Linux ARM
Arch Linux ARM is a port of Arch Linux for ARM processors. Its design philosophy is "simplicity and full control to the end user," and like its parent operating system Arch Linux, aims to be very Unix-like.
```

3. Get only the link for the second topic.
```
$ wikis --no-summary -c 2 arch linux
Arch Linux ARM
https://en.wikipedia.org/wiki/Arch_Linux_ARM
```

# Installation
For now the installation can only be done using compiling the source code.
1. Clone this repo.
2. Open terminal in the cloned directory and run following command.
```
$ cargo install --path .
```
