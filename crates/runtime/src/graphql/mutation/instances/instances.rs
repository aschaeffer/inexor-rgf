use async_graphql::*;

use crate::graphql::mutation::MutationEntityInstances;
use crate::graphql::mutation::MutationFlowInstances;
use crate::graphql::mutation::MutationRelationInstances;

#[derive(Default)]
pub struct MutationInstances;

/// Mutations on instances.
#[Object]
impl MutationInstances {
    /// Mutations on entity instances.
    async fn entities(&self) -> MutationEntityInstances {
        MutationEntityInstances
    }

    /// Mutations on relation instances.
    async fn relations(&self) -> MutationRelationInstances {
        MutationRelationInstances
    }

    /// Mutations for flows and their contained instances.
    async fn flows(&self) -> MutationFlowInstances {
        MutationFlowInstances
    }
}
