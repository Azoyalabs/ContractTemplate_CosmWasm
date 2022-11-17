use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;


#[cw_serde]
pub struct InstantiateMsg {

}

#[cw_serde]
pub enum AdminExecuteMsg {
    UpdateAdmin { new_admin: String },

}

#[cw_serde]
pub enum ExecuteMsg {
    SampleExecute { },

    Admin(AdminExecuteMsg),
}

#[cw_serde]
pub enum MigrateMsg {
    
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetAdminResponse)]
    GetAdmin {},

    // GetCount returns the current count as a json-encoded number
    #[returns(SampleQueryResponse)]
    SampleQuery {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct SampleQueryResponse {
    pub value: bool,
}

#[cw_serde]
pub struct GetAdminResponse {
    pub admin: Option<Addr>,
}
