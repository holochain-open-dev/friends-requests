use hdk::prelude::*;
use hdk::AGENT_ADDRESS;
use holochain_wasm_utils::api_serialization::commit_entry::CommitEntryOptions;
use hdk::holochain_core_types::signature::{Provenance, Signature};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct FriendRequest {
    pub friends: Vec<Address>,
    pub friends_status: bool,
    pub last_header_address: Option<Address>,
}

impl FriendRequest {
    pub fn initial(friend: Address) -> FriendRequest {
        FriendRequest {
            friends: vec![friend, AGENT_ADDRESS.clone()],
            last_header_address: None,
            friends_status: true,
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App("friend_request".into(), self.into())
    }

    pub fn address(&self) -> ZomeApiResult<Address> {
        hdk::entry_address(&self.entry())
    }
}

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: "friend_request",
        description: "this is a same entry defintion",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<FriendRequest>| {
            Ok(())
        },
        links: [
            from!(
                "%agent_id",
                link_type: "friend->friend_request",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}

pub fn create_friend_request(request: FriendRequest, provenance: Provenance) -> ZomeApiResult<Address> {
    let friend_address = provenance.source();
    let address = commit_with_provenance(&request.entry(), provenance)?;

    hdk::link_entries(
        &AGENT_ADDRESS.clone(),
        &address,
        "friend->friend_request",
        String::from(friend_address.clone()).as_str(),
    )?;
    hdk::link_entries(
        &friend_address,
        &address,
        "friend->friend_request",
        String::from(AGENT_ADDRESS.clone()).as_str(),
    )?;

    Ok(address)
}


pub fn commit_with_provenance(entry: &Entry, provenance: Provenance) -> ZomeApiResult<Address> {
    let address = hdk::entry_address(&entry)?;

    let signature = hdk::sign(address)?;

    let my_provenance = Provenance::new(AGENT_ADDRESS.clone(), Signature::from(signature));

    let options = CommitEntryOptions::new(vec![provenance, my_provenance]);

    let address = hdk::commit_entry_result(&entry, options)?;
    Ok(address.address())
}
