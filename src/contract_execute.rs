use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{msg::ExecuteMsg, ContractError};

pub fn route_execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SampleExecute {} => sample_execute(deps, info),

        // shouldn't happen here
        ExecuteMsg::Admin(_) => return Err(ContractError::Never {}),
    }
}

fn sample_execute(_deps: DepsMut, _info: MessageInfo) -> Result<Response, ContractError> {
    return Ok(Response::new());
}
