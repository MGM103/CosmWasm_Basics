use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Map;
use crate::msg::{ GameMove, GameResult };

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameState {
    pub host: Addr,
    pub opponent: Option<Addr>,
    pub host_move: Option<GameMove>,
    pub opponent_move: Option<GameMove>,
    pub game_result: Option<GameResult>,
}

pub const STATE: Map<&Addr, GameState> = Map::new("game_state");
