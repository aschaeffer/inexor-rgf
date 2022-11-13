pub use component_manager_impl::*;
// pub use dynamic_graph_impl::*;
pub use entity_behaviour_manager_impl::*;
pub use entity_behaviour_registry_impl::*;
pub use entity_component_behaviour_manager_impl::*;
pub use entity_component_behaviour_registry_impl::*;
pub use entity_instance_manager_impl::*;
pub use entity_type_manager_impl::*;
pub use entity_vertex_manager_impl::*;
pub use event_manager_impl::*;
pub use flow_instance_manager_impl::*;
pub use flow_type_manager_impl::*;
pub use graph_database_impl::*;
pub use graphql_query_service_impl::*;
pub use graphql_schema_manager_impl::*;
pub use graphql_server_impl::*;
pub use plugin_registry_impl::*;
pub use reactive_entity_instance_manager_impl::*;
pub use reactive_flow_instance_manager_impl::*;
pub use reactive_relation_instance_manager_impl::*;
pub use relation_behaviour_manager_impl::*;
pub use relation_behaviour_registry_impl::*;
pub use relation_component_behaviour_manager_impl::*;
pub use relation_component_behaviour_registry_impl::*;
pub use relation_edge_manager_impl::*;
pub use relation_instance_manager_impl::*;
pub use relation_type_manager_impl::*;
pub use shutdown_manager_impl::*;
pub use web_resource_manager_impl::*;

pub mod component_manager_impl;
// pub mod dynamic_graph_impl;
pub mod entity_behaviour_manager_impl;
pub mod entity_behaviour_registry_impl;
pub mod entity_component_behaviour_manager_impl;
pub mod entity_component_behaviour_registry_impl;
pub mod entity_instance_manager_impl;
pub mod entity_type_manager_impl;
pub mod entity_vertex_manager_impl;
pub mod event_manager_impl;
pub mod flow_instance_manager_impl;
pub mod flow_type_manager_impl;
pub mod graph_database_impl;
pub mod graphql_query_service_impl;
pub mod graphql_schema_manager_impl;
pub mod graphql_server_impl;
pub mod plugin_registry_impl;
pub mod reactive_entity_instance_manager_impl;
pub mod reactive_flow_instance_manager_impl;
pub mod reactive_relation_instance_manager_impl;
pub mod reactive_type_identifiers;
pub mod relation_behaviour_manager_impl;
pub mod relation_behaviour_registry_impl;
pub mod relation_component_behaviour_manager_impl;
pub mod relation_component_behaviour_registry_impl;
pub mod relation_edge_manager_impl;
pub mod relation_instance_manager_impl;
pub mod relation_type_manager_impl;
pub mod shutdown_manager_impl;
pub mod web_resource_manager_impl;

// TODO: Move this to a core model

const NAMESPACE_CORE: &str = "core";
const NAMESPACE_LOGICAL: &str = "logical";
const NAMESPACE_FLOW: &str = "flow";

const TYPE_SYSTEM_EVENT: &str = "system_event";
const TYPE_GENERIC_FLOW: &str = "generic_flow";
const TYPE_SHUTDOWN: &str = "shutdown";

const COMPONENT_LABELED: &str = "labeled";
const COMPONENT_ACTION: &str = "action";
const COMPONENT_EVENT: &str = "event";

const PROPERTY_LABEL: &str = "label";
const PROPERTY_SHUTDOWN: &str = "shutdown";
const PROPERTY_TRIGGER: &str = "trigger";
const PROPERTY_EVENT: &str = "event";
