use std::collections::HashMap;

use serde_json::Value;
use uuid::Uuid;

use crate::model::EntityInstance;
use crate::model::EntityType;
use crate::model::EntityTypeType;

#[allow(dead_code)]
pub struct EntityInstanceBuilder {
    ty: EntityTypeType,
    id: Option<Uuid>,
    properties: HashMap<String, Value>,
}

#[allow(dead_code)]
impl EntityInstanceBuilder {
    pub fn new(ty: EntityTypeType) -> EntityInstanceBuilder {
        EntityInstanceBuilder {
            ty,
            id: None,
            properties: HashMap::new(),
        }
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> EntityInstanceBuilder {
        EntityInstanceBuilder::new(EntityTypeType::new_from_type(namespace, type_name))
    }

    pub fn id(&mut self, id: Uuid) -> &mut EntityInstanceBuilder {
        self.id = Some(id);
        self
    }

    pub fn property<S: Into<String>>(&mut self, property_name: S, value: Value) -> &mut EntityInstanceBuilder {
        self.properties.insert(property_name.into(), value);
        self
    }

    pub fn build(&self) -> EntityInstance {
        EntityInstance::new(self.ty.clone(), self.id.unwrap_or_else(Uuid::new_v4), self.properties.clone())
    }
}

impl From<EntityType> for EntityInstanceBuilder {
    fn from(entity_type: EntityType) -> Self {
        let mut builder = EntityInstanceBuilder {
            ty: entity_type.ty.clone(),
            id: None,
            properties: HashMap::new(),
        };
        for property_type in entity_type.properties {
            builder.property(property_type.name.clone(), property_type.data_type.default_value());
        }
        builder
    }
}
