use bevy::log::LogPlugin;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.build().disable::<LogPlugin>());
    let dot = bevy_mod_debugdump::data_graph_dot(
        &mut app,
        &bevy_mod_debugdump::data_graph::Settings {
            include_system: Some(Box::new(|s| s.name().starts_with("bevy_ui"))),
            include_schedule: Some(Box::new(|s| s.label().0.as_dyn_eq().dyn_eq(&PostUpdate))),
            ..default()
        },
    );
    println!("{dot}");
}
