use serde::Deserialize;
use std::env::args;
use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};

#[derive(Deserialize)]
struct SearchResult {
    _topic: String,
    titles: Vec<String>,
    _descriptions: Vec<String>,
    links: Vec<String>,
}

pub fn search(topic: &str) -> (Vec<String>, Vec<String>) {
    let recieved = reqwest::blocking::get(format!(
        "https://en.wikipedia.org/w/api.php?format=json&action=opensearch&search={}",
        topic
    ))
    .expect("Error when searching for the topic");

    let json: SearchResult = serde_json::from_str(&recieved.text().unwrap())
        .expect("Error on parsing JSON. Changes in the API?");

    (json.titles, json.links)
}

#[derive(Deserialize)]
struct Summary {
    extract: String,
}

pub fn summarize(title: &str) -> String {
    let recieved = reqwest::blocking::get(format!(
        "https://en.wikipedia.org/api/rest_v1/page/summary/{}",
        title
    ))
    .expect("Error when getting the summary");

    let json: Summary = serde_json::from_str(&recieved.text().unwrap()).unwrap();
    json.extract
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

pub struct TopicSelectorTerminal;

impl TopicSelector for TopicSelectorTerminal {
    fn select(&self, topics: &[String]) -> Option<usize> {
        print!("Topics:\n");
        topics
            .iter()
            .enumerate()
            .for_each(|(i, topic)| print!("{}: {}\n", i + 1, topic));

        print!("Select a topic (Default: \"{}\"): ", topics[0]);
        io::stdout().flush().expect("How does flushin stdout fail?");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed the read choice");

        if guess.trim().is_empty() {
            Some(0)
        } else {
            let choice = guess.trim().parse::<i32>().expect("Error parsing the input");
            if choice > 0 && choice <= topics.len() as i32 {
                Some(choice as usize - 1)
            } else {
                println!("Index out of bound");
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
pub struct TopicTakerStdin;

impl TopicTaker for TopicTakerStdin {
    fn take_topic(&self) -> Option<String> {
        let mut topic = String::new();

        io::stdin()
            .lock()
            .read_line(&mut topic)
            .expect("Failed to read line");

        Some(topic.trim().to_string())
    }
}
