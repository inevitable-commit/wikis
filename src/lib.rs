use percent_encoding::{AsciiSet, CONTROLS};
use reqwest::header::LOCATION;
use reqwest::redirect::Policy;
use serde::Deserialize;
use std::env::args;
use std::io::{self, BufRead, Write};
use std::process::{exit, Command, Stdio};

#[derive(Deserialize)]
struct SearchResult {
    _topic: String,
    titles: Vec<String>,
    _descriptions: Vec<String>,
    links: Vec<String>,
}

#[derive(Deserialize)]
struct Summary {
    extract: String,
}

const FRAGMENT: &AsciiSet = &CONTROLS.add(b'/');

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (https://github.com/inevitable-commit/wikis)"
);

pub struct MyClient {
    client: reqwest::blocking::Client,
}

impl MyClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .gzip(true)
                .user_agent(APP_USER_AGENT)
                .redirect(Policy::none())
                .build()
                .expect("Error building client"),
        }
    }

    pub fn search(&self, lang: &str, topic: &str) -> (Vec<String>, Vec<String>) {
        let response = self
            .client
            .get(format!(
                "https://{}.wikipedia.org/w/api.php?format=json&action=opensearch&search={}",
                lang, topic
            ))
            .send()
            .expect("Error when searching for the topic");

        let json: SearchResult = response
            .json::<SearchResult>()
            .expect("Error on parsing JSON.");

        (json.titles, json.links)
    }

    pub fn summarize(&self, lang: &str, title: &str, link: &str) -> (String, String, String) {
        let encoded_title =
            percent_encoding::utf8_percent_encode(title, FRAGMENT).to_string();
        let response = self
            .client
            .get(format!(
                "https://{}.wikipedia.org/api/rest_v1/page/summary/{}",
                //"https://{}.wikipedia.org/w/api.php?format=json&action=query&prop=extracts&exintro&explaintext&redirects=1&titles={}",
                lang,
                encoded_title
            ))
            .send()
            .expect("Error when requesting summary for the topic");

        let status = response.status();

        if status.is_success() {
            (
                title.to_string(),
                link.to_string(),
                response.json::<Summary>().unwrap().extract,
            )
        } else if status.as_u16() == 403 { // 403 for Special:Random. Also redirection
            let response = self
                .client
                .get(format!("https://{}.wikipedia.org/wiki/{}", lang, title))
                .send()
                .expect("Error when fetching HTML page of the topic");

            let status = response.status();

            if status.is_redirection() {
                let link = response
                    .headers()
                    .get(LOCATION)
                    .expect("Expecting location header")
                    .to_str()
                    .expect("Header contained non ASCII characters.");

                let title = link
                    .split(&format!("https://{}.wikipedia.org/wiki/", lang))
                    .last()
                    .unwrap();

                self.summarize(lang, title, link) // finger cross, no infinite recursion. Potential
                                                  // problems: Titles with symbols
            } else {
                eprintln!(
                    "Expected redirection.\nTitle: {}\nLink: {}\n encoded_title: {}",
                    title, link, encoded_title
                );
                exit(1)
            }
        } else {
            eprintln!("Couldn't fetch the summary on {}. The link is {}", title, link);
            exit(1)
        }
    }
}

// can't figure out proper interfaces
pub trait TopicTaker {
    fn take_topic(&self) -> Option<String>;
}

pub trait TopicSelector {
    fn select(&self, topics: &[String]) -> Option<usize>;

    fn skip_selection_if_single_topic(&self) -> bool {
        true
    }
}

pub trait ResultProcessor {
    fn process(&mut self, topic: String, link: String, summary: String);
}

pub struct TopicSelectorTerminal {
    pub show_prompt_text: bool,
}

impl TopicSelector for TopicSelectorTerminal {
    fn select(&self, topics: &[String]) -> Option<usize> {
        if self.show_prompt_text {
            print!("Topics:\n");
            topics
                .iter()
                .enumerate()
                .for_each(|(i, topic)| print!("{:>2}: {}\n", i + 1, topic));

            print!("Select a topic (Default: \"{}\"): ", topics[0]);
        }

        io::stdout().flush().expect("How does flushin stdout fail?");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed the read choice");

        if guess.trim().is_empty() {
            Some(0)
        } else {
            let choice = guess
                .trim()
                .parse::<i32>()
                .expect("Error parsing the input");
            if choice > 0 && choice <= topics.len() as i32 {
                Some(choice as usize - 1)
            } else {
                eprintln!("Index out of bound");
                None
            }
        }
    }
}

//unused codes
pub struct NthTopicSelector(usize);

impl TopicSelector for NthTopicSelector {
    fn select(&self, _topics: &[String]) -> Option<usize> {
        Some(self.0)
    }
}

pub struct TopicSelectorDmenu;

impl TopicSelector for TopicSelectorDmenu {
    fn select(&self, topics: &[String]) -> Option<usize> {
        let number_of_lines = 5;
        let mut dmenu = Command::new("dmenu")
            .arg("-i")
            .arg("-l")
            .arg(number_of_lines.to_string())
            .arg("-p")
            .arg("Select a topic: ")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Error with dmenu");

        if let Some(stdin) = dmenu.stdin.as_mut() {
            let s = topics.join("\n");

            stdin
                .write_all(s.as_bytes())
                .expect("Failed to write to dmenu")
        }

        let output = dmenu
            .wait_with_output()
            .expect("Error on waiting for dmenu");
        let selection = String::from_utf8_lossy(&output.stdout);

        topics.iter().position(|s| *s == selection.trim())
    }
}

pub struct Dunst;

impl ResultProcessor for Dunst {
    fn process(&mut self, topic: String, link: String, summary: String) {
        Command::new("notify-send")
            .args([&topic, &summary])
            .output()
            .unwrap();
    }
}

pub struct Terminal;

impl ResultProcessor for Terminal {
    fn process(&mut self, topic: String, link: String, summary: String) {
        println!("{}\n{}\n{}", topic, link, summary);
    }
}

pub struct TopicTakerArg;

impl TopicTaker for TopicTakerArg {
    fn take_topic(&self) -> Option<String> {
        let mut arguments = args();
        let _ = arguments.next();
        arguments.next()
    }
}

// when using with TopicSelectorTerminal provide the topic selection with the topic like:
// "terraria\n2". because pipe is closed and topic read using stdin returns ""
pub struct TopicTakerStdin {
    pub show_prompt_text: bool,
}

impl TopicTaker for TopicTakerStdin {
    fn take_topic(&self) -> Option<String> {
        if self.show_prompt_text {
            print!("Enter query: ");
            io::stdout().flush().expect("How does flushin stdout fail?");
        }

        let mut topic = String::new();
        io::stdin()
            .lock()
            .read_line(&mut topic)
            .expect("Failed to read line");

        Some(topic.trim().to_string())
    }
}
