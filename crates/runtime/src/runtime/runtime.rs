use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

use async_trait::async_trait;
use log::debug;
use log::info;
use tokio::time::error::Elapsed;

use crate::api::*;
use crate::config::InstanceAddress;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;

#[wrapper]
pub struct RunningState(RwLock<bool>);

#[provides]
fn create_running_state_wrapper() -> RunningState {
    RunningState(RwLock::new(false))
}

#[async_trait]
pub trait Runtime: Send + Sync {
    async fn config(&self);

    //  + Lifecycle
    async fn init(&self);

    async fn post_init(&self);

    async fn pre_shutdown(&self);

    async fn shutdown(&self);

    async fn run(&self);

    fn stop(&self);

    fn is_running(&self) -> bool;

    /// Waits for the GraphQL server to be started.
    /// Times out if the GraphQL server is not running after the given duration.
    async fn wait_for_started(&self, timeout_duration: Duration) -> Result<(), Elapsed>;

    /// Waits for the GraphQL server has been stopped.
    async fn wait_for_stopped(&self);

    /// Waits for the GraphQL server has been stopped.
    /// Times out if the GraphQL server is still running after the given duration.
    async fn wait_for_stopped_with_timeout(&self, timeout_duration: Duration) -> Result<(), Elapsed>;

    /// Returns the address of the runtime.
    fn address(&self) -> InstanceAddress;

    fn get_command_manager(&self) -> Arc<dyn CommandManager>;

    fn get_component_manager(&self) -> Arc<dyn ComponentManager>;

    fn get_component_import_export_manager(&self) -> Arc<dyn ComponentImportExportManager>;

    fn get_component_provider_registry(&self) -> Arc<dyn ComponentProviderRegistry>;

    fn get_config_manager(&self) -> Arc<dyn ConfigManager>;

    fn get_dynamic_graph_query_service(&self) -> Arc<dyn DynamicGraphQueryService>;

    fn get_dynamic_graph_schema_manager(&self) -> Arc<dyn DynamicGraphSchemaManager>;

    fn get_entity_behaviour_manager(&self) -> Arc<dyn EntityBehaviourManager>;

    fn get_entity_behaviour_registry(&self) -> Arc<dyn EntityBehaviourRegistry>;

    fn get_entity_component_behaviour_manager(&self) -> Arc<dyn EntityComponentBehaviourManager>;

    fn get_entity_component_behaviour_registry(&self) -> Arc<dyn EntityComponentBehaviourRegistry>;

    fn get_entity_instance_import_export_manager(&self) -> Arc<dyn EntityInstanceImportExportManager>;

    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager>;

    fn get_entity_type_import_export_manager(&self) -> Arc<dyn EntityTypeImportExportManager>;

    fn get_entity_type_provider_registry(&self) -> Arc<dyn EntityTypeProviderRegistry>;

    fn get_event_manager(&self) -> Arc<dyn SystemEventManager>;

    fn get_flow_type_manager(&self) -> Arc<dyn FlowTypeManager>;

    fn get_flow_type_import_export_manager(&self) -> Arc<dyn FlowTypeImportExportManager>;

    fn get_flow_type_provider_registry(&self) -> Arc<dyn FlowTypeProviderRegistry>;

    // TODO: fn get_flow_instance_manager(&self) -> Arc<dyn FlowInstanceManager>;

    // TODO: fn get_flow_instance_import_export_manager(&self) -> Arc<dyn FlowInstanceImportExportManager>;

    fn get_graphql_query_service(&self) -> Arc<dyn GraphQLQueryService>;

    fn get_graphql_schema_manager(&self) -> Arc<dyn GraphQLSchemaManager>;

    fn get_graphql_server(&self) -> Arc<dyn GraphQLServer>;

    fn get_instance_service(&self) -> Arc<dyn InstanceService>;

    fn get_namespace_manager(&self) -> Arc<dyn NamespaceManager>;

    fn get_plugin_container_manager(&self) -> Arc<dyn PluginContainerManager>;

    fn get_plugin_repository_manager(&self) -> Arc<dyn PluginRepositoryManager>;

    fn get_reactive_entity_manager(&self) -> Arc<dyn ReactiveEntityManager>;

    fn get_reactive_flow_manager(&self) -> Arc<dyn ReactiveFlowManager>;

    fn get_reactive_relation_manager(&self) -> Arc<dyn ReactiveRelationManager>;

    fn get_relation_behaviour_manager(&self) -> Arc<dyn RelationBehaviourManager>;

    fn get_relation_behaviour_registry(&self) -> Arc<dyn RelationBehaviourRegistry>;

    fn get_relation_component_behaviour_manager(&self) -> Arc<dyn RelationComponentBehaviourManager>;

    fn get_relation_component_behaviour_registry(&self) -> Arc<dyn RelationComponentBehaviourRegistry>;

    fn get_relation_instance_import_export_manager(&self) -> Arc<dyn RelationInstanceImportExportManager>;

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager>;

    fn get_relation_type_import_export_manager(&self) -> Arc<dyn RelationTypeImportExportManager>;

    fn get_relation_type_provider_registry(&self) -> Arc<dyn RelationTypeProviderRegistry>;

    fn get_remotes_manager(&self) -> Arc<dyn RemotesManager>;

    fn get_shutdown_manager(&self) -> Arc<dyn ShutdownManager>;

    fn get_web_resource_manager(&self) -> Arc<dyn WebResourceManager>;
}

#[component]
pub struct RuntimeImpl {
    running: RunningState,

    command_manager: Wrc<dyn CommandManager>,
    component_manager: Wrc<dyn ComponentManager>,
    component_import_export_manager: Wrc<dyn ComponentImportExportManager>,
    component_provider_registry: Wrc<dyn ComponentProviderRegistry>,
    config_manager: Wrc<dyn ConfigManager>,
    dynamic_graph_query_service: Wrc<dyn DynamicGraphQueryService>,
    dynamic_graph_schema_manager: Wrc<dyn DynamicGraphSchemaManager>,
    event_manager: Wrc<dyn SystemEventManager>,
    entity_behaviour_manager: Wrc<dyn EntityBehaviourManager>,
    entity_behaviour_registry: Wrc<dyn EntityBehaviourRegistry>,
    entity_component_behaviour_manager: Wrc<dyn EntityComponentBehaviourManager>,
    entity_component_behaviour_registry: Wrc<dyn EntityComponentBehaviourRegistry>,
    entity_instance_import_export_manager: Wrc<dyn EntityInstanceImportExportManager>,
    entity_type_manager: Wrc<dyn EntityTypeManager>,
    entity_type_import_export_manager: Wrc<dyn EntityTypeImportExportManager>,
    entity_type_provider_registry: Wrc<dyn EntityTypeProviderRegistry>,
    flow_type_manager: Wrc<dyn FlowTypeManager>,
    flow_type_import_export_manager: Wrc<dyn FlowTypeImportExportManager>,
    flow_type_provider_registry: Wrc<dyn FlowTypeProviderRegistry>,
    // TODO: flow_instance_manager: Wrc<dyn FlowInstanceManager>,
    // TODO: flow_instance_import_export_manager: Wrc<dyn FlowInstanceImportExportManager>,
    graphql_query_service: Wrc<dyn GraphQLQueryService>,
    graphql_schema_manager: Wrc<dyn GraphQLSchemaManager>,
    graphql_server: Wrc<dyn GraphQLServer>,
    instance_service: Wrc<dyn InstanceService>,
    namespace_manager: Wrc<dyn NamespaceManager>,
    shutdown_manager: Wrc<dyn ShutdownManager>,
    reactive_entity_manager: Wrc<dyn ReactiveEntityManager>,
    reactive_relation_manager: Wrc<dyn ReactiveRelationManager>,
    reactive_flow_manager: Wrc<dyn ReactiveFlowManager>,
    relation_behaviour_manager: Wrc<dyn RelationBehaviourManager>,
    relation_behaviour_registry: Wrc<dyn RelationBehaviourRegistry>,
    relation_component_behaviour_manager: Wrc<dyn RelationComponentBehaviourManager>,
    relation_component_behaviour_registry: Wrc<dyn RelationComponentBehaviourRegistry>,
    relation_instance_import_export_manager: Wrc<dyn RelationInstanceImportExportManager>,
    relation_type_manager: Wrc<dyn RelationTypeManager>,
    relation_type_import_export_manager: Wrc<dyn RelationTypeImportExportManager>,
    relation_type_provider_registry: Wrc<dyn RelationTypeProviderRegistry>,
    remotes_manager: Wrc<dyn RemotesManager>,
    runtime_types_provider: Wrc<dyn RuntimeTypesProvider>,
    plugin_container_manager: Wrc<dyn PluginContainerManager>,
    plugin_context_factory: Wrc<dyn PluginContextFactory>,
    plugin_repository_manager: Wrc<dyn PluginRepositoryManager>,
    plugin_resolver: Wrc<dyn PluginResolver>,
    web_resource_manager: Wrc<dyn WebResourceManager>,
}

#[async_trait]
#[provides]
impl Runtime for RuntimeImpl {
    async fn config(&self) {
        self.config_manager.init().await;
    }

    async fn init(&self) {
        // Order matters
        // Type System
        self.component_manager.init().await;
        self.entity_type_manager.init().await;
        self.relation_type_manager.init().await;
        self.flow_type_manager.init().await;
        // Type Providers
        self.component_provider_registry.init().await;
        self.entity_type_provider_registry.init().await;
        self.relation_type_provider_registry.init().await;
        self.flow_type_provider_registry.init().await;
        self.runtime_types_provider.init().await;
        // Plugin System
        self.plugin_context_factory.init().await;
        self.plugin_repository_manager.init().await;
        self.plugin_resolver.init().await;
        // Instance System
        self.reactive_flow_manager.init().await;
        // GraphQL API
        self.web_resource_manager.init().await;
        self.graphql_schema_manager.init().await;
        self.graphql_query_service.init().await;
        self.graphql_server.init().await;
        // System Layer
        self.shutdown_manager.init().await;
        self.event_manager.init().await;
        self.reactive_entity_manager.init().await;
        self.remotes_manager.init().await;
        self.command_manager.init().await;
        // Dynamic Graph API
        self.dynamic_graph_schema_manager.init().await;
        self.dynamic_graph_query_service.init().await;
    }

    async fn post_init(&self) {
        // Order matters
        // Type System
        self.component_manager.post_init().await;
        self.entity_type_manager.post_init().await;
        self.runtime_types_provider.post_init().await;
        self.relation_type_manager.post_init().await;
        self.flow_type_manager.post_init().await;
        // Type Providers
        self.component_provider_registry.post_init().await;
        self.entity_type_provider_registry.post_init().await;
        self.relation_type_provider_registry.post_init().await;
        self.flow_type_provider_registry.post_init().await;
        // Plugin System
        self.plugin_context_factory.post_init().await;
        self.plugin_repository_manager.post_init().await;
        self.plugin_resolver.post_init().await;
        // Instance System
        self.reactive_flow_manager.post_init().await;
        // GraphQL API
        self.web_resource_manager.post_init().await;
        self.graphql_schema_manager.post_init().await;
        self.graphql_query_service.post_init().await;
        self.graphql_server.post_init().await;
        // System Layer
        self.shutdown_manager.post_init().await;
        self.event_manager.post_init().await;
        self.reactive_entity_manager.post_init().await; // after event_manager!
        self.remotes_manager.post_init().await;
        self.command_manager.post_init().await;
        // Dynamic Graph API
        self.dynamic_graph_schema_manager.post_init().await;
        self.dynamic_graph_query_service.post_init().await;
    }

    async fn pre_shutdown(&self) {
        // Reverse order matters
        // Dynamic Graph API
        self.dynamic_graph_query_service.pre_shutdown().await;
        self.dynamic_graph_schema_manager.pre_shutdown().await;
        // System Layer
        self.command_manager.pre_shutdown().await;
        self.remotes_manager.pre_shutdown().await;
        self.reactive_entity_manager.pre_shutdown().await;
        self.event_manager.pre_shutdown().await;
        self.shutdown_manager.pre_shutdown().await;
        // GraphQL API
        self.graphql_server.pre_shutdown().await;
        self.graphql_query_service.pre_shutdown().await;
        self.graphql_schema_manager.pre_shutdown().await;
        self.web_resource_manager.pre_shutdown().await;
        // Instance System
        self.reactive_flow_manager.pre_shutdown().await;
        // Plugin System
        self.plugin_resolver.pre_shutdown().await;
        self.plugin_repository_manager.pre_shutdown().await;
        self.plugin_context_factory.pre_shutdown().await;
        // Type Providers
        self.runtime_types_provider.pre_shutdown().await;
        self.flow_type_provider_registry.post_init().await;
        self.relation_type_provider_registry.post_init().await;
        self.entity_type_provider_registry.post_init().await;
        self.component_provider_registry.post_init().await;
        // Type System
        self.flow_type_manager.post_init().await;
        self.relation_type_manager.pre_shutdown().await;
        self.entity_type_manager.pre_shutdown().await;
        self.component_manager.pre_shutdown().await;
    }

    async fn shutdown(&self) {
        // Reverse order matters
        // Dynamic Graph API
        self.dynamic_graph_query_service.shutdown().await;
        self.dynamic_graph_schema_manager.shutdown().await;
        // System Layer
        self.command_manager.shutdown().await;
        self.remotes_manager.shutdown().await;
        self.reactive_entity_manager.shutdown().await;
        self.event_manager.shutdown().await;
        self.shutdown_manager.shutdown().await;
        // GraphQL API
        self.graphql_server.shutdown().await;
        self.graphql_query_service.shutdown().await;
        self.graphql_schema_manager.shutdown().await;
        self.web_resource_manager.shutdown().await;
        // Instance System
        self.reactive_flow_manager.shutdown().await;
        // Plugin System
        self.plugin_resolver.shutdown().await;
        self.plugin_repository_manager.shutdown().await;
        self.plugin_context_factory.shutdown().await;
        // Type Providers
        self.runtime_types_provider.shutdown().await;
        self.flow_type_provider_registry.post_init().await;
        self.relation_type_provider_registry.post_init().await;
        self.entity_type_provider_registry.post_init().await;
        self.component_provider_registry.post_init().await;
        // Type System
        self.flow_type_manager.post_init().await;
        self.relation_type_manager.shutdown().await;
        self.entity_type_manager.shutdown().await;
        self.component_manager.shutdown().await;
    }

    async fn run(&self) {
        // Signal handling
        let terminate = Arc::new(AtomicBool::new(false));
        // This channel allows the main thread to stop the GraphQL server thread
        let (graphql_server_stop_sender, graphql_server_stop_receiver) = crossbeam::channel::unbounded::<()>();
        // This channel allows the GraphQL server thread to tell the main thread that it has been finished
        let (graphql_server_stopped_sender, graphql_server_stopped_receiver) = crossbeam::channel::unbounded::<()>();
        // Clone GraphQL server and move the reference into the GraphQL server thread
        let graphql_server = self.graphql_server.clone();
        // GraphQL server thread: Create a new thread for the GraphQL server
        // TODO: add thread name
        let graphql_server_handle = tokio::spawn(async move {
            // Run the GraphQL server
            info!("Run the GraphQL server.");
            graphql_server.serve(graphql_server_stop_receiver).await;
            debug!("Successfully stopped GraphQL Server.");
            // Tell the main thread, that the GraphQL server thread has finished
            let _result = graphql_server_stopped_sender.send(());
        });

        {
            let mut running = self.running.0.write().unwrap();
            *running = true;
        }

        {
            let _r_sigint = signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&terminate));
            let _r_sigterm = signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&terminate));

            let mut stopping = false;
            while self.is_running() && !stopping && !terminate.load(Ordering::Relaxed) {
                tokio::time::sleep(Duration::from_millis(100)).await;
                let r = graphql_server_stopped_receiver.try_recv();
                if r.is_ok() {
                    debug!("Stopping the main thread");
                    stopping = true;
                }
                if self.shutdown_manager.is_shutdown() {
                    stopping = true;
                }
            }
        } // Drop "running"

        // Stop GraphQL server thread, if it is still running
        debug!("Stopping the GraphQL server thread");
        let _graphql_server_stop_result = graphql_server_stop_sender.send_timeout((), Duration::from_millis(100));

        // Be sure that the GraphQL server thread is gone
        let _ = graphql_server_handle.await;
        info!("Bye.");

        // Ensure the running state is now set to false even if the loop was terminated
        // externally because the running state is checked from outside.
        {
            let mut running = self.running.0.write().unwrap();
            *running = false;
        }
    }

    fn stop(&self) {
        {
            let mut running = self.running.0.write().unwrap();
            *running = false;
        }
    }

    fn is_running(&self) -> bool {
        *self.running.0.read().unwrap().deref()
    }

    async fn wait_for_started(&self, timeout_duration: Duration) -> Result<(), Elapsed> {
        tokio::time::timeout(timeout_duration, self.wait_for_started_internal()).await
    }

    async fn wait_for_stopped(&self) {
        while self.is_running() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    async fn wait_for_stopped_with_timeout(&self, timeout_duration: Duration) -> Result<(), Elapsed> {
        tokio::time::timeout(timeout_duration, self.wait_for_stopped()).await
    }

    fn address(&self) -> InstanceAddress {
        self.instance_service.get_instance_info().address()
    }

    fn get_command_manager(&self) -> Arc<dyn CommandManager> {
        self.command_manager.clone()
    }

    fn get_component_manager(&self) -> Arc<dyn ComponentManager> {
        self.component_manager.clone()
    }

    fn get_component_import_export_manager(&self) -> Arc<dyn ComponentImportExportManager> {
        self.component_import_export_manager.clone()
    }

    fn get_component_provider_registry(&self) -> Arc<dyn ComponentProviderRegistry> {
        self.component_provider_registry.clone()
    }

    fn get_config_manager(&self) -> Arc<dyn ConfigManager> {
        self.config_manager.clone()
    }

    fn get_dynamic_graph_query_service(&self) -> Arc<dyn DynamicGraphQueryService> {
        self.dynamic_graph_query_service.clone()
    }

    fn get_dynamic_graph_schema_manager(&self) -> Arc<dyn DynamicGraphSchemaManager> {
        self.dynamic_graph_schema_manager.clone()
    }

    fn get_entity_behaviour_manager(&self) -> Arc<dyn EntityBehaviourManager> {
        self.entity_behaviour_manager.clone()
    }

    fn get_entity_behaviour_registry(&self) -> Arc<dyn EntityBehaviourRegistry> {
        self.entity_behaviour_registry.clone()
    }

    fn get_entity_component_behaviour_manager(&self) -> Arc<dyn EntityComponentBehaviourManager> {
        self.entity_component_behaviour_manager.clone()
    }

    fn get_entity_component_behaviour_registry(&self) -> Arc<dyn EntityComponentBehaviourRegistry> {
        self.entity_component_behaviour_registry.clone()
    }

    fn get_entity_instance_import_export_manager(&self) -> Arc<dyn EntityInstanceImportExportManager> {
        self.entity_instance_import_export_manager.clone()
    }

    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager> {
        self.entity_type_manager.clone()
    }

    fn get_entity_type_import_export_manager(&self) -> Arc<dyn EntityTypeImportExportManager> {
        self.entity_type_import_export_manager.clone()
    }

    fn get_entity_type_provider_registry(&self) -> Arc<dyn EntityTypeProviderRegistry> {
        self.entity_type_provider_registry.clone()
    }

    fn get_event_manager(&self) -> Arc<dyn SystemEventManager> {
        self.event_manager.clone()
    }

    fn get_flow_type_manager(&self) -> Arc<dyn FlowTypeManager> {
        self.flow_type_manager.clone()
    }

    fn get_flow_type_import_export_manager(&self) -> Arc<dyn FlowTypeImportExportManager> {
        self.flow_type_import_export_manager.clone()
    }

    fn get_flow_type_provider_registry(&self) -> Arc<dyn FlowTypeProviderRegistry> {
        self.flow_type_provider_registry.clone()
    }

    // TODO: get_flow_instance_manager
    // fn get_flow_instance_manager(&self) -> Arc<dyn FlowInstanceManager> {
    //     self.flow_instance_manager.clone()
    // }

    // TODO: get_flow_instance_import_export_manager
    // fn get_flow_instance_import_export_manager(&self) -> Arc<dyn FlowInstanceImportExportManager> {
    //     self.flow_instance_import_export_manager.clone()
    // }

    fn get_graphql_query_service(&self) -> Arc<dyn GraphQLQueryService> {
        self.graphql_query_service.clone()
    }

    fn get_graphql_schema_manager(&self) -> Arc<dyn GraphQLSchemaManager> {
        self.graphql_schema_manager.clone()
    }

    fn get_graphql_server(&self) -> Arc<dyn GraphQLServer> {
        self.graphql_server.clone()
    }

    fn get_instance_service(&self) -> Arc<dyn InstanceService> {
        self.instance_service.clone()
    }

    fn get_namespace_manager(&self) -> Arc<dyn NamespaceManager> {
        self.namespace_manager.clone()
    }

    fn get_plugin_container_manager(&self) -> Arc<dyn PluginContainerManager> {
        self.plugin_container_manager.clone()
    }

    fn get_plugin_repository_manager(&self) -> Arc<dyn PluginRepositoryManager> {
        self.plugin_repository_manager.clone()
    }

    fn get_reactive_entity_manager(&self) -> Arc<dyn ReactiveEntityManager> {
        self.reactive_entity_manager.clone()
    }

    fn get_reactive_flow_manager(&self) -> Arc<dyn ReactiveFlowManager> {
        self.reactive_flow_manager.clone()
    }

    fn get_reactive_relation_manager(&self) -> Arc<dyn ReactiveRelationManager> {
        self.reactive_relation_manager.clone()
    }

    fn get_relation_behaviour_manager(&self) -> Arc<dyn RelationBehaviourManager> {
        self.relation_behaviour_manager.clone()
    }

    fn get_relation_behaviour_registry(&self) -> Arc<dyn RelationBehaviourRegistry> {
        self.relation_behaviour_registry.clone()
    }

    fn get_relation_component_behaviour_manager(&self) -> Arc<dyn RelationComponentBehaviourManager> {
        self.relation_component_behaviour_manager.clone()
    }

    fn get_relation_component_behaviour_registry(&self) -> Arc<dyn RelationComponentBehaviourRegistry> {
        self.relation_component_behaviour_registry.clone()
    }

    fn get_relation_instance_import_export_manager(&self) -> Arc<dyn RelationInstanceImportExportManager> {
        self.relation_instance_import_export_manager.clone()
    }

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager> {
        self.relation_type_manager.clone()
    }

    fn get_relation_type_import_export_manager(&self) -> Arc<dyn RelationTypeImportExportManager> {
        self.relation_type_import_export_manager.clone()
    }

    fn get_relation_type_provider_registry(&self) -> Arc<dyn RelationTypeProviderRegistry> {
        self.relation_type_provider_registry.clone()
    }

    fn get_remotes_manager(&self) -> Arc<dyn RemotesManager> {
        self.remotes_manager.clone()
    }

    fn get_shutdown_manager(&self) -> Arc<dyn ShutdownManager> {
        self.shutdown_manager.clone()
    }

    fn get_web_resource_manager(&self) -> Arc<dyn WebResourceManager> {
        self.web_resource_manager.clone()
    }
}

impl RuntimeImpl {
    async fn wait_for_started_internal(&self) {
        while !self.is_running() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::get_runtime;
    use log::LevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::config::Appender;
    use log4rs::config::Root;
    use log4rs::Config;
    use std::time::Duration;

    /// This starts the runtime in an async environment.
    ///
    /// The runtime will be started including GraphQL server and fully
    /// initialized. After 2 seconds the runtime will be stopped.
    #[tokio::test(flavor = "multi_thread")]
    async fn test_run() {
        let stdout = ConsoleAppender::builder().build();
        let config = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
            .expect("Failed to create logger");
        if let Err(error) = log4rs::init_config(config) {
            eprintln!("Failed to configure logger: {}", error);
        }
        let rt = get_runtime();
        let runtime = rt.clone();
        tokio::spawn(async move {
            let runtime = runtime;
            runtime.init().await;
            runtime.post_init().await;
            runtime.run().await;
            runtime.pre_shutdown().await;
            runtime.shutdown().await;
        });
        tokio::time::sleep(Duration::from_secs(2)).await;
        rt.stop();
    }
}
