use cosmwasm_contract_template::msg::*;
use cosmwasm_std::{Addr, Coin};
use cw_multi_test::{App, Executor};
use robot_code_gen::Robot;

pub trait CosmwasmContractTemplateRobot {
    fn sample_execute(&mut self, app: &mut App, contract: &Addr, caller: &Addr, funds: Vec<Coin>);
    fn get_admin(&self, app: &App, contract: &Addr) -> GetAdminResponse;
    fn sample_query(&self, app: &App, contract: &Addr) -> SampleQueryResponse;
}

impl CosmwasmContractTemplateRobot for Robot {
    fn sample_execute(&mut self, app: &mut App, contract: &Addr, caller: &Addr, funds: Vec<Coin>) {
        let msg = ExecuteMsg::SampleExecute {};
        app.execute_contract(caller.to_owned(), contract.to_owned(), &msg, &funds)
            .unwrap();
    }
    fn get_admin(&self, app: &App, contract: &Addr) -> GetAdminResponse {
        let msg = QueryMsg::GetAdmin {};
        return app
            .wrap()
            .query_wasm_smart(contract.to_owned(), &msg)
            .unwrap();
    }
    fn sample_query(&self, app: &App, contract: &Addr) -> SampleQueryResponse {
        let msg = QueryMsg::SampleQuery {};
        return app
            .wrap()
            .query_wasm_smart(contract.to_owned(), &msg)
            .unwrap();
    }
}
