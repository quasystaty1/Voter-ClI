use serenity::builder::CreateApplicationCommand;
use serenity::model::mention;
use crate::commands::gov::{get_proposal_details, vote};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{CommandDataOptionValue,CommandDataOption};


pub fn run(options: &[CommandDataOption], user: mention::Mention) -> String {
    let chain_option = options
        .get(0)
        .expect("expected a string option")
        .resolved
        .as_ref()
        .expect("expected string object");
    let id_option = options
        .get(1)
        .expect("expected a proposal id")
        .resolved
        .as_ref()
        .expect("expected an integer object");
    let vote_option = options
        .get(2)
        .expect("expected a proposal vote")
        .resolved
        .as_ref()
        .expect("expected an String object");
    let wallet_option = options
        .get(3)
        .expect("expected a wallet name")
        .resolved
        .as_ref()
        .expect("expected an String object");
    let id = if let CommandDataOptionValue::Integer(id) = id_option {
        *id 
    } else {
        0
    };
    let chain = if let CommandDataOptionValue::String(chain) = chain_option {
        chain.to_string()
    } else {
        "error".to_string()
    };
    let voting = if let CommandDataOptionValue::String(vote) = vote_option {
        vote.to_string()
    } else {
        "error".to_string()
    };
    let wallet = if let CommandDataOptionValue::String(wallet) = wallet_option {
        wallet.to_string()
    } else {
        "error".to_string()
    };

    match chain.as_str() {
        "osmosisd" => format!("{}\n{}",user.to_string(),vote(&chain, id.to_string().as_str(), &voting, &wallet)),
        "kujirad" => format!("{}\n{}",user.to_string(),vote(&chain, id.to_string().as_str(), &voting, &wallet)),
        _ => "insert a real chain buddy".to_string() 
    }
    
}


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("vote")
        .description("vote on a governance proposal")
        .create_option(|option|{
            option
                .name("chain")
                .description("chain to query from")
                .kind(CommandOptionType::String)
                .add_string_choice("osmosis", "osmosisd")
                .add_string_choice("kujira", "kujirad")
                .required(true)
        })
        .create_option(|option| {
            option
                .name("id")
                .description("proposal id")
                .kind(CommandOptionType::Integer)
                .min_int_value(1)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("vote")
                .description("what to vote")
                .kind(CommandOptionType::String)
                .add_string_choice("yes", "yes")
                .add_string_choice("no", "no")
                .add_string_choice("no_with_veto", "no_with_veto")
                .add_string_choice("abstain", "abstain")
                .required(true)
        })
        .create_option(|option| {
            option
                .name("wallet")
                .description("voting wallet name")
                .kind(CommandOptionType::String)
                .add_string_choice("wallet", "wallet_name")
                .required(true)
        })
}

