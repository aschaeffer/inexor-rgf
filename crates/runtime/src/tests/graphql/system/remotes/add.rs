use async_graphql::Request;
use std::sync::Arc;

use serde::Deserialize;

use crate::api::GraphQLQueryService;
use crate::model_runtime::InstanceAddress;
use crate::model_runtime::InstanceInfo;
use crate::runtime::Runtime;
use crate::runtime::RuntimeBuilder;
use crate::tests::graphql::system::remotes::util::address_to_vars;

#[derive(Deserialize, Debug)]
struct Mutation {
    system: MutationSystem,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MutationSystem {
    remotes: MutationSystemAddRemote,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MutationSystemAddRemote {
    add: InstanceInfo,
}

#[tokio::test(flavor = "multi_thread")]
async fn test_add_remote() {
    RuntimeBuilder::new()
        .ignore_config_files()
        .disable_all_plugins(true)
        .pick_free_port()
        .init()
        .await
        .post_init()
        .await
        .spawn()
        .await
        .with_runtime(|runtime: Arc<dyn Runtime>| async move {
            let query_service = runtime.get_graphql_query_service();
            let instance_service = runtime.get_instance_service();
            let remotes_manager = runtime.get_remotes_manager();
            remotes_manager.remove_all();

            // RT: Expect that no remotes exists
            assert_eq!(remotes_manager.get_all().len(), 0);

            // GQL: Add self as remote
            let address = instance_service.get_instance_info().address();
            let instance_info = mutation_add_remote(&query_service, &address).await;

            // Compare local instance info with returned instance info
            assert_eq!(instance_service.get_instance_info(), instance_info);

            // RT: Expect that one remote exist
            assert_eq!(remotes_manager.get_all().len(), 1);
        })
        .await
        .pre_shutdown()
        .await
        .shutdown()
        .await;
}

const MUTATION_ADD_REMOTE: &str = include_str!("../../../../../graphql/system/remotes/add.graphql");

async fn mutation_add_remote(query_service: &Arc<dyn GraphQLQueryService>, address: &InstanceAddress) -> InstanceInfo {
    let request = Request::new(MUTATION_ADD_REMOTE).variables(address_to_vars(address));
    let response = query_service.execute(request).await;
    assert!(response.errors.is_empty());
    let data = response.data.into_json().expect("Failed to get json data from graphql response");
    println!("{}", data);
    let data: Mutation = serde_json::from_value(data).expect("Failed to deserialize json into target data model");
    data.system.remotes.add
}
