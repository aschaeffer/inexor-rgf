use std::sync::Arc;

use async_graphql::*;

use crate::api::PluginContainerManager;
use crate::api::PluginResolver;
use crate::graphql::query::GraphQLPlugin;

#[derive(Default)]
pub struct MutationPlugins;

/// Mutations for plugins.
#[Object]
impl MutationPlugins {
    async fn stop(&self, context: &Context<'_>, name: String) -> Result<GraphQLPlugin> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager>>()?;
        let plugin_resolver = context.data::<Arc<dyn PluginResolver>>()?;
        let id = plugin_container_manager.get_id(&name).ok_or_else(|| Error::new("Plugin with name not found"))?;
        // Stop plugin
        plugin_container_manager
            .stop(&id)
            .map_err(|e| Error::new(format!("Failed to start {}: {:?}", &id, e)))?;
        // Make all transitions until the plugin and all dependent plugins have stopped
        plugin_resolver.resolve_until_idle().await;
        Ok(GraphQLPlugin { id })
    }

    async fn start(&self, context: &Context<'_>, name: String) -> Result<GraphQLPlugin> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager>>()?;
        let plugin_resolver = context.data::<Arc<dyn PluginResolver>>()?;
        let id = plugin_container_manager.get_id(&name).ok_or_else(|| Error::new("Plugin with name not found"))?;
        // Start plugin
        plugin_container_manager
            .start(&id)
            .map_err(|e| Error::new(format!("Failed to start {}: {:?}", &id, e)))?;
        // Make all transitions until the plugin has started
        plugin_resolver.resolve_until_idle().await;
        // Start dependent plugins
        while plugin_container_manager.start_dependent_with_satisfied_dependencies(&id) {
            // Resolve until all dependent plugins are started
            plugin_resolver.resolve_until_idle().await;
        }
        Ok(GraphQLPlugin { id })
    }

    async fn restart(&self, context: &Context<'_>, name: String) -> Result<GraphQLPlugin> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager>>()?;
        let plugin_resolver = context.data::<Arc<dyn PluginResolver>>()?;
        let id = plugin_container_manager.get_id(&name).ok_or_else(|| Error::new("Plugin with name not found"))?;
        // Stop plugin
        plugin_container_manager
            .stop(&id)
            .map_err(|e| Error::new(format!("Failed to start {}: {:?}", &id, e)))?;
        // Make all transitions until the plugin and all dependent plugins have stopped
        plugin_resolver.resolve_until_idle().await;
        // Start plugin
        plugin_container_manager
            .start(&id)
            .map_err(|e| Error::new(format!("Failed to start {}: {:?}", &id, e)))?;
        // Make all transitions until the plugin has started
        plugin_resolver.resolve_until_idle().await;
        // Start dependent plugins
        while plugin_container_manager.start_dependent_with_satisfied_dependencies(&id) {
            // Resolve until all dependent plugins are started
            plugin_resolver.resolve_until_idle().await;
        }
        Ok(GraphQLPlugin { id })
    }

    /// Uninstalls a plugin
    async fn uninstall(&self, context: &Context<'_>, name: String) -> Result<bool> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager>>()?;
        let plugin_resolver = context.data::<Arc<dyn PluginResolver>>()?;
        let id = plugin_container_manager.get_id(&name).ok_or_else(|| Error::new("Plugin with name not found"))?;
        // plugin_container_manager.set_state(&id)
        plugin_container_manager
            .uninstall(&id)
            .map_err(|e| Error::new(format!("Failed to uninstall {}: {:?}", &id, e)))?;
        plugin_resolver.resolve_until_idle().await;
        Ok(true)
    }

    /// Redeploys a plugin which is already installed, resolved or active.
    async fn redeploy(&self, context: &Context<'_>, name: String) -> Result<GraphQLPlugin> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager>>()?;
        let plugin_resolver = context.data::<Arc<dyn PluginResolver>>()?;
        let id = plugin_container_manager.get_id(&name).ok_or_else(|| Error::new("Plugin with name not found"))?;
        plugin_container_manager
            .redeploy(&id)
            .map_err(|e| Error::new(format!("Failed to start {}: {:?}", &id, e)))?;
        plugin_resolver.resolve_until_idle().await;
        Ok(GraphQLPlugin { id })
    }
}
