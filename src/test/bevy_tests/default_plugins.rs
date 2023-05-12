use bevy::a11y::AccessibilityPlugin;
use bevy::app::PluginGroupBuilder;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::time::TimePlugin;
use bevy::asset::AssetPlugin;
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::pbr::PbrPlugin;
use bevy::render::pipelined_rendering::PipelinedRenderingPlugin;
use bevy::render::RenderPlugin;
use bevy::sprite::SpritePlugin;
use bevy::winit::WinitPlugin;

pub struct NoRenderBevyIntegrationTestPlugin;

impl PluginGroup for NoRenderBevyIntegrationTestPlugin {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(LogPlugin::default())
            .add(TaskPoolPlugin::default())
            .add(TypeRegistrationPlugin::default())
            .add(FrameCountPlugin::default())
            .add(TimePlugin::default())
            .add(TransformPlugin::default())
            .add(HierarchyPlugin::default())
            .add(DiagnosticsPlugin::default())
            .add(InputPlugin::default())
            .add(WindowPlugin::default())
            .add(AccessibilityPlugin)
            .add(AssetPlugin::default());

        #[cfg(feature = "debug_asset_server")]
        {
            group = group.add(bevy_asset::debug_asset_server::DebugAssetServerPlugin::default());
        }

        #[cfg(feature = "bevy_scene")]
        {
            group = group.add(bevy_scene::ScenePlugin::default());
        }

        // group = group.add(WinitPlugin::default());

        group = group
            .add(RenderPlugin::default())
            // NOTE: Load this after renderer initialization so that it knows about the supported
            // compressed texture formats
            .add(ImagePlugin::default());

        // group = group
        //     .add(PipelinedRenderingPlugin::default());

        group = group.add(CorePipelinePlugin::default());

        group = group.add(SpritePlugin::default());

        #[cfg(feature = "bevy_text")]
        {
            group = group.add(bevy_text::TextPlugin::default());
        }

        #[cfg(feature = "bevy_ui")]
        {
            group = group.add(bevy_ui::UiPlugin::default());
        }

        group = group.add(PbrPlugin::default());

        // NOTE: Load this after renderer initialization so that it knows about the supported
        // compressed texture formats
        #[cfg(feature = "bevy_gltf")]
        {
            group = group.add(bevy_gltf::GltfPlugin::default());
        }

        #[cfg(feature = "bevy_audio")]
        {
            group = group.add(bevy_audio::AudioPlugin::default());
        }

        #[cfg(feature = "bevy_gilrs")]
        {
            group = group.add(bevy_gilrs::GilrsPlugin::default());
        }

        #[cfg(feature = "bevy_animation")]
        {
            group = group.add(bevy_animation::AnimationPlugin::default());
        }

        group
    }
}
