use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{msg::AdminExecuteMsg, state::ADMIN, ContractError};

pub fn route_admin_execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: AdminExecuteMsg,
) -> Result<Response, ContractError> {
    // validate caller is admin
    if !match ADMIN.load(deps.storage) {
        Ok(admin) => admin == info.sender,
        Err(_) => false,
    } {
        return Err(ContractError::Unauthorized {});
    }

    match msg {
        AdminExecuteMsg::UpdateAdmin { new_admin } => update_admin(deps, new_admin),
    }
}

fn update_admin(deps: DepsMut, new_admin: String) -> Result<Response, ContractError> {
    let new_admin = deps.api.addr_validate(&new_admin)?;

    ADMIN.update(deps.storage, |_| -> Result<_, ContractError> {
        return Ok(new_admin);
    })?;

    return Ok(Response::new());
}
