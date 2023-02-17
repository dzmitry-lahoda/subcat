use std::{sync::Arc, vec, collections::HashMap};

use jsonrpsee_core::{
    client::ClientT, rpc_params, Serialize, __reexports::serde::Deserialize, traits::ToRpcParams, JsonValue,
};
use jsonrpsee_ws_client::WsClientBuilder;
// use pallet_transaction_payment_rpc::{
//     TransactionPayment, TransactionPaymentApiClient, TransactionPaymentRuntimeApi,
// };
// //use serde::{Deserialize, Serialize};
// use pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo;
use sp_core::Bytes;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcMethods {
    pub methods: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryInfo {
    pub encoded_xt: Bytes,
    pub at: Option<sp_core::H256>,
}

impl ToRpcParams for QueryInfo {
    fn to_rpc_params(
        self,
    ) -> Result<Option<Box<jsonrpsee_core::JsonRawValue>>, jsonrpsee_core::Error> {
        Ok(Some(
            jsonrpsee_core::JsonRawValue::from_string(serde_json::to_string(&self).unwrap())
                .unwrap(),
        ))
    }
}

// struct Dummy;
// impl jsonrpsee_core::client::ClientT for Dummy {
//     fn notification<'life0,'life1,'async_trait,Params, >(&'life0 self,method: &'life1 str,params:Params) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<(),jsonrpsee_core::Error> > + core::marker::Send+'async_trait> >where Params:jsonrpsee_core::traits::ToRpcParams+Send,Params:'async_trait+ ,'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
//         todo!()
//     }

//     fn request<'life0,'life1,'async_trait,R,Params, >(&'life0 self,method: &'life1 str,params:Params) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<R,jsonrpsee_core::Error> > + core::marker::Send+'async_trait> >where R:jsonrpsee_core::DeserializeOwned,Params:jsonrpsee_core::traits::ToRpcParams+Send,R:'async_trait+ ,Params:'async_trait+ ,'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
//         todo!()
//     }

//     fn batch_request<'a,'life0,'async_trait,R, >(&'life0 self,batch:jsonrpsee_core::params::BatchRequestBuilder<'a>) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<jsonrpsee_core::client::BatchResponse<'a,R> ,jsonrpsee_core::Error> > + core::marker::Send+'async_trait> >where R:jsonrpsee_core::DeserializeOwned+std::fmt::Debug+'a,'a:'async_trait+ ,R:'async_trait+ ,'life0:'async_trait,Self:'async_trait {
//         todo!()
//     }
// }

use subxt::{dynamic::Value, tx::PairSigner,  OnlineClient, PolkadotConfig, SubstrateConfig};
use codec::Encode;
use sp_core::sr25519::Pair;
use sp_keyring::AccountKeyring;

#[tokio::main]
async fn main() {
    let client = WsClientBuilder::default()
        .build("wss://statemine-rpc.dwellir.com:443")
        .await
        .unwrap();
    let client = Arc::new(client);
    let result: RpcMethods = client.request("rpc_methods", rpc_params![]).await.unwrap();

    println!("Hello, world! {:?}", result);

    let api = OnlineClient::<SubstrateConfig>::from_url("wss://statemine-rpc.dwellir.com:443").await.unwrap();
    
    let alice = pair_signer(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id();
    let tx = subxt::dynamic::tx(
        "Balances",
        "transfer",
        vec![
            // A value representing a MultiAddress<AccountId32, _>. We want the "Id" variant, and that
            // will ultimately contain the bytes for our destination address (there is a new type wrapping
            // the address, but our encoding will happily ignore such things and do it's best to line up what
            // we provide with what it needs).
            Value::unnamed_variant("Id", [Value::from_bytes(&dest)]),
            // A value representing the amount we'd like to transfer.
            Value::u128(123_456_789_012_345),
        ],
    );

    let encoded_xt : Vec<_> = api.tx().create_signed(&tx, &alice, <_>::default()).await.unwrap().into_encoded();

    let result: JsonValue = client
        .request(
            "payment_queryInfo",
            QueryInfo {
                at: None,
                encoded_xt: Bytes(encoded_xt),
            },
        )
        .await
        .unwrap();
    println!("Hello, world! {:?}", result);

    //let transaction_payment = TransactionPayment::new(client.clone());
    //let transaction_payment: Dummy = panic!(); // = TransactionPayment::new(client.clone());
    // TransactionPaymentApiClient::<sp_core::H256, RuntimeDispatchInfo<u128>>::query_info(
    //     &transaction_payment,
    //     panic!(),
    //     panic!(),
    // )
    // .await
    // .unwrap();
    // //let result : RpcMethods = client.request("payment_queryInfo", rpc_params![]).await.unwrap();
}



pub fn pair_signer(pair: Pair) -> PairSigner<SubstrateConfig, Pair> {
    PairSigner::new(pair)
}