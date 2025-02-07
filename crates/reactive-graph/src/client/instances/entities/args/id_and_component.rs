use crate::client::error::CommandError;
use crate::client::error::CommandError::NotFound;
use crate::client::types::components::args::type_id::ComponentTypeIdArgs;
use clap::Args;
use uuid::Uuid;

/// Identifies a component of an entity instance.
#[derive(Args, Debug, Clone)]
pub(crate) struct IdAndComponentArgs {
    /// The id of the reactive instance.
    pub id: Uuid,

    /// The component type of the reactive instance.
    #[clap(flatten)]
    pub component_ty: ComponentTypeIdArgs,
}

impl IdAndComponentArgs {
    pub fn id_not_found(&self) -> CommandError {
        NotFound(format!("The instance with the id {} was not found", &self.id))
    }
}
