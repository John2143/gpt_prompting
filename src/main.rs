use std::{borrow::Cow, collections::BTreeSet};

use anyhow::bail;
use async_openai::{Client, types::{CreateChatCompletionRequestArgs, ChatCompletionRequestMessageArgs, Role}};


#[derive(serde::Deserialize, Debug)]
pub struct R {
    pub artist: String,
    pub song_name: String,
    pub description: Option<String>,
}

/// this can convert indented multiline strings into non-indented strings
/// see [`space_align_cow`] if your string is 
pub fn space_align(input: &str) -> String {
    space_align_cow(input).to_string()
}

pub fn space_align_cow(input: &str) -> Cow<str> {
    let mut set = BTreeSet::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let diff = line.len() - line.trim_start().len();
        set.insert(diff);
    }

    let min_trim = match set.pop_first() {
        Some(s) => s,
        None => return Cow::Borrowed(input.trim()),
    };


    // bad performance
    let new_str = input
        .lines()
        .map(|line| {
            if line.trim().is_empty() {
                return line;
            }

            return &line[min_trim..];
        })
        .collect::<Vec<_>>()
        .join("\n");

    Cow::Owned(new_str.trim().into())
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let songs = space_align("
        NEW MAGIC WAND, Tyler the creator
        Fazers, King Geedorah
        NEVER, JID
    ");

    let p = space_align(&format!("\
        You are assistant designed to reccomend music.
        You are given a list of songs, and you should suggest what music should be played next.
        Don't recommend artists that fell off. Fire only.

        Step 1: Identify the common musical themes in the given songs.
        Step 2: Generate a list of 2-4 artists that produce similar songs
        Step 3: Choose one song from each artist to recommend
        Step 4: Print the results as a json array with the keys 'artist' and 'song_name'.
    "));

    let s = send_open_api(p, songs).await?;
    println!("{}", s);

    Ok(())
}

pub async fn send_open_api(system: String, user: String) -> anyhow::Result<String> {
    let client = Client::new();


    let chat = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages(vec![
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(system)
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(user)
                .build()?,
        ])
        .n(1)
        .temperature(0.2)
        .user("2143")
        .max_tokens(200u16)
        .build()?;

    let c = client.chat().create(chat).await?;
    for choice in c.choices {
        return Ok(choice.message.content);
    }

    bail!("nah bruv");
}
