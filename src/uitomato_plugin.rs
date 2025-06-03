use bevy::{ prelude::*, text::{FontSmoothing, LineHeight}};

use crate::WindowSize;


pub struct UiTomatoPlugin;

impl Plugin for UiTomatoPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_ui);
    }
}


#[derive(Component)]
pub struct TimerText;

#[derive(Component)]
pub struct RestTimerText;

#[derive(Component)]
pub struct InputField;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, window_res: Res<WindowSize>) {
    let font_handle: Handle<Font> = asset_server.load("fonts/CHERI.TTF");
    let window_size = window_res.clone();


    commands.spawn((Text2d::new("Hola Tomato <3"), TextFont{
        font: font_handle.clone(),
        font_size: 30.0,
        font_smoothing: FontSmoothing::None,
        line_height: LineHeight::Px(100.0),
        ..default()
    },
    TextColor(Color::srgb(1.0, 0.3, 0.2)),
    Transform::from_xyz(0., window_size.height / 2.0 - 100.0, 0.0)
    ));

    //commands.spawn(Text2d::new("uguemuwhen osas"));

    // timer text, recuerda hacer el struct para despues encontrar este componente n.n!
    commands.spawn((
        Text2d::new("00:00"),
        TextFont{
            font_size: 20.0,
            font_smoothing: FontSmoothing::None,
            line_height: LineHeight::Px(10.0),
            ..default()
        }, Transform::from_xyz(0., 20.0 , 0.0),
        TimerText
     ));
     
     commands.spawn((
         Text2d::new("Enter time here..."),
         TextFont{
             font: font_handle,
             font_size: 16.0,
             font_smoothing: FontSmoothing::None,
             ..default()
         },
         TextColor(Color::srgb(0.7, 0.7, 0.7)), // Gray placeholder text
         Transform::from_xyz(0., -20.0, 0.0),
         InputField
     ));
     
     commands.spawn((
     Text2d::new("Rest Time"), 
     TextFont{..default()},
     Transform::from_xyz(0., -60.0 , 0.0),
     RestTimerText
    ));

}