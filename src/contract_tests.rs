#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier};
    use cosmwasm_std::{
        coins, MemoryStorage, OwnedDeps,
    };


    use crate::contract::instantiate;
    use crate::instantiation::msg::InstantiateMsg;

    const TEST_DENOM: &str = "uusd";
    const TEST_CREATOR: &str = "creator";
    const _TEST_USER: &str = "user";
    const _TEST_USER2: &str = "user2";

    const _TEST_PRICE: u64 = 10000000;

    const _TEST_INVALID_DENOM: &str = "notuusd";

    fn instantiate_contract() -> OwnedDeps<MemoryStorage, MockApi, MockQuerier> {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info(TEST_CREATOR, &coins(1000, TEST_DENOM));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        return deps;
    }

    #[test]
    fn instantiate_success() {
        let _deps = instantiate_contract();
    }
}
