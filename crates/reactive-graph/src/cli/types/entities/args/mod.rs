use crate::cli::output_format::OutputFormatArgs;
use crate::cli::types::entities::commands::EntityTypesCommands;
use clap::Args;

pub mod add_extension;
pub mod add_property;
pub mod create;
pub mod remove_extension;
pub mod remove_property;
pub mod type_id;
pub mod update_description;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct EntityTypesArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<EntityTypesCommands>,

    #[arg(global = true, short, long)]
    pub output_format: Option<OutputFormatArgs>,
}
