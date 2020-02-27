use crate::request::*;
use hdk::holochain_core_types::entry::cap_entries::{CapFunctions, CapabilityType};
use hdk::holochain_core_types::signature::{Provenance, Signature};
use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MessageBody {
    pub request: FriendRequest,
    pub capability_grant_address: Address,
    pub signature: String,
}

pub fn handle_friend_request(
    friend_address: Address,
    message: MessageBody,
) -> ZomeApiResult<String> {
    if !message.request.friends.contains(&friend_address) {
        return Err(ZomeApiError::from(format!(
            "Cannot make a request for another agent"
        )));
    }

    let provenance = Provenance::new(friend_address.clone(), Signature::from(message.signature));
    let request = message.request;

    match hdk::verify_signature(provenance.clone(), request.address()?)? {
        false => Err(ZomeApiError::from(format!("Cannot validate signature"))),
        true => {
            create_friend_request(request, provenance)?;

            hdk::commit_capability_claim(
                "see_profile",
                friend_address.clone(),
                message.capability_grant_address,
            )?;
            let address = hdk::commit_capability_grant(
                "see_profile",
                CapabilityType::Assigned,
                Some(vec![friend_address]),
                CapFunctions::default(),
            )?;

            Ok(String::from(address))
        }
    }
}
