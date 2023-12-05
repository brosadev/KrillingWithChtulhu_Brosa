/* EXAMPLE fn: how to add a score event to any system */
/*
pub fn how_to_use(
	mut score_event: EventWriter<DisplayEvent>,
	input: Res<Input<KeyCode>>,
	) {
	if input.pressed(KeyCode::E) {
		score_event.send(DisplayEvent { points: 1 });
	}
}***********************************/

use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct ScoreText;

#[derive(Resource, Debug, Default)]
pub struct DisplayData{
	pub total_score: usize,
}

#[derive(Event, Default)]
pub struct DisplayEvent{
	pub points: usize,
}

pub fn setup(
	mut commands: Commands,
	) {
	commands.spawn((
		TextBundle::from_sections([
			TextSection::new(		
				"Score: ",
				TextStyle {	
					font_size: 25.0,
					..default()
				}),
			TextSection::from_style( 
				TextStyle {	
					font_size: 25.0,
					..default()
				})
				,
			])
			.with_text_alignment(TextAlignment::Center)
			.with_style(Style {
				position_type: PositionType::Absolute,
				bottom: Val::Px(5.0),
				right: Val::Px(5.0),
				..default()
			}),
			ScoreText,
	));
}

pub fn update_score(
	mut score_events: EventReader<DisplayEvent>,
	mut query: Query<&mut Text, With<ScoreText>>,
	mut display_data: ResMut<DisplayData>
	) {
	for mut score in &mut query {
		for event in score_events.read() {
			display_data.total_score += event.points;
			score.sections[1].value = format!("{}", display_data.total_score);
		}
	}
}
