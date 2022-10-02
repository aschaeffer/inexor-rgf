use std::sync::Arc;

use async_trait::async_trait;
use indradb::EdgeKey;
use uuid::Uuid;

use crate::api::Lifecycle;
use crate::model::ReactiveEntityInstance;

pub const SYSTEM_EVENT_PROPERTY_EVENT: &str = "event";
pub const SYSTEM_EVENT_PROPERTY_LABEL: &str = "label";

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum SystemEventTypes {
    ComponentCreated,
    ComponentUpdated,
    ComponentDeleted,
    EntityTypeCreated,
    EntityTypeComponentAdded,
    EntityTypeComponentRemoved,
    EntityTypePropertyAdded,
    EntityTypePropertyRemoved,
    EntityTypeExtensionAdded,
    EntityTypeExtensionRemoved,
    EntityTypeDeleted,
    RelationTypeCreated,
    RelationTypeComponentAdded,
    RelationTypeComponentRemoved,
    RelationTypePropertyAdded,
    RelationTypePropertyRemoved,
    RelationTypeExtensionAdded,
    RelationTypeExtensionRemoved,
    RelationTypeDeleted,
    FlowTypeCreated,
    FlowTypeUpdated,
    FlowTypeDeleted,

    /// The type system has changed
    TypeSystemChanged,

    EntityInstanceCreated,
    EntityInstanceDeleted,
    RelationInstanceCreated,
    RelationInstanceDeleted,
    FlowInstanceCreated,
    FlowInstanceDeleted,
}

pub enum SystemEvent {
    ComponentCreated(String),
    ComponentUpdated(String),
    ComponentDeleted(String),
    EntityTypeCreated(String),
    EntityTypeComponentAdded(String, String),
    EntityTypeComponentRemoved(String, String),
    EntityTypePropertyAdded(String, String),
    EntityTypePropertyRemoved(String, String),
    EntityTypeExtensionAdded(String, String),
    EntityTypeExtensionRemoved(String, String),
    EntityTypeDeleted(String),
    RelationTypeCreated(String),
    RelationTypeComponentAdded(String, String),
    RelationTypeComponentRemoved(String, String),
    RelationTypePropertyAdded(String, String),
    RelationTypePropertyRemoved(String, String),
    RelationTypeExtensionAdded(String, String),
    RelationTypeExtensionRemoved(String, String),
    RelationTypeDeleted(String),
    FlowTypeCreated(String),
    FlowTypeUpdated(String),
    FlowTypeDeleted(String),
    TypeSystemChanged,
    EntityInstanceCreated(Uuid),
    EntityInstanceDeleted(Uuid),
    RelationInstanceCreated(EdgeKey),
    RelationInstanceDeleted(EdgeKey),
    FlowInstanceCreated(Uuid),
    FlowInstanceDeleted(Uuid),
}

impl From<&SystemEvent> for SystemEventTypes {
    fn from(event: &SystemEvent) -> Self {
        match event {
            SystemEvent::ComponentCreated(_) => SystemEventTypes::ComponentCreated,
            SystemEvent::ComponentUpdated(_) => SystemEventTypes::ComponentUpdated,
            SystemEvent::ComponentDeleted(_) => SystemEventTypes::ComponentDeleted,
            SystemEvent::EntityTypeCreated(_) => SystemEventTypes::EntityTypeCreated,
            SystemEvent::EntityTypeComponentAdded(_, _) => SystemEventTypes::EntityTypeComponentAdded,
            SystemEvent::EntityTypeComponentRemoved(_, _) => SystemEventTypes::EntityTypeComponentRemoved,
            SystemEvent::EntityTypePropertyAdded(_, _) => SystemEventTypes::EntityTypePropertyAdded,
            SystemEvent::EntityTypePropertyRemoved(_, _) => SystemEventTypes::EntityTypePropertyRemoved,
            SystemEvent::EntityTypeExtensionAdded(_, _) => SystemEventTypes::EntityTypeExtensionAdded,
            SystemEvent::EntityTypeExtensionRemoved(_, _) => SystemEventTypes::EntityTypeExtensionRemoved,
            SystemEvent::EntityTypeDeleted(_) => SystemEventTypes::EntityTypeDeleted,
            SystemEvent::RelationTypeCreated(_) => SystemEventTypes::RelationTypeCreated,
            SystemEvent::RelationTypeComponentAdded(_, _) => SystemEventTypes::RelationTypeComponentAdded,
            SystemEvent::RelationTypeComponentRemoved(_, _) => SystemEventTypes::RelationTypeComponentRemoved,
            SystemEvent::RelationTypePropertyAdded(_, _) => SystemEventTypes::RelationTypePropertyAdded,
            SystemEvent::RelationTypePropertyRemoved(_, _) => SystemEventTypes::RelationTypePropertyRemoved,
            SystemEvent::RelationTypeExtensionAdded(_, _) => SystemEventTypes::RelationTypeExtensionAdded,
            SystemEvent::RelationTypeExtensionRemoved(_, _) => SystemEventTypes::RelationTypeExtensionRemoved,
            SystemEvent::RelationTypeDeleted(_) => SystemEventTypes::RelationTypeDeleted,
            SystemEvent::FlowTypeCreated(_) => SystemEventTypes::FlowTypeCreated,
            SystemEvent::FlowTypeUpdated(_) => SystemEventTypes::FlowTypeUpdated,
            SystemEvent::FlowTypeDeleted(_) => SystemEventTypes::FlowTypeDeleted,
            SystemEvent::TypeSystemChanged => SystemEventTypes::TypeSystemChanged,
            SystemEvent::EntityInstanceCreated(_) => SystemEventTypes::EntityInstanceCreated,
            SystemEvent::EntityInstanceDeleted(_) => SystemEventTypes::EntityInstanceDeleted,
            SystemEvent::RelationInstanceCreated(_) => SystemEventTypes::RelationInstanceCreated,
            SystemEvent::RelationInstanceDeleted(_) => SystemEventTypes::RelationInstanceDeleted,
            SystemEvent::FlowInstanceCreated(_) => SystemEventTypes::FlowInstanceCreated,
            SystemEvent::FlowInstanceDeleted(_) => SystemEventTypes::FlowInstanceDeleted,
        }
    }
}

#[async_trait]
pub trait SystemEventManager: Send + Sync + Lifecycle {
    fn emit_event(&self, event: SystemEvent);

    fn get_system_event_instances(&self) -> Vec<Arc<ReactiveEntityInstance>>;

    fn get_system_event_instance(&self, event_type: SystemEventTypes) -> Option<Arc<ReactiveEntityInstance>>;
}
