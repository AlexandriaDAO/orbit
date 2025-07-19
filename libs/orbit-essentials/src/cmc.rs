use crate::utils::check_balance_before_transfer;
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::management_canister::main::{CanisterSettings, CreateCanisterArgument};
use serde::Serialize;

// LOCAL DEVELOPMENT ONLY - This fork bypasses CMC for local development
// DO NOT DEPLOY TO MAINNET - Management canister cannot create canisters with initial cycles on mainnet
// WARNING: This code is modified to use management canister instead of CMC

#[derive(
    CandidType, Deserialize, Serialize, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd,
)]
pub struct SubnetFilter {
    pub subnet_type: Option<String>,
}

/// Options to select subnets when creating a canister.
#[derive(
    CandidType, Deserialize, Serialize, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd,
)]
pub enum SubnetSelection {
    /// Choose a random subnet that satisfies the specified properties
    Filter(SubnetFilter),
    /// Choose a specific subnet
    Subnet { subnet: Principal },
}

/// Argument taken by `create_canister` endpoint of the CMC.
#[derive(candid::CandidType, serde::Serialize)]
struct CreateCanister {
    pub subnet_selection: Option<SubnetSelection>,
    pub settings: Option<CanisterSettings>,
}

/// Error type for `create_canister` endpoint of the CMC.
#[derive(candid::CandidType, candid::Deserialize, serde::Serialize)]
enum CreateCanisterError {
    Refunded {
        refund_amount: u128,
        create_error: String,
    },
}

pub async fn create_canister(
    subnet_selection: Option<SubnetSelection>,
    initial_cycles: u128,
) -> Result<Principal, String> {
    // Log ignored subnet selection for debugging
    if let Some(selection) = &subnet_selection {
        ic_cdk::println!(
            "LOCAL DEV: Ignoring subnet selection {:?}, using local subnet",
            selection
        );
    }
    
    check_balance_before_transfer(initial_cycles).await?;
    
    let create_args = CreateCanisterArgument {
        settings: None,
    };
    
    // Use management canister directly for local development
    let (canister_id,) = ic_cdk::api::management_canister::main::create_canister(
        create_args,
        initial_cycles,
    )
    .await
    .map_err(|e| format!("Failed to create canister: {:?}", e))?;
    
    Ok(canister_id.canister_id)
}
