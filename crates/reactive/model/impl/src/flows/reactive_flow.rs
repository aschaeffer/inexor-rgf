use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::RwLock;

use serde_json::Map;
use serde_json::Value;
use uuid::Uuid;

use crate::ReactiveEntity;
use crate::ReactiveRelation;
use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::FlowInstance;
use reactive_graph_graph::FlowInstanceCreationError;
use reactive_graph_graph::Mutability;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::PropertyInstanceGetter;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::RelationInstance;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_graph::TypeDefinition;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_reactive_model_api::ReactiveFlowConstructionError;
use reactive_graph_reactive_model_api::ReactiveInstance;
use reactive_graph_reactive_model_api::ReactivePropertyContainer;

pub struct ReactiveFlowInstance {
    /// The id of the flow corresponds to the id of the wrapper entity instance.
    pub id: Uuid,

    /// The type definition of the entity type of the wrapper entity instance.
    pub ty: EntityTypeId,

    /// The flow contains entity instances. The entity instance may also
    /// be contained in other flows.
    pub entity_instances: RwLock<HashMap<Uuid, ReactiveEntity>>,

    /// The flow contains relation instances. The relation instances may also
    /// be contained in other flows.
    pub relation_instances: RwLock<HashMap<RelationInstanceId, ReactiveRelation>>,

    /// List of entities that has been added since creation of the flow.
    pub entities_added: RwLock<Vec<Uuid>>,

    /// List of entities that has been removed since creation of the flow.
    pub entities_removed: RwLock<Vec<Uuid>>,

    /// List of relations that has been added since creation of the flow.
    pub relations_added: RwLock<Vec<RelationInstanceId>>,

    /// List of relations that has been removed since creation of the flow.
    pub relations_removed: RwLock<Vec<RelationInstanceId>>,
}

impl ReactiveFlowInstance {
    pub fn new(wrapper_entity_instance: ReactiveEntity) -> Self {
        let mut entity_instances = HashMap::new();
        entity_instances.insert(wrapper_entity_instance.id, wrapper_entity_instance.clone());
        ReactiveFlowInstance {
            id: wrapper_entity_instance.id,
            ty: wrapper_entity_instance.ty.clone(),
            entity_instances: RwLock::new(entity_instances),
            relation_instances: RwLock::new(HashMap::new()),
            entities_added: RwLock::new(Vec::new()),
            entities_removed: RwLock::new(Vec::new()),
            relations_added: RwLock::new(Vec::new()),
            relations_removed: RwLock::new(Vec::new()),
        }
    }

    pub fn has_entity(&self, entity_instance: ReactiveEntity) -> bool {
        self.entity_instances.read().unwrap().contains_key(&entity_instance.id)
    }

    pub fn has_entity_by_id(&self, id: Uuid) -> bool {
        self.entity_instances.read().unwrap().contains_key(&id)
    }

    pub fn get_entity(&self, id: Uuid) -> Option<ReactiveEntity> {
        let reader = self.entity_instances.read().unwrap();
        reader.get(&id).cloned()
    }

    pub fn get_wrapper_entity_instance(&self) -> Option<ReactiveEntity> {
        self.get_entity(self.id)
    }

    pub fn add_entity(&self, entity_instance: ReactiveEntity) {
        let id = entity_instance.id;
        if !self.has_entity_by_id(id) {
            self.entity_instances.write().unwrap().insert(id, entity_instance);
            self.entities_added.write().unwrap().push(id);
            // self.entities_removed.write().unwrap().remove(entity_instance.id);
        }
    }

    pub fn remove_entity(&self, id: Uuid) {
        self.entity_instances.write().unwrap().remove(&id);
        self.entities_removed.write().unwrap().push(id);
    }

    pub fn has_relation(&self, relation_instance: ReactiveRelation) -> bool {
        self.relation_instances.read().unwrap().contains_key(&relation_instance.id())
    }

    pub fn has_relation_by_key(&self, relation_instance_id: &RelationInstanceId) -> bool {
        self.relation_instances.read().unwrap().contains_key(relation_instance_id)
    }

    pub fn get_relation(&self, relation_instance_id: &RelationInstanceId) -> Option<ReactiveRelation> {
        let reader = self.relation_instances.read().unwrap();
        reader.get(relation_instance_id).cloned()
    }

    pub fn add_relation(&self, relation_instance: ReactiveRelation) {
        let id = relation_instance.id();
        if !self.has_relation_by_key(&id) {
            self.relation_instances.write().unwrap().insert(id.clone(), relation_instance);
            self.relations_added.write().unwrap().push(id);
        }
    }

    pub fn remove_relation(&self, relation_instance_id: &RelationInstanceId) {
        self.relation_instances.write().unwrap().remove(relation_instance_id);
        self.relations_removed.write().unwrap().push(relation_instance_id.clone());
    }

    pub fn tick(&self) {
        let reader = self.entity_instances.read().unwrap();
        for (_, entity_instance) in reader.iter() {
            entity_instance.tick();
        }
    }
}

impl From<ReactiveEntity> for ReactiveFlowInstance {
    fn from(wrapper_entity_instance: ReactiveEntity) -> Self {
        ReactiveFlowInstance::new(wrapper_entity_instance)
    }
}

impl TryFrom<FlowInstance> for ReactiveFlowInstance {
    type Error = ReactiveFlowConstructionError;

    fn try_from(flow_instance: FlowInstance) -> Result<Self, ReactiveFlowConstructionError> {
        let flow_id = flow_instance.id;
        let mut entity_instances = HashMap::new();
        let mut wrapper = None;
        for (id, entity_instance) in flow_instance.entity_instances {
            let reactive_entity = ReactiveEntity::from(entity_instance);
            entity_instances.insert(id, reactive_entity.clone());
            if id == flow_id {
                wrapper = Some(reactive_entity.clone());
            }
        }
        if wrapper.is_none() {
            return Err(ReactiveFlowConstructionError::MissingWrapperInstance);
        }
        let mut relation_instances = HashMap::new();
        for (id, relation_instance) in flow_instance.relation_instances {
            // let id = relation_instance.id();
            let outbound = entity_instances.get(&relation_instance.outbound_id);
            if outbound.is_none() {
                // outbound entity missing
                return Err(ReactiveFlowConstructionError::MissingOutboundEntityInstance(relation_instance.outbound_id));
            }
            let inbound = entity_instances.get(&relation_instance.inbound_id);
            if inbound.is_none() {
                // inbound entity missing
                return Err(ReactiveFlowConstructionError::MissingInboundEntityInstance(relation_instance.inbound_id));
            }
            let outbound = outbound.unwrap().clone();
            let inbound = inbound.unwrap().clone();
            let reactive_relation = ReactiveRelation::new_from_instance(outbound, inbound, relation_instance.clone());
            relation_instances.insert(id, reactive_relation);
        }
        Ok(ReactiveFlowInstance {
            id: flow_id,
            ty: flow_instance.ty,
            entity_instances: RwLock::new(entity_instances),
            relation_instances: RwLock::new(relation_instances),
            // wrapper: wrapper.unwrap(),
            entities_added: RwLock::new(Vec::new()),
            entities_removed: RwLock::new(Vec::new()),
            relations_added: RwLock::new(Vec::new()),
            relations_removed: RwLock::new(Vec::new()),
        })
    }
}

impl PropertyInstanceGetter for ReactiveFlowInstance {
    fn get<S: Into<String>>(&self, property_name: S) -> Option<Value> {
        self.get_entity(self.id).and_then(|e| e.properties.get(&property_name.into()).map(|p| p.get()))
    }

    fn as_bool<S: Into<String>>(&self, property_name: S) -> Option<bool> {
        self.get_entity(self.id)
            .and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_bool()))
    }

    fn as_u64<S: Into<String>>(&self, property_name: S) -> Option<u64> {
        self.get_entity(self.id)
            .and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_u64()))
    }

    fn as_i64<S: Into<String>>(&self, property_name: S) -> Option<i64> {
        self.get_entity(self.id)
            .and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_i64()))
    }

    fn as_f64<S: Into<String>>(&self, property_name: S) -> Option<f64> {
        self.get_entity(self.id)
            .and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_f64()))
    }

    fn as_string<S: Into<String>>(&self, property_name: S) -> Option<String> {
        self.get_entity(self.id)
            .and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_string()))
    }

    fn as_array<S: Into<String>>(&self, property_name: S) -> Option<Vec<Value>> {
        self.get_entity(self.id)
            .and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_array()))
    }

    fn as_object<S: Into<String>>(&self, property_name: S) -> Option<Map<String, Value>> {
        self.get_entity(self.id)
            .and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_object()))
    }
}

impl PropertyInstanceSetter for ReactiveFlowInstance {
    fn set_checked<S: Into<String>>(&self, property_name: S, value: Value) {
        if let Some(instance) = self.get_entity(self.id) {
            if let Some(instance) = instance.properties.get(&property_name.into()) {
                instance.set_checked(value);
            }
        }
    }

    fn set<S: Into<String>>(&self, property_name: S, value: Value) {
        if let Some(instance) = self.get_entity(self.id) {
            if let Some(instance) = instance.properties.get(&property_name.into()) {
                instance.set(value);
            }
        }
    }

    fn set_no_propagate_checked<S: Into<String>>(&self, property_name: S, value: Value) {
        if let Some(instance) = self.get_entity(self.id) {
            if let Some(instance) = instance.properties.get(&property_name.into()) {
                instance.set_no_propagate_checked(value);
            }
        }
    }

    fn set_no_propagate<S: Into<String>>(&self, property_name: S, value: Value) {
        if let Some(instance) = self.get_entity(self.id) {
            if let Some(instance) = instance.properties.get(&property_name.into()) {
                instance.set_no_propagate(value);
            }
        }
    }

    fn mutability<S: Into<String>>(&self, property_name: S) -> Option<Mutability> {
        self.get_entity(self.id)
            .and_then(|instance| instance.properties.get(&property_name.into()).map(|p| p.value().mutability))
    }

    fn set_mutability<S: Into<String>>(&self, property_name: S, mutability: Mutability) {
        if let Some(instance) = self.get_entity(self.id) {
            if let Some(mut property_instance) = instance.properties.get_mut(&property_name.into()) {
                property_instance.set_mutability(mutability);
            }
        }
    }

    // TODO: fn set(&self, Map<String, Value>
    // TODO: Set values transactional: first set all values internally, then send all affected streams
}

impl NamespacedTypeGetter for ReactiveFlowInstance {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for ReactiveFlowInstance {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }
}

impl Display for ReactiveFlowInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}__{}", &self.ty, self.id)
    }
}

impl TryFrom<ReactiveFlowInstance> for FlowInstance {
    type Error = FlowInstanceCreationError;

    fn try_from(reactive_flow: ReactiveFlowInstance) -> Result<Self, FlowInstanceCreationError> {
        let wrapper = reactive_flow.get_entity(reactive_flow.id);
        if wrapper.is_none() {
            return Err(FlowInstanceCreationError);
        }
        let wrapper = wrapper.unwrap();
        let entity_instance: EntityInstance = wrapper.clone().into();
        let mut flow_instance = FlowInstance::from(entity_instance);
        flow_instance.description = wrapper.description.clone();
        reactive_flow.entity_instances.read().unwrap().iter().for_each(|(_, reactive_entity)| {
            if reactive_entity.id != reactive_flow.id {
                let entity_instance: EntityInstance = reactive_entity.into();
                flow_instance.entity_instances.push(entity_instance);
            }
        });
        reactive_flow.relation_instances.read().unwrap().iter().for_each(|(_, reactive_instance)| {
            let relation_instance = RelationInstance::from(reactive_instance);
            flow_instance.relation_instances.push(relation_instance);
        });
        Ok(flow_instance)
    }
}

impl TryFrom<&ReactiveFlowInstance> for FlowInstance {
    type Error = FlowInstanceCreationError;

    fn try_from(reactive_flow: &ReactiveFlowInstance) -> Result<Self, FlowInstanceCreationError> {
        let wrapper = reactive_flow.get_entity(reactive_flow.id);
        if wrapper.is_none() {
            return Err(FlowInstanceCreationError);
        }
        let wrapper = wrapper.unwrap();
        let entity_instance: EntityInstance = wrapper.clone().into();
        let mut flow_instance = FlowInstance::from(entity_instance);
        flow_instance.description = wrapper.description.clone();
        reactive_flow.entity_instances.read().unwrap().iter().for_each(|(_, reactive_entity)| {
            if reactive_entity.id != reactive_flow.id {
                let entity_instance: EntityInstance = reactive_entity.into();
                flow_instance.entity_instances.push(entity_instance);
            }
        });
        reactive_flow.relation_instances.read().unwrap().iter().for_each(|(_, reactive_relation)| {
            let relation_instance: RelationInstance = reactive_relation.into();
            flow_instance.relation_instances.push(relation_instance);
        });
        Ok(flow_instance)
    }
}

#[derive(Clone)]
pub struct ReactiveFlow(Arc<ReactiveFlowInstance>);

impl ReactiveFlow {
    pub fn new(wrapper_entity_instance: ReactiveEntity) -> Self {
        ReactiveFlowInstance::new(wrapper_entity_instance).into()
    }
}
impl Deref for ReactiveFlow {
    type Target = Arc<ReactiveFlowInstance>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ReactiveFlowInstance> for ReactiveFlow {
    fn from(reactive_flow: ReactiveFlowInstance) -> Self {
        ReactiveFlow(Arc::new(reactive_flow))
    }
}

impl TryFrom<FlowInstance> for ReactiveFlow {
    type Error = ReactiveFlowConstructionError;

    fn try_from(flow_instance: FlowInstance) -> Result<Self, Self::Error> {
        ReactiveFlowInstance::try_from(flow_instance).map(|reactive_flow| reactive_flow.into())
    }
}

impl TryFrom<ReactiveFlow> for FlowInstance {
    type Error = FlowInstanceCreationError;

    fn try_from(reactive_flow: ReactiveFlow) -> Result<Self, FlowInstanceCreationError> {
        FlowInstance::try_from(reactive_flow.0.deref())
    }
}
