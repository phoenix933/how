pub use hdk::prelude::*;
pub use hdk::prelude::Path;
pub use error::{HowError, HowResult};

pub mod error;
pub mod alignment;
pub mod signals;

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    // grant unrestricted access to accept_cap_claim so other agents can send us claims
    let mut functions = BTreeSet::new();
    functions.insert((zome_info()?.name, "recv_remote_signal".into()));
    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        // empty access converts to unrestricted
        access: ().into(),
        functions,
    })?;
    Ok(InitCallbackResult::Pass)
}

entry_defs![
    Path::entry_def(),
    alignment::Alignment::entry_def()
];
