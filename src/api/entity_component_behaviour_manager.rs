use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::model::BehaviourTypeId;
use crate::model::Component;
use crate::model::ReactiveEntityInstance;
use crate::reactive::BehaviourTransitionError;

#[async_trait]
pub trait EntityComponentBehaviourManager: Send + Sync {
    /// Adds new behaviours to the given entity instance.
    fn add_behaviours_to_entity(&self, entity_instance: Arc<ReactiveEntityInstance>);

    /// Possibly adds new behaviour to the given entity instance's component
    fn add_behaviours_to_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: Component);

    /// Removes behaviours from the given entity instance.
    fn remove_behaviours_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>);

    /// Removes behaviour from the given entity instance's component
    fn remove_behaviours_from_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: Component);

    /// Removes behaviours from the given entity instance by uuid.
    fn remove_behaviours_by_id(&self, id: &Uuid);

    /// Connect
    fn connect(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;

    /// Disconnect
    fn disconnect(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;

    /// Reconnect
    fn reconnect(&self, entity_instance: Arc<ReactiveEntityInstance>, behaviour_ty: &BehaviourTypeId) -> Result<(), BehaviourTransitionError>;
}
