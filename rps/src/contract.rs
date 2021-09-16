#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, StdError};

use crate::error::ContractError;
use crate::msg::{MoveResponse, ExecuteMsg, QueryMsg, OpponentResponse, GameMove};
use crate::state::{GameState, STATE};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {

    let state = GameState {
        host: info.sender.clone(),
        opponent: None,
        host_move: None,
        opponent_move: None,
        game_result: None,
    };

    STATE.save(deps.storage, &info.sender, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("host", &info.sender)
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::StartGame {opponent, host_move} => try_start_game(deps, info, opponent, host_move),
    }
}

pub fn try_start_game(
    deps: DepsMut, 
    info: MessageInfo,
    opponent: Addr,
    host_move: GameMove,
) -> StdResult<Response> {

    let opponent_addr = deps.api.addr_validate(opponent.as_str())?;

    //let state = STATE.load(deps.storage, &info.sender);

    let updated_state = |state: Option<GameState>| -> StdResult<GameState> {
        match state {
            Some(game) => Ok(GameState {
                    host: game.host,
                    opponent: Some(opponent_addr),
                    host_move: Some(host_move),
                    opponent_move: None,
                    game_result: None,
                }),
            None => return Err(StdError::generic_err("Something went wrong starting the game, could not update state.")),
        }
    };

    STATE.update(deps.storage, &info.sender, updated_state)?;
    
    Ok(Response::new().add_attribute("method", "start_game"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, info: MessageInfo, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMove {} => to_binary(&query_move(deps, info)?),
        QueryMsg::GetOpponent {} => to_binary(&query_opponent(deps, info)?),
    }
}

fn query_move(deps: Deps, info: MessageInfo) -> StdResult<MoveResponse> {
    let state = STATE.load(deps.storage, &info.sender)?;
    Ok(MoveResponse { move_type: state.host_move.unwrap() })
}

fn query_opponent(deps: Deps, info: MessageInfo) -> StdResult<OpponentResponse> {
    let state = STATE.load(deps.storage, &info.sender)?;
    Ok(OpponentResponse {opponent: state.opponent.unwrap()})
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};
    use crate::msg::InstantiateMsg;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let _msg = InstantiateMsg {};
        let info = mock_info("Creator", &coins(1000, "uusd"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info).unwrap();
        assert_eq!(0, res.messages.len());

        // only the original creator can reset the counter
        let host_info = mock_info("Creator", &coins(20000, "uusd"));
        let host_info1 = mock_info("Creator", &coins(20000, "uusd"));
        let msg = ExecuteMsg::StartGame { opponent: Addr::unchecked(String::from("Opponent")), host_move: GameMove::Rock{} };
        let _res = execute(deps.as_mut(), mock_env(), host_info, msg).unwrap();

        // Opponent address should not be opponent
        let res = query(deps.as_ref(), mock_env(), host_info1, QueryMsg::GetOpponent {}).unwrap();
        let value: OpponentResponse = from_binary(&res).unwrap();
        assert_eq!("Opponent", value.opponent);

        // Move should now be rock
        let res = query(deps.as_ref(), mock_env(), mock_info("Creator", &coins(20000, "uusd")), QueryMsg::GetMove {}).unwrap();
        let value: MoveResponse = from_binary(&res).unwrap();
        assert_eq!(GameMove::Rock {}, value.move_type);
    }
}
