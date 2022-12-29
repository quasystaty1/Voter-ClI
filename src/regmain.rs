use serde_json::Error;
// use serenity::framework;
// use serenity::framework::standard::macros::help;
extern crate chrono;

use std::env;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
// use serenity::framework::StandardFramework;
// use serenity::framework::standard::CommandResult;
// use serenity::framework::standard::macros::{command, group};
use std::process::Command;
use serenity::model::gateway::Ready;

const ALLPROPOSALS: &str = "gov";
const PROPOSAL: &str = "proposal";
const VOTE: &str = "vote";
const HELP_MESSAGE: &str = "
          Hello there, Human!

          You have summoned me. Let's see about getting you what you need.

          ? Need technical help?
          => Post in the <#CHANNEL_ID> channel and other humans will assist you.
          
          ? Looking for the Code of Conduct?
          => Here it is: <https://opensource.facebook.com/code-of-conduct> 
          
          ? Something wrong?
          => You can flag an admin with @admin
          
          I hope that resolves your issue!
          -- Helpbot
          
          ";
pub fn get_output_bytes(program: &str, args: Vec<&str>) -> Vec<u8> {
    let output = Command::new(program).args(args).output().unwrap();
    if output.status.success() == false {
        return  format!("Error: {:?}",output.status.success()).as_bytes().to_vec();
    }

    output.stdout
}
pub fn get_proposal_details(chain: &str, id: &str) -> String {
    let mut str: String = String::new();
    let args = ["query", "gov", "proposal", id, "-o", "json"].to_vec();
    let output = get_output_bytes(chain, args);
    let parsed_output = parser::Root::new(output);
    str.push_str(&format!("title: {}\ndescription: {}",parsed_output.content.title, parsed_output.content.description));
    str
}
pub fn get_all_proposals_from_chain(chain: &str) -> String {
    let mut str: String = String::new();
    let args = ["query", "gov", "proposals","--status", "voting_period","--reverse","--limit","5", "-o", "json"].to_vec();
    let output = get_output_bytes(chain, args);
    if output.len() < 100 {
        return String::from_utf8(output).unwrap();
    }
    let parsed_output = allparser::Root::new(output);
    for proposal in parsed_output.proposals {
        str.push_str(&format!("id: {}\ntitle: {}\n", proposal.proposal_id, proposal.content.title));
    }
    str
}
pub fn vote(chain: &str, id: &str, vote: &str, wallet: &str) -> String {
    let mut str: String = String::new();
    let args = ["tx", "gov", "vote",id, vote, "--from",wallet
    ,"-y","--keyring-backend","file"
    ,"--chain-id", "kayio-1"
    ,"--gas-prices=1ukuji", "--gas=auto", "--gas-adjustment", "1.3"].to_vec();
    let output = get_output_bytes(chain, args);
    if output.len() < 100 {
        return String::from_utf8(output).unwrap();
    }
    let parsed_output = voteparser::Root::new(output);
    str.push_str(&format!("txhash: {}",parsed_output.txhash));
    str
}
// #[group("governance")]
// #[commands(gov)]
// struct General;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    let mut content = msg.content.as_str().split_whitespace();
    
    let mut ret_string = String::new();
    match  content.next() {
        // all osmosisd
        Some(VOTE) => {let string = vote(content.next().unwrap(), content.next().unwrap(), content.next().unwrap(), content.next().unwrap());
            if let Err(why) = msg.channel_id.say(&ctx.http, string).await {
              println!("Error sending message: {:?}", why);
        }},
        Some(ALLPROPOSALS) => {let string = get_all_proposals_from_chain(content.next().unwrap());
        if let Err(why) = msg.channel_id.say(&ctx.http, string).await {
          println!("Error sending message: {:?}", why);
        }},
        // proposal osmosisd 362
        Some(PROPOSAL) => {let string = get_proposal_details(content.next().unwrap(), content.next().unwrap());
            if let Err(why) = msg.channel_id.say(&ctx.http, string).await {
              println!("Error sending message: {:?}", why);
            }},
        _ => (),
    }
    let msg_time = msg.timestamp;
}
async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
}
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let framework = StandardFramework::new()
    // .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
    // .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN")
    .expect("Expected a token in the environment");
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
    // let arb_vec = querier::query_arbs().unwrap();
    // for arb in arb_vec {
    //     println!("{}% on {}",arb.0 , get_name_time(&arb.1));
    // }
    Ok(())
}

// #[command]
// async fn gov(ctx: &Context, msg: &Message) -> CommandResult {
//     msg.reply(ctx, "Pong!").await.unwrap();
//     Ok(())
// }