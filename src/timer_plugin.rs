use bevy::{ecs::system::command, prelude::*};
use crate::uitomato_plugin::{TimerText, RestTimerText};

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorkTemporizer(Timer::from_seconds(45.*60., TimerMode::Once)))
        .insert_resource(RestTemporizer(Timer::from_seconds(15.*60., TimerMode::Once)))
        .insert_resource(Repetition(3))
        .add_systems(Update, system_timer);
    }
}

// no

#[derive(Resource)]
 struct WorkTemporizer(Timer);

 #[derive(Resource)]
 struct RestTemporizer(Timer);

 #[derive(Resource)]
 struct Repetition(i32);


fn system_timer( 
    time: Res<Time>, 
    mut temporizer: ResMut<WorkTemporizer>, 
    mut rest_temporizer: ResMut<RestTemporizer>,
    mut repetition: ResMut<Repetition>, 
    mut text_queries: ParamSet<(
        Query<&mut Text2d, With<TimerText>>,
        Query<&mut Text2d, With<RestTimerText>>
    )>
) {
    
    if repetition.0 > 0 {

        let work_timer = temporizer.0.tick(time.delta());
        if let Ok(mut text_component) = text_queries.p0().single_mut() {
            text_component.0 = format!("{:.2}", work_timer.remaining_secs()/60.0);
        }

        if temporizer.0.finished() {

           
           if let Ok(mut rest_text_component) = text_queries.p1().single_mut() {
                rest_text_component.0 = format!("{:.2}",&rest_temporizer.0.tick(time.delta()).remaining_secs()/60.0);
                temporizer.0.pause();
                rest_temporizer.0.unpause();
            }


            if rest_temporizer.0.finished() {
                repetition.0 = repetition.0 - 1;
                println!("Timer finished {}", repetition.0);
                rest_temporizer.0.pause();
                rest_temporizer.0.reset();
                temporizer.0.reset();
                temporizer.0.unpause();
            }

            
        }
    }
}









