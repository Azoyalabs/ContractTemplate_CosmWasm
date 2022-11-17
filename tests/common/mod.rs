#[cfg(test)]
pub mod test_utils {
    use cosmwasm_std::Empty;
    use cw_multi_test::{ContractWrapper, Contract};
    
    // You'll need to change the lib name here
    use cosmwasm_contract_template::contract::{instantiate, execute, query};
    
    pub fn get_contract() -> Box<dyn Contract<Empty>> {
        let contract =
            ContractWrapper::new_with_empty(execute, instantiate, query); //.with_reply(reply);
        Box::new(contract)
    }
}

