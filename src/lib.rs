use bevy_app::App;
use bevy_ecs::schedule::{ScheduleLabel, Schedules};

mod dot;

pub mod event_graph;
#[cfg(feature = "render_graph")]
pub mod render_graph;
pub mod schedule_graph;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
struct ScheduleDebugGroup;

/// Formats the events into a dot graph.
#[track_caller]
pub fn events_graph_dot(
    app: &mut App,
    labels: Vec<Box<dyn ScheduleLabel>>,
    settings: &event_graph::Settings,
) -> String {
    app.world
        .resource_scope::<Schedules, _>(|world, mut schedules| {
            let ignored_ambiguities = schedules.ignored_scheduling_ambiguities.clone();
            let mut contexts: Vec<event_graph::EventGraphContext> = Vec::new();
            for l in labels.iter() {
                let Some((_, schedule)) = schedules
                    .iter_mut()
                    .find(|s| (**l).as_dyn_eq().dyn_eq(s.0.as_dyn_eq()))
                else {
                    continue;
                };
                schedule.graph_mut().initialize(world);

                let _ = schedule.graph_mut().build_schedule(
                    world.components(),
                    ScheduleDebugGroup.intern(),
                    &ignored_ambiguities,
                );
                let context = event_graph::events_graph_dot(schedule, world, settings);
                contexts.push(context);
            }
            event_graph::print_context(schedules.as_ref(), &contexts, world, settings)
        })
}

/// Prints the schedule with default settings.
pub fn print_events_graph(app: &mut App, schedule_labels: Vec<Box<dyn ScheduleLabel>>) {
    let dot = events_graph_dot(app, schedule_labels, &event_graph::Settings::default());
    println!("{dot}");
}

/// Formats the schedule into a dot graph.
#[track_caller]
pub fn schedule_graph_dot(
    app: &mut App,
    label: impl ScheduleLabel,
    settings: &schedule_graph::Settings,
) -> String {
    app.world
        .resource_scope::<Schedules, _>(|world, mut schedules| {
            let ignored_ambiguities = schedules.ignored_scheduling_ambiguities.clone();

            let schedule = schedules
                .get_mut(label)
                .ok_or_else(|| "schedule doesn't exist".to_string())
                .unwrap();
            schedule.graph_mut().initialize(world);
            let _ = schedule.graph_mut().build_schedule(
                world.components(),
                ScheduleDebugGroup.intern(),
                &ignored_ambiguities,
            );

            schedule_graph::schedule_graph_dot(schedule, world, settings)
        })
}

/// Prints the schedule with default settings.
pub fn print_schedule_graph(app: &mut App, schedule_label: impl ScheduleLabel) {
    let dot = schedule_graph_dot(app, schedule_label, &schedule_graph::Settings::default());
    println!("{dot}");
}

/// Returns the current render graph using [`render_graph_dot`](render_graph::render_graph_dot).
/// # Example
/// ```rust,no_run
/// use bevy::prelude::*;
///
/// fn main() {
///     let mut app = App::new();
///     app.add_plugins(DefaultPlugins);
///     let settings = bevy_mod_debugdump::render_graph::Settings::default();
///     let dot = bevy_mod_debugdump::render_graph_dot(&mut app, &settings);
///     println!("{dot}");
/// }
/// ```
#[cfg(feature = "render_graph")]
pub fn render_graph_dot(app: &App, settings: &render_graph::Settings) -> String {
    use bevy_render::render_graph::RenderGraph;

    let render_app = app
        .get_sub_app(bevy_render::RenderApp)
        .unwrap_or_else(|_| panic!("no render app"));
    let render_graph = render_app.world.get_resource::<RenderGraph>().unwrap();

    render_graph::render_graph_dot(render_graph, settings)
}

/// Prints the current render graph using [`render_graph_dot`](render_graph::render_graph_dot).
/// # Example
/// ```rust,no_run
/// use bevy::prelude::*;
///
/// fn main() {
///     let mut app = App::new();
///     app.add_plugins(DefaultPlugins);
///     bevy_mod_debugdump::print_render_graph(&mut app);
/// }
/// ```
#[cfg(feature = "render_graph")]
pub fn print_render_graph(app: &mut App) {
    let dot = render_graph_dot(app, &render_graph::Settings::default());
    println!("{dot}");
}
