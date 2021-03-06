use cosmwasm_std::{Timestamp, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Basic configuration for the contract
/// The contract will have no admin so this will need to be set correctly
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub withdraw_address: String, // the address whose funds are locked in this contract
    pub withdraw_delay_in_days: u64, // withdraw delay in days
    pub native_denom: String,     // native chain denom - presumably urxp
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Can be run by the admin_address
    /// Starts the withdraw process and creates a timestamp
    /// of when the funds will be ready for claim
    StartWithdraw {},
    /// When the funds are ready to be claimed,
    /// this allows them to actually be claimed
    ExecuteWithdraw {},
}

/// This should only be sudo-callable by the governance
/// module of the chain.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    /// Executes an immediate burn of any funds held by the contract
    ExecuteBurn {},
    /// Sends the specified amount from the contract balance
    /// to a nominated address
    ExecuteSend { recipient: String, amount: Uint128 },
    /// Sends all funds held by the contract
    /// to a nominated address
    ExecuteSendAll { recipient: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// This returns the configured contract info
    GetConfig {},
    /// If a withdrawal has been initiated, this gets
    /// the timestamp that it will be ready to claim
    GetWithdrawalReadyTime {},
    /// Checks if a withdrawal is possible yet
    /// returns a bool response
    IsWithdrawalReady {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WithdrawalTimestampResponse {
    pub withdrawal_ready_timestamp: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WithdrawalReadyResponse {
    pub is_withdrawal_ready: bool,
}
