use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::{error::ContractError, msg::Response};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, HandleResponse, InitResponse, MessageInfo, StdResult,
};

pub fn init(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    Ok(InitResponse::default())
}

// And declare a custom Error variant for the ones where you will want to make use of it
pub fn handle(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: HandleMsg,
) -> Result<HandleResponse, ContractError> {
    Ok(HandleResponse::default())
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Test { input } => get(deps, input),
    }
}

fn get(_deps: Deps, input: String) -> StdResult<Binary> {
    let response = Response { data: input };
    let resp_bin: Binary = to_binary(&response)?;
    Ok(resp_bin)
}
