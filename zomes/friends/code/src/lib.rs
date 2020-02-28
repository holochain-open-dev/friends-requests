#![feature(proc_macro_hygiene)]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate holochain_json_derive;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use hdk::holochain_core_types::{
    entry::cap_entries::{CapFunctions, CapabilityType},
    time::Timeout,
};
use hdk::prelude::*;
use std::convert::TryInto;

use hdk_proc_macros::zome;

pub mod message;
pub mod request;

use message::MessageBody;
use request::FriendRequest;

#[zome]
mod friends_requests_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn friends_request_def() -> ValidatingEntryType {
        request::entry_def()
    }

    #[zome_fn("hc_public")]
    fn make_friend_request(friend_address: Address) -> ZomeApiResult<()> {
        let grant_address = hdk::commit_capability_grant(
            "see_profile",
            CapabilityType::Assigned,
            Some(vec![friend_address.clone()]),
            CapFunctions::default(),
        )?;

        let request = FriendRequest::initial(friend_address.clone());

        let entry_address = request.address()?;

        let signature = hdk::sign(entry_address)?;

        let message = MessageBody {
            request,
            capability_grant_address: grant_address,
            signature,
        };

        let result: String = hdk::send(
            friend_address.clone(),
            JsonString::from(message).to_string(),
            Timeout::default(),
        )?;

        if result.contains("Error"){
            return Err(ZomeApiError::from(String::from(result)));
        }
        hdk::commit_capability_claim(
            "see_profile",
            friend_address,
            Address::from(result.as_str()),
        )?;

        Ok(())
    }


    #[zome_fn("hc_public")]
    fn get_friends() -> ZomeApiResult<Vec<Address>> {
        request::get_friends()
    }

    #[receive]
    fn receive_friend_request(address: Address, message: JsonString) -> String {
        let success: Result<MessageBody, _> = JsonString::from_json(&message).try_into();
        match success {
            Err(err) => format!("Error: {}", err),
            Ok(message) => {
                /*
                    UNCOMMENT THIS TO LET THE USER DECIDE WHETHER TO ACCEPT THE REQUEST IN THE UI
                let r = hdk::emit_signal(
                    message.signal.as_str(),
                    JsonString::from_json(&format!("{{message: {:?}}}", message)),
                );
                json!(r).to_string() */
                match message::handle_friend_request(address, message) {
                    Ok(capability_address) => capability_address,
                    Err(error) => format!("Error Cannot validate friend request: {:?}", error),
                }
            }
        }
    }
}
