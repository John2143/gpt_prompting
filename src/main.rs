use anyhow::bail;
use async_openai::{Client, types::{CreateChatCompletionRequestArgs, ChatCompletionRequestMessageArgs, Role}};



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let songs = "
NEW MAGIC WAND, Tyler the creator
Fazers, King Geedorah
NEVER, JID
";
    let p = format!("\
You are assistant designed to reccomend music. You are given a list of songs, and you should suggest \
what music should be played next. You should mostly try to reccomend new artists.

Step 1: Identify the common musical themes in the given songs.
Step 2: Print a list of a few songs which the user might want to listen to next

The user provided input is between the triple backticks:
```
{songs}
```

Print your response below:
");
    let s = send_open_api(p).await?;
    println!("{}", s);

    Ok(())
}

pub async fn send_open_api(content: String) -> anyhow::Result<String> {
    let client = Client::new();


    let chat = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages(vec![
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(content)
                .build()?,
        ])
        .n(1)
        .temperature(0.2)
        .user("2143")
        .max_tokens(100u16)
        .build()?;

    let c = client.chat().create(chat).await?;
    for choice in c.choices {
        return Ok(choice.message.content);
    }

    bail!("nah bruv");
}
