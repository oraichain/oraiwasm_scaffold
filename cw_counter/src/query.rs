#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult};

use crate::msg::{QueryMsg, QueryResponse};
use crate::state::STATE;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Counter {} => query_counter(deps, env),
    }
}

pub fn query_counter(deps: Deps, _env: Env) -> StdResult<Binary> {
    let current_state = STATE.load(deps.storage)?;
    let counter = current_state.counter;

    let resp = to_binary(&QueryResponse { counter }).unwrap();
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use crate::contract::instantiate;
    use crate::execute::execute;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, QueryResponse};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::to_binary;

    use super::query;

    const ADDR: &str = "addr1";

    #[test]
    fn test_query() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR, &[]);
        let expect_data_0 = to_binary(&QueryResponse { counter: 0 }).unwrap();
        let expect_data_1 = to_binary(&QueryResponse { counter: 1 }).unwrap();

        let msg = InstantiateMsg {};
        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // query one time.
        let msg = QueryMsg::Counter {};
        let resp = query(deps.as_ref(), env.clone(), msg).unwrap();
        assert_eq!(resp, expect_data_0);

        // query two time
        let msg = ExecuteMsg::Update {};
        let _resp = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        let msg = QueryMsg::Counter {};
        let resp = query(deps.as_ref(), env, msg).unwrap();
        assert_eq!(resp, expect_data_1);
    }
}
