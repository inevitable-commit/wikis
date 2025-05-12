use std::io::{self, Write};

use clap::Parser;
use wikis::{search, summarize, TopicSelector, TopicSelectorTerminal};

/// A CLI tool to fetch a summary on a topic from Wikipedia.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Whether to provide link
    #[arg(long)]
    no_link: bool,

    /// Whether to provide summary
    #[arg(long)]
    no_summary: bool,

    /// Index of topic to choose
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..))]
    choice: Option<u8>,

    /// Topic to be searched on the Wikipedia
    topic: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let topic = args.topic.join(" ");

    let (topics, links) = search(&topic);

    let choice = if topics.len() > 1 {
        if let Some(c) = args.choice.map(|t| t as usize) {
            if c < 1 || c > topics.len() {
                println!("Index out of bound");
                return;
            }
            c - 1
        } else {
            let from_term = TopicSelectorTerminal {}.select(&topics);

            match from_term {
                Some(c) => c,
                None => {
                    return;
                }
            }
        }
    } else if topics.len() == 1 {
        0
    } else {
        println!("Nothing related to {} was found.", topic);
        return;
    };

    print!("{}\n", topics[choice]);

    if !args.no_link {
        print!("{}\n", links[choice]);
    }

    if !args.no_summary {
        let summary = summarize(&topics[choice]);
        print!("{}\n", summary);
    }

    io::stdout().flush().unwrap();
}
