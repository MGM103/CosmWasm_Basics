use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    StartGame {
        opponent: Addr,
        host_move: GameMove,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    //GetOwner returns the opponent of the host
    GetOpponent {},
    //GetMove returns the last move the owner of the contract made
    GetMove {},
    //GetOwner returns the owner of the game being played
    GetOwner {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MoveResponse {
    pub move_type: GameMove,
}

//Response struct for ownsership query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OpponentResponse {
    pub opponent: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerResponse {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GameMove {
    //All moves a player can make
    Rock {},
    Paper {},
    Scissors {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GameResult {
    //All moves a player can make
    HostWins {},
    OpponentWins {},
    Tie {},
}