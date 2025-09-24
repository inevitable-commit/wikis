# wikis
A CLI tool to fetch a summary on a topic from Wikipedia.

Similar to [wikit](https://github.com/KorySchneider/wikit), except with lots of limitations.
# Installation
For now the installation can only be done by compiling the source code and installing the program using Cargo.
1. Clone this repo.
2. Open terminal in the cloned directory and run following command.
```
$ cargo install --path .
```
# Usage
**Syntax**: 

``wikis [OPTIONS] <TOPIC|--query-stdin|--random|--randomrootpage>``

**Arguments**:

`[TOPIC]...`    Topic to search on the Wikipedia.

**Options**:

| Flag                    | Description                                                                                                                                                                                                                                                                                                                                                         |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--no-link`             | Don't provide the link                                                                                                                                                                                                                                                                                                                                              |
| `--no-summary`          | Don't provide the summary                                                                                                                                                                                                                                                                                                                                           |
| `--lang <LANG>`         | Language edition of Wikipedia to use; defaults to en for English; Language code available [here](https://en.wikipedia.org/wiki/List_of_Wikipedias#Active_editions)                                                                                                                                                                                                  |
| `-c, --choice <CHOICE>` | Index of the topic to choose without prompting                                                                                                                                                                                                                                                                                                                      |
| `--browser`             | Open the Wikipedia page in default browser                                                                                                                                                                                                                                                                                                                          |
| `--v1`                  | Use `summarize` version 1. It uses the API endpoint `https://{lang}.wikipedia.org/api/rest_v1/page/summary/{title}` to fetch the summary. By default `summarize` version 2 is used which uses the API endpoint `https://{lang}.wikipedia.org/w/api.php?format=json&action=query&prop=extracts&exintro&explaintext&redirects=1&titles={title}` to fetch the summary. |
| `--query-stdin`         | Take query from Stdin instead from arguments                                                                                                                                                                                                                                                                                                                        |
| `--no-prompt-text`      | No texts in prompts                                                                                                                                                                                                                                                                                                                                                 |
| `--random`              | Get summary on a random title                                                                                                                                                                                                                                                                                                                                       |
| `--randomrootpage`      | Get summary on a random root page                                                                                                                                                                                                                                                                                                                                   |
| `-h, --help`            | Print help                                                                                                                                                                                                                                                                                                                                                          |
| `-V, --version`         | Print version                                                                                                                                                                                                                                                                                                                                                       |

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

4. Get only the summary in Hindi language.
```
$ wikis --no-link --lang hi Sharukh Khan
Sharukh Khan
शाहरख खान, जिनह अकसर  शाह रख खान क रप म शरय दिया जाता ह और अनौपचारिक रप म एस॰आर॰क॰ नाम स सनदरभित किया जाता, यह एक भारतीय फिलम अभिनता ह। अकसर मीडिया म इनह "बॉलीवड का बादशाह", "किग खान", "रोमास किग" और किग ऑफ बॉलीवड नामो स पकारा जाता ह। शाहरख खान न रोमटिक नाटको स लकर ऐकशन थरिलर जसी शलियो म 72 हिनदी फिलमो म अभिनय किया ह। फिलम उदयोग म उनक योगदान क लिय उनहोन तीस नामाकनो म स चौदह फिलमफयर परसकार जीत ह। व और दिलीप कमार ही ऐस दो अभिनता ह जिनहोन फिलमफयर सरवशरषठ अभिनता परसकार 8 बार जीता ह। 2005 म भारत सरकार न उनह भारतीय सिनमा क परति उनक योगदान क लिए पदम शरी स सममानित किया। 2020 म दनिया क सबस अमीर अभिनता मान गए थ
```

5. Open the Wikipedia page in default browser.
```
$ wikis --browser "don't starve together"
Opening the link in browser
```

6. Take query form `Stdin`. Also with no text in prompts.
```
$ wikis --no-prompt-text --query-stdin
arch linux
1
Arch Linux
https://en.wikipedia.org/wiki/Arch_Linux
Arch Linux is an open source, rolling release Linux distribution. Arch Linux is kept up-to-date by regularly updating the individual pieces of software that it comprises. Arch Linux is intentionally minimal, and is meant to be configured by the user during installation so they may add only what they require.
```

7. Get a summary on a random title
```
$ wikis --random
Huguenot_Memorial_Chapel_and_Monument
https://en.wikipedia.org/wiki/Huguenot_Memorial_Chapel_and_Monument
Huguenot Memorial Chapel and Monument is a historic church located at Manakin, Powhatan County, Virginia.
```