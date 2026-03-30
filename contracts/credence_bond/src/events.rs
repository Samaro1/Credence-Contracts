use soroban_sdk::{Address, Env, Symbol};

/// Emitted when a new bond is created.
///
/// # Topics
/// * `Symbol` - "bond_created"
/// * `Address` - The identity owning the bond
///
/// # Data
/// * `i128` - The initial bonded amount
/// * `u64` - The duration of the bond in seconds
/// * `bool` - Whether the bond is rolling
pub fn emit_bond_created(
    e: &Env,
    identity: &Address,
    amount: i128,
    duration: u64,
    is_rolling: bool,
) {
    let topics = (Symbol::new(e, "bond_created"), identity.clone());
    let data = (amount, duration, is_rolling);
    e.events().publish(topics, data);
}

/// Emitted when an existing bond is increased (topped up).
///
/// # Topics
/// * `Symbol` - "bond_increased"
/// * `Address` - The identity owning the bond
///
/// # Data
/// * `i128` - The additional amount added
/// * `i128` - The new total bonded amount
pub fn emit_bond_increased(e: &Env, identity: &Address, added_amount: i128, new_total: i128) {
    let topics = (Symbol::new(e, "bond_increased"), identity.clone());
    let data = (added_amount, new_total);
    e.events().publish(topics, data);
}

/// Emitted when funds are successfully withdrawn from a bond.
///
/// # Topics
/// * `Symbol` - "bond_withdrawn"
/// * `Address` - The identity owning the bond
///
/// # Data
/// * `i128` - The amount withdrawn
/// * `i128` - The remaining bonded amount
pub fn emit_bond_withdrawn(e: &Env, identity: &Address, amount_withdrawn: i128, remaining: i128) {
    let topics = (Symbol::new(e, "bond_withdrawn"), identity.clone());
    let data = (amount_withdrawn, remaining);
    e.events().publish(topics, data);
}

/// Emitted when a bond is slashed by an admin.
///
/// # Topics
/// * `Symbol` - "bond_slashed"
/// * `Address` - The identity owning the bond
///
/// # Data
/// * `i128` - The amount slashed in this event
/// * `i128` - The new total slashed amount for this bond
pub fn emit_bond_slashed(e: &Env, identity: &Address, slash_amount: i128, total_slashed: i128) {
    let topics = (Symbol::new(e, "bond_slashed"), identity.clone());
    let data = (slash_amount, total_slashed);
    e.events().publish(topics, data);
}
/// Emitted when a new claim is added for a user.
///
/// # Topics
/// * `Symbol` - "claim_added"
/// * `Address` - The user who can claim
///
/// # Data
/// * `crate::claims::ClaimType` - The type of claim
/// * `i128` - The amount to be claimed
/// * `u64` - The source ID that generated this claim
pub fn emit_claim_added(e: &Env, user: &Address, claim: &crate::claims::PendingClaim) {
    let topics = (Symbol::new(e, "claim_added"), user.clone());
    let data = (claim.claim_type, claim.amount, claim.source_id);
    e.events().publish(topics, data);
}

/// Emitted when claims are processed by a user.
///
/// # Topics
/// * `Symbol` - "claims_processed"
/// * `Address` - The user who claimed
///
/// # Data
/// * `u32` - Number of claims processed
/// * `i128` - Total amount claimed
/// * `soroban_sdk::Vec<crate::claims::ClaimType>` - Types of claims processed
pub fn emit_claims_processed(
    e: &Env,
    user: &Address,
    result: &crate::claims::ClaimResult,
    _processed_claims: &soroban_sdk::Vec<crate::claims::PendingClaim>,
) {
    let topics = (Symbol::new(e, "claims_processed"), user.clone());
    let data = (
        result.processed_count,
        result.total_amount,
        result.claim_types.clone(),
    );
    e.events().publish(topics, data);
}

/// Emitted when expired claims are cleaned up.
///
/// # Topics
/// * `Symbol` - "claims_expired"
/// * `Address` - The user whose claims expired
///
/// # Data
/// * `u32` - Number of expired claims removed
/// * `i128` - Total amount of expired claims
pub fn emit_claims_expired(e: &Env, user: &Address, expired_count: u32, expired_amount: i128) {
    let topics = (Symbol::new(e, "claims_expired"), user.clone());
    let data = (expired_count, expired_amount);
    e.events().publish(topics, data);
}

/// Emitted when upgrade authorization is initialized.
///
/// # Topics
/// * `Symbol` - "upgrade_auth_initialized"
/// * `Address` - The upgrade admin address
pub fn emit_upgrade_auth_initialized(e: &Env, admin: &Address) {
    let topics = (Symbol::new(e, "upgrade_auth_initialized"), admin.clone());
    e.events().publish(topics, ());
}

/// Emitted when upgrade authorization is granted.
///
/// # Topics
/// * `Symbol` - "upgrade_auth_granted"
/// * `Address` - The admin who granted authorization
/// * `Address` - The address that received authorization
///
/// # Data
/// * `crate::upgrade_auth::UpgradeRole` - The role granted
pub fn emit_upgrade_auth_granted(
    e: &Env,
    admin: &Address,
    address: &Address,
    role: crate::upgrade_auth::UpgradeRole,
) {
    let topics = (
        Symbol::new(e, "upgrade_auth_granted"),
        admin.clone(),
        address.clone(),
    );
    e.events().publish(topics, role);
}

/// Emitted when upgrade authorization is revoked.
///
/// # Topics
/// * `Symbol` - "upgrade_auth_revoked"
/// * `Address` - The admin who revoked authorization
/// * `Address` - The address whose authorization was revoked
pub fn emit_upgrade_auth_revoked(e: &Env, admin: &Address, address: &Address) {
    let topics = (
        Symbol::new(e, "upgrade_auth_revoked"),
        admin.clone(),
        address.clone(),
    );
    e.events().publish(topics, ());
}

/// Emitted when an upgrade is proposed.
///
/// # Topics
/// * `Symbol` - "upgrade_proposed"
/// * `Address` - The proposer address
///
/// # Data
/// * `u64` - The proposal ID
/// * `Address` - The new implementation address
pub fn emit_upgrade_proposed(
    e: &Env,
    proposer: &Address,
    proposal_id: u64,
    new_implementation: &Address,
) {
    let topics = (Symbol::new(e, "upgrade_proposed"), proposer.clone());
    let data = (proposal_id, new_implementation);
    e.events().publish(topics, data);
}

/// Emitted when an upgrade proposal is approved.
///
/// # Topics
/// * `Symbol` - "upgrade_approved"
/// * `Address` - The approver address
///
/// # Data
/// * `u64` - The proposal ID
pub fn emit_upgrade_approved(e: &Env, approver: &Address, proposal_id: u64) {
    let topics = (Symbol::new(e, "upgrade_approved"), approver.clone());
    e.events().publish(topics, proposal_id);
}

/// Emitted when an upgrade is executed.
///
/// # Topics
/// * `Symbol` - "upgrade_executed"
/// * `Address` - The executor address
/// * `Address` - The new implementation address
///
/// # Data
/// * `Option<u64>` - The proposal ID (if any)
pub fn emit_upgrade_executed(
    e: &Env,
    executor: &Address,
    new_implementation: &Address,
    proposal_id: Option<u64>,
) {
    let topics = (
        Symbol::new(e, "upgrade_executed"),
        executor.clone(),
        new_implementation.clone(),
    );
    e.events().publish(topics, proposal_id);
}
