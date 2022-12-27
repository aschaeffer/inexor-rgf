use std::sync::Arc;

use crate::model::ComponentBehaviourTypeId;
use crate::model::ReactiveRelationInstance;
use crate::plugins::RelationComponentBehaviourRegistry;
use crate::reactive::BehaviourFactory;

pub struct RelationComponentBehaviourRegistryImpl {
    relation_component_behaviour_manager: Arc<dyn crate::api::RelationComponentBehaviourManager>,
    relation_component_behaviour_registry: Arc<dyn crate::api::RelationComponentBehaviourRegistry>,
    reactive_relation_instance_manager: Arc<dyn crate::api::ReactiveRelationInstanceManager>,
}

impl RelationComponentBehaviourRegistryImpl {
    pub fn new(
        relation_component_behaviour_manager: Arc<dyn crate::api::RelationComponentBehaviourManager>,
        relation_component_behaviour_registry: Arc<dyn crate::api::RelationComponentBehaviourRegistry>,
        reactive_relation_instance_manager: Arc<dyn crate::api::ReactiveRelationInstanceManager>,
    ) -> Self {
        Self {
            relation_component_behaviour_manager,
            relation_component_behaviour_registry,
            reactive_relation_instance_manager,
        }
    }
}

impl RelationComponentBehaviourRegistry for RelationComponentBehaviourRegistryImpl {
    fn register(&self, component_behaviour_ty: ComponentBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveRelationInstance> + Send + Sync>) {
        self.relation_component_behaviour_registry.register(component_behaviour_ty.clone(), factory);
        self.reactive_relation_instance_manager
            .add_behaviour_to_all_relation_components(&component_behaviour_ty);
    }

    fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {
        self.relation_component_behaviour_registry.unregister(component_behaviour_ty);
        self.relation_component_behaviour_manager
            .remove_behaviours_by_behaviour(&component_behaviour_ty.behaviour_ty);
    }
}
