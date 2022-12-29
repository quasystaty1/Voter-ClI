use serenity::builder::CreateApplicationCommand;
use serenity::model::mention;
use std::process::Command;
use std::env;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{CommandDataOptionValue,CommandDataOption};
use crate::commands::parser::Root;
use crate::commands::allparser::Root as AllRoot;
// use crate::commands::voteparser::Root as VoteRoot;
pub fn vote(chain: &str, id: &str, vote: &str, wallet: &str) -> String {
    let pass = env::var("KEYPASSWD").unwrap();
    let pass = pass.as_str();
    let mut data= String::new();
    let args = ["tx", "gov", "vote",id, vote, "--from", wallet
    ,"--keyring-backend","file"
    ,"--chain-id", "kayio-1"
    ,"--gas-prices=1ukuji", "--gas=auto", "--gas-adjustment", "1.3", "-y"].to_vec();
    let mut child_output = Command::new("echo").arg(pass).stdout(std::process::Stdio::piped()).spawn().unwrap();
    if let Some(child_out) = child_output.stdout.take() {
        let output = Command::new(chain).args(args).stdin(child_out).stdout(std::process::Stdio::piped()).spawn().unwrap();
        let head_stdout = output.wait_with_output().unwrap();
        data = String::from_utf8(head_stdout.stdout).unwrap();
    }
    if data.len() == 0 {
        return "error while voting".into();
    }
    data
    // let parsed_output = VoteRoot::new(data);
    // str.push_str(&format!("txhash: {}",parsed_output.txhash));
    // str
}
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
    let parsed_output = Root::new(output);
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
    let parsed_output = AllRoot::new(output);
    for proposal in parsed_output.proposals {
        str.push_str(&format!("id: {}\ntitle: {}\n", proposal.proposal_id, proposal.content.title));
    }
    str
}
pub fn run(options: &[CommandDataOption], user: mention::Mention) -> String {
    let option = options
        .get(0)
        .expect("expected a string option")
        .resolved
        .as_ref()
        .expect("expected string object");
    let chain = if let CommandDataOptionValue::String(chain) = option {
        chain.to_string()
    } else {
        "error".to_string()
    };
    match chain.as_str() {
        "osmosisd" => format!("{}\n{}",user.to_string(), get_all_proposals_from_chain(chain.as_str())),
        _ => "insert a real chain buddy".to_string() 
    }
    
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("gov").description("get all governance proposals from a chain").create_option(|option|{
        option
            .name("chain")
            .description("chain to query from")
            .kind(CommandOptionType::String)
            .add_string_choice("osmosis", "osmosisd")
            .required(true)
    })
}