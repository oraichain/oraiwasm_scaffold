#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use crate::state::{State, STATE};

const CONTRACT_NAME: &str = "crates.io:reward-pay&ment";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ZERO_CODE: i32 = 0;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state = State { counter: 0 };

    STATE.save(deps.storage, &state)?;
    Ok(Response::new().add_attribute("counter", ZERO_CODE.to_string()))
}

#[cfg(test)]
mod tests {

    use crate::msg::InstantiateMsg;
    use cosmwasm_std::attr;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use super::{instantiate, ZERO_CODE};

    const ADDR1: &str = "addr1";

    // instantiate with provide admin address
    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);

        let msg = InstantiateMsg {};
        let resp = instantiate(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            resp.attributes,
            vec![attr("counter", ZERO_CODE.to_string())]
        )
    }
}
