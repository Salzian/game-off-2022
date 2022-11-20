use bevy::app::Plugin;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub(crate) struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // If this is a debug build
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new());
        }
    }
}
