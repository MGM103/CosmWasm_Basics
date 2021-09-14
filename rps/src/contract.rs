#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr};

use crate::error::ContractError;
use crate::msg::{MoveResponse, ExecuteMsg, InstantiateMsg, QueryMsg, OpponentResponse};
use crate::state::{State, STATE};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    let opponent_addr = Addr::unchecked("");

    let state = State {
        move_type: msg.move_type,
        owner: info.sender.clone(),
        opponent: opponent_addr,
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("move_type", msg.move_type.to_string())
        .add_attribute("opponent", Addr::unchecked(""))
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    //info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::StartGame {opponent} => try_start_game(deps, opponent),
    }
}

pub fn try_start_game(
    deps: DepsMut, 
    opponent: Addr
) -> StdResult<Response> {

    let opponent = deps.api.addr_validate(opponent.as_str())?;

    STATE.update(deps.storage, |mut state| -> StdResult<_> {
        state.opponent = opponent;
        Ok(state)
    })?;
    
    Ok(Response::new().add_attribute("method", "start_game"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMove {} => to_binary(&query_move(deps)?),
        QueryMsg::GetOpponent {} => to_binary(&query_opponent(deps)?),
    }
}

fn query_move(deps: Deps) -> StdResult<MoveResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(MoveResponse { move_type: state.move_type })
}

fn query_opponent(deps: Deps) -> StdResult<OpponentResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(OpponentResponse {opponent: state.opponent})
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { move_type: 1 };
        let info = mock_info("terra1xg6qez5rrm6vtanj0ss3sn0ua4ympxu53uys6g", &coins(1000, "uusd"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state to check count
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetMove {}).unwrap();
        let value: MoveResponse = from_binary(&res).unwrap();
        assert_eq!(1, value.move_type);
    }

    #[test]
    fn start_game() {
        let mut deps = mock_dependencies(&[]);

        //Setup of contract
        let msg = InstantiateMsg { move_type: 1 };
        let info = mock_info("terra1xg6qez5rrm6vtanj0ss3sn0ua4ympxu53uys6g", &coins(1000, "uusd"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let false_opponent_info = mock_info("", &coins(20000, "uusd"));
        let msg = ExecuteMsg::StartGame { opponent: false_opponent_info.sender };
        let res = execute(deps.as_mut(), mock_env(), msg);
        match res {
            Err(_error) => {}
            _ => panic!("Must return error"),
        }

        // only the original creator can reset the counter
        let opponent_info = mock_info("terra1xg6qez5rrm6vtanj0ss3sn0ua4ympxu53uys6g", &coins(20000, "uusd"));
        let msg = ExecuteMsg::StartGame { opponent: opponent_info.sender };
        let _res = execute(deps.as_mut(), mock_env(), msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOpponent {}).unwrap();
        let value: OpponentResponse = from_binary(&res).unwrap();
        assert_eq!("terra1xg6qez5rrm6vtanj0ss3sn0ua4ympxu53uys6g", value.opponent);
    }
}
