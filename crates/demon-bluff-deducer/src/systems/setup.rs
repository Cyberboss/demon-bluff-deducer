use bevy::{
	core_pipeline::core_2d::Camera2d,
	ecs::system::{Commands, ResMut},
	gizmos::config::{DefaultGizmoConfigGroup, GizmoConfigStore},
	render::view::RenderLayers,
};

pub fn setup(mut commands: Commands, mut gizmos_config_store: ResMut<GizmoConfigStore>) {
	commands.spawn(Camera2d);

	let (default, _) = gizmos_config_store.config_mut::<DefaultGizmoConfigGroup>();
	// TODO: ensure gizmos are layered below other stuff
	default.render_layers = RenderLayers::from_layers(&[0]);
}
