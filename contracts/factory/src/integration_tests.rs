#[cfg(test)]
mod tests {
    use crate::helpers::CwTemplateContract;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::Addr;
    use cw_multi_test::{Contract, ContractWrapper, Executor};
    use sg_multi_test::StargazeApp;
    use sg_std::StargazeMsgWrapper;

    pub fn contract_template() -> Box<dyn Contract<StargazeMsgWrapper>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const USER: &str = "USER";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "denom";

    // fn mock_app() -> StargazeApp {
    //     AppBuilder::new().build(|router, _, storage| {
    //         router
    //             .bank
    //             .init_balance(
    //                 storage,
    //                 &Addr::unchecked(USER),
    //                 vec![Coin {
    //                     denom: NATIVE_DENOM.to_string(),
    //                     amount: Uint128::new(1),
    //                 }],
    //             )
    //             .unwrap();
    //     })
    // }

    fn custom_mock_app() -> StargazeApp {
        StargazeApp::default()
    }

    fn proper_instantiate() -> (StargazeApp, CwTemplateContract) {
        let mut app = custom_mock_app();
        let cw_template_id = app.store_code(contract_template());

        let msg = InstantiateMsg {};
        let cw_template_contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "test",
                None,
            )
            .unwrap();

        let cw_template_contract = CwTemplateContract(cw_template_contract_addr);

        (app, cw_template_contract)
    }

    // mod count {
    //     use super::*;
    //     use crate::msg::ExecuteMsg;

    //     #[test]
    //     fn count() {
    //         let (mut app, cw_template_contract) = proper_instantiate();

    //         let msg = ExecuteMsg::Increment {};
    //         let cosmos_msg = cw_template_contract.call(msg).unwrap();
    //         app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
    //     }
    // }
}
