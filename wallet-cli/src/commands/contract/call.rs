//! Subcommand to call a contract.

use clap::ArgMatches;
use hex::{FromHex, ToHex};
use keys::Address;
use proto::api_grpc::Wallet;
use proto::core::TriggerSmartContract;
use serde_json::json;

use crate::error::Error;
use crate::utils::abi;
use crate::utils::client;
use crate::utils::trx;
use crate::utils::jsont;

pub fn main(matches: &ArgMatches) -> Result<(), Error> {
    let sender = matches
        .value_of("SENDER")
        .and_then(|s| s.parse::<Address>().ok())
        .ok_or(Error::Runtime("wrong sender address format"))?;
    let contract = matches
        .value_of("CONTRACT")
        .and_then(|s| s.parse::<Address>().ok())
        .ok_or(Error::Runtime("wrong contract address format"))?;
    let method = matches.value_of("METHOD").expect("required in cli.yml; qed");

    let data = match (matches.values_of("ARGS"), matches.value_of("data")) {
        (Some(args), None) => {
            let types = extract_types(method)?;
            if matches.occurrences_of("ARGS") as usize != types.len() {
                return Err(Error::Runtime("wrong number of ARGS"));
            }
            // Fix tron base58checked addresses, remove 0x41
            let values = args
                .zip(types.iter())
                .map(|(arg, ty)| {
                    if ty == &"address" {
                        arg.parse::<Address>()
                            .map(|addr| addr.encode_hex::<String>()[2..].to_owned())
                            .map_err(Error::from)
                    } else {
                        Ok(arg.to_owned())
                    }
                })
                .collect::<Result<Vec<_>, Error>>()?;
            let mut data = (&abi::fnhash(method)[..]).to_owned();
            data.append(&mut abi::encode_params(&types, &values)?);
            eprintln!("! data = {:}", data.encode_hex::<String>());
            data
        }
        (None, Some(data_hex)) => Vec::from_hex(data_hex)?,
        // nullary call
        (None, None) => Vec::from(&abi::fnhash(method)[..]),
        (_, _) => unreachable!("set conflicts in cli.yml; qed"),
    };

    let mut trigger_contract = TriggerSmartContract {
        owner_address: sender.to_bytes().to_owned(),
        contract_address: contract.to_bytes().to_owned(),
        data: data.into(),
        ..Default::default()
    };

    if let Some(value) = matches.value_of("value") {
        trigger_contract.set_call_value(trx::parse_amount_with_surfix(value, "TRX", 6)?);
    }

    if let Some(token_id) = matches.value_of("token-id") {
        let value = matches.value_of("token-value").expect("constraint in cli.yml; qed");
        trigger_contract.set_token_id(token_id.parse()?);
        trigger_contract.set_call_token_value(trx::parse_amount(value)?);
    }

    if matches.is_present("const") {
        let (_, resp, _) = client::new_grpc_client()?
            .trigger_constant_contract(Default::default(), trigger_contract)
            .wait()?;
        let mut trx_ext = serde_json::to_value(&resp)?;
        jsont::fix_transaction_ext(&mut trx_ext)?;
        let ret = json!({
            "result": trx_ext["result"],
            "constant_result": trx_ext["constant_result"],
        });
        println!("{:}", serde_json::to_string_pretty(&ret)?);
        Ok(())
    } else {
        trx::TransactionHandler::handle(trigger_contract, matches)
            .map_raw_transaction(|raw| raw.set_fee_limit(1_000_000))
            .run()
    }
}

#[inline]
fn extract_types(fnname: &str) -> Result<Vec<&str>, Error> {
    let start = fnname.find('(').ok_or(Error::Runtime("malformed method name"))?;
    let end = fnname.find(')').ok_or(Error::Runtime("malformed method name"))?;
    Ok(fnname[start + 1..end].split(",").filter(|ty| !ty.is_empty()).collect())
}
