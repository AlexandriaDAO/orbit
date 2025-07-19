# PRD: Replace CMC with Management Canister for Local Development

## Problem Statement
The Cycles Minting Canister (CMC) cannot run in local dfx environments due to missing system APIs (`mint_cycles`). Since this fork of Orbit will only be used locally, we need to replace CMC calls with management canister calls while maintaining the same interface.

## Solution Overview
Replace all CMC interactions with direct management canister calls, maintaining the same function signatures and behavior to ensure compatibility with canisters that expect CMC-like interfaces.

## Technical Approach

### 1. Replace `create_canister` Function
Update `orbit_essentials/src/cmc.rs::create_canister` to use management canister directly:
```rust
pub async fn create_canister(
    subnet_selection: SubnetSelection,
    initial_cycles: u128,
) -> Result<Principal, String> {
    // Ignore subnet_selection for local development
    // Use management canister directly
    let create_args = CreateCanisterArgument {
        settings: None,
    };
    
    let (canister_id,) = ic_cdk::api::management_canister::main::create_canister(
        create_args,
        initial_cycles,
    ).await.map_err(|e| format!("Failed to create canister: {:?}", e))?;
    
    Ok(canister_id.canister_id)
}
```

### 2. Remove CMC Deployment
- Remove CMC from `dfx.json`
- Remove `--init-cmc` from deployment scripts
- Remove CMC setup functions from `orbit` script

### 3. Disable Cycle Minting
In `core/station/impl/src/services/cycle_manager.rs`:
```rust
// Replace MintCycles with a no-op or remove the FundManager entirely
// For local dev, cycles are unlimited anyway
```

## Implementation Steps

1. **Update `cmc.rs`** (30 min)
   - Replace `create_canister` implementation
   - Keep all type definitions and interfaces the same
   - Add comment explaining this is for local-only use

2. **Clean up deployment scripts** (15 min)
   - Remove CMC references from `dfx.json`
   - Update `deploy_all.sh` to skip CMC
   - Update `orbit` script to remove CMC functions

3. **Disable cycle minting** (15 min)
   - Remove or stub out `MintCycles` operations
   - Keep the interface for compatibility

4. **Testing** (30 min)
   - Verify station deployment works
   - Verify external canister creation works
   - Test with your other projects that interact with Orbit

## Benefits
- Simpler code - no environment detection needed
- Guaranteed to work locally
- Maintains exact same interfaces for compatibility
- No risk of accidentally affecting mainnet (since this fork won't deploy there)

## Key Changes

### Files to Modify:
1. `libs/orbit-essentials/src/cmc.rs` - Replace implementation
2. `core/station/impl/src/services/cycle_manager.rs` - Disable minting
3. `dfx.json` - Remove CMC canister
4. `orbit` - Remove CMC setup functions
5. `deploy_all.sh` - Remove CMC deployment steps

### Functions to Replace:
1. `create_canister()` - Use management canister
2. `MintCycles` - Make no-op or remove

## Success Criteria
1. Can deploy stations without CMC
2. Can create external canisters without CMC
3. All Orbit features work locally
4. Other projects can interact with Orbit stations the same way they would on mainnet