// use std::{process::{Command, Output}, fmt::Error};

// fn main() {
//     // let output = Command::new("osmosisd").args(["query", "gov", "proposals","--reverse", "--count-total","--limit","5", "-o", "json"]).output().unwrap();
//     // if output.status.success() == false {
//     //     println!("{:?}", String::from_utf8(output.stderr).unwrap().trim());
//     //     println!("failed");
//     // }
//     // let stdout = allparser::Root::new(output.stdout);
//         let str = get_all_proposals_from_chain("osmosisd");
//         // let str = get_proposal_details("osmosisd", "363");
//         println!("{}", str);
// }
// pub fn get_output_bytes(program: &str, args: Vec<&str>) -> Vec<u8> {
//     let output = Command::new(program).args(args).output().unwrap();
//     if output.status.success() == false {
//         return  "Error".as_bytes().to_vec();
//     }
//     output.stdout
// }
// pub fn get_all_proposals_from_chain(chain: &str) -> String {
//     let mut str: String = String::new();
//     let args = ["query", "gov", "propsals","--reverse", "--count-total","--limit","5", "-o", "json"].to_vec();
//     let output = get_output_bytes(chain, args);
//     if output.len() < 100 {
//         return String::from_utf8(output).unwrap();
//     }
//     let parsed_output = allparser::Root::new(output);
//     for proposal in parsed_output.proposals {
//         str.push_str(&format!("id: {}\ntitle: {}\n", proposal.proposal_id, proposal.content.title));
//     }
//     str
// }

// pub fn get_proposal_details(chain: &str, id: &str) -> String {
//     let mut str: String = String::new();
//     let args = ["query", "gov", "proposal", id, "-o", "json"].to_vec();
//     let output = get_output_bytes(chain, args);
//     let parsed_output = parser::Root::new(output);
//     str.push_str(&format!("title: {}\ndescription: {}",parsed_output.content.title, parsed_output.content.description));
//     str
// }

// pub fn vote(chain: &str,id: &str, vote: &str, wallet: &str) -> String {
//     let mut str: String = String::new();
//     let args = ["tx", "gov", "vote", id, vote, "--from", wallet, "--chain-id", chain ].to_vec();
//     let output = get_output_bytes(chain, args);
//     let string = String::from_utf8(output).unwrap();
//     return string
// }
