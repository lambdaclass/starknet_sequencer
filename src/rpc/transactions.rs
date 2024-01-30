use crate::rpc::{BroadcastedTransaction, BroadcastedDeclareTransaction, BroadcastedDeployAccountTransaction, BroadcastedInvokeTransaction};
use starknet_in_rust::{
    transaction::{DeployAccount, InvokeFunction, Transaction, Declare, DeclareV2},
    utils::{Address, ClassHash},
    Felt252,
};

// This must be removed when merged to "Added primitive store" PR
use starknet_core::types::FieldElement;

pub fn to_felt252(field_element: FieldElement) -> Felt252 {
    Felt252::from_bytes_be(&field_element.to_bytes_be())
}

fn convert_to_invoke_transaction(invoke: BroadcastedInvokeTransaction) -> Transaction {
    Transaction::InvokeFunction(InvokeFunction::new(
        Address(to_felt252(invoke.calldata
            .first()
            .expect("Invoke does not contain contract address")
            .clone())),
        to_felt252(invoke.calldata
            .get(1)
            .expect("Invoke does not contain function selector")
            .clone()),
        invoke.max_fee.try_into().unwrap(),
        Felt252::ZERO,
        invoke.calldata[2..]
            .iter()
            .map(|&e| to_felt252(e))
            .collect::<Vec<_>>(),
        invoke.signature.iter().map(|&e| to_felt252(e)).collect::<Vec<_>>(),
        Felt252::ZERO,
        Some(to_felt252(invoke.nonce)),
    )
    .unwrap())
}

fn convert_deploy_transaction(deploy: BroadcastedDeployAccountTransaction) -> Transaction {
    Transaction::DeployAccount(DeployAccount::new(
        ClassHash::from(to_felt252(deploy.class_hash)),
        deploy.max_fee.try_into().unwrap(),
        Felt252::ZERO,
        to_felt252(deploy.nonce),
        deploy.constructor_calldata
            .iter()
            .map(|&e| to_felt252(e))
            .collect::<Vec<_>>(),
        deploy.signature.iter().map(|&e| to_felt252(e)).collect::<Vec<_>>(),
        to_felt252(deploy.contract_address_salt),
        Felt252::ZERO,
    )
    .unwrap())
}

fn convert_declare_transaction(declare: BroadcastedDeclareTransaction) -> Transaction {
    match declare {
        BroadcastedDeclareTransaction::V1(declare) => Transaction::Declare(Declare::new(
            declare.contract_class,
            Felt252::ZERO,
            Address(to_felt252(declare.sender_address)),
            declare.max_fee.try_into().unwrap(),
            Felt252::ONE,
            declare.signature.iter().map(|&e| to_felt252(e)).collect::<Vec<_>>(),
            to_felt252(declare.nonce),
        )
        .unwrap()),
        BroadcastedDeclareTransaction::V2(declare) => Transaction::DeclareV2(Box::new(
            DeclareV2::new(
                declare.contract_class,
                casm_contract_class,
                to_felt252(declare.compiled_class_hash),
                Felt252::ZERO,
                Address(to_felt252(declare.sender_address)),
                declare.max_fee.try_into().unwrap(),
                Felt252::TWO,
                declare.signature.iter().map(|&e| to_felt252(e)).collect::<Vec<_>>(),
                to_felt252(declare.nonce),
            )
            .unwrap(),
        )),
    }
}

pub fn convert_to_transaction(broadcasted_tx: BroadcastedTransaction) -> Transaction {
    match broadcasted_tx {
        BroadcastedTransaction::Invoke(invoke) => convert_to_invoke_transaction(invoke),
        BroadcastedTransaction::DeployAccount(deploy) => convert_deploy_transaction(deploy),
        BroadcastedTransaction::Declare(declare) => convert_declare_transaction(declare),
    }
}
