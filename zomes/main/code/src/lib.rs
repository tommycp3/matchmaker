#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    AGENT_ADDRESS,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
};

use hdk::holochain_json_api::{
    json::JsonString,
    error::JsonError
};

use hdk::holochain_persistence_api::{
    cas::content::Address
};

use hdk_proc_macros::zome;

// see https://developer.holochain.org/api/latest/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct GameProposal {
    agent: Address,
    message: String,
}

#[zome]
mod my_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
     fn game_proposal_entry_def() -> ValidatingEntryType {
        entry!(
            name: "game_proposal",
            description: "this is a same entry representing a proposal to play a game",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<GameProposal>| {
                        Ok(())
                    }
                    
                        
               
        )
    }

    #[zome_fn("hc_public")]
    fn create_proposal(message: String)-> ZomeApiResult<Address> {
        // first create the strut
        let proposal = GameProposal {
            message: message,
            agent: AGENT_ADDRESS.to_string().into(),
        };

        // next create an entry
        let entry = Entry::App(

            "game_proposal".into(),
            proposal.into(),
            );

        // finally commit the entry.
        // this adds it to the DHT an dlocal chain
        // no semi-colon on this line means return the result
        // the result is the address of the new entry on the DHT if successful
        hdk::commit_entry(&entry)
    }




}
