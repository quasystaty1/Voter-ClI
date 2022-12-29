use serenity::builder::CreateApplicationCommand;
use serenity::model::mention;
use crate::commands::gov::get_proposal_details;
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
        .expect("expected an proposal id")
        .resolved
        .as_ref()
        .expect("expected an integer object");
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

    match chain.as_str() {
        "osmosisd" => format!("{}\n{}",user.to_string(), get_proposal_details(chain.as_str(), id.to_string().as_str())),
        _ => "insert a real chain buddy".to_string() 
    }
    
}


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("proposal")
        .description("get a governance proposals from a chain")
        .create_option(|option|{
            option
                .name("chain")
                .description("chain to query from")
                .kind(CommandOptionType::String)
                .add_string_choice("osmosis", "osmosisd")
                .required(true)
        })
        .create_option(|option| {
            option
                .name("id")
                .description("the proposal id to query")
                .kind(CommandOptionType::Integer)
                .min_int_value(1)
                .required(true)
        })
}

