use bevy::{prelude::*, render::view::window, window::PrimaryWindow};
mod timer_plugin;
mod uitomato_plugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window: Some(Window {
            title: "Tomato".to_string(),
            resolution: (250.0, 400.0).into(),
            ..default()
        }),
        close_when_requested: true,
        exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
    }))
    .add_systems(PreStartup, camera_setup)
    .add_plugins(timer_plugin::TimerPlugin)
    .add_plugins(uitomato_plugin::UiTomatoPlugin)
    
    .run();
}

#[derive(Resource, Debug, Clone)]
struct WindowSize {
    width: f32,
    height: f32,
}


fn camera_setup(mut commands: Commands, mut window: Query<&mut Window, With<PrimaryWindow>>) {
    commands.spawn(Camera2d::default());

    for windsize in window.iter_mut() {
        
        let winsize = WindowSize {
            width: windsize.width(),
            height: windsize.height(),        
        };
        commands.insert_resource(winsize);
      //  println!("Window size: {:?}", windsize.width());
    }
}