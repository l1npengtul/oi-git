use crate::level::Levels;
use crate::office::{OfficeEntities, SceneLocations};
use crate::player::PlayerLookingAt;
use crate::prelude::*;

pub mod conv_cp437;
mod text_sprite;
pub use text_sprite::*;
mod screen;
use crate::player::fsm::PlayerStateMachine;
pub use screen::TerminalScreenTarget;

mod spawn;

pub const PROPMPT: &str = "[r]estart | [s]end | [c]ode | [e]xit\n>>";
pub const TERM_DIM: (f32, f32) = (1280.0, 960.0);
pub const TERM_W: f32 = TERM_DIM.0;
pub const TERM_H: f32 = TERM_DIM.1;

pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TerminalCommand>()
            .add_plugin(TextSpritePlugin)
            .add_enter_system(
                GameState::MainMenu,
                TerminalInput::spawn.label("terminal_spawn"),
            )
            .add_enter_system(GameState::MainMenu, TerminalScreenTarget::set_up_2d)
            .add_system(
                TerminalInput::take_input
                    .run_in_state(GameState::MainMenu)
                    .run_if_resource_equals(PlayerStateMachine::INTERACTING),
            );
    }
}

#[derive(Component)]
pub struct TerminalInput {
    pub user_inp_start: usize,
}

impl TerminalInput {
    // fn is_looked_at(player_looking_at: Res<PlayerLookingAt>, office: Res<OfficeEntities>) -> bool {
    //     // FIXME: give the terminal a proper collider, this is
    //     // really really really broken
    //     player_looking_at.entity == Some(*office.entities.get("collider_desk").unwrap())
    // }
    //
    // fn is_player_close(
    //     office_l: Res<SceneLocations>,
    //     q_player: Query<&Transform, With<Player>>,
    // ) -> bool {
    //     let term = *office_l.locations.get("point3d_terminal").unwrap();
    //     let player = *q_player.single();
    //     let dist = term.translation.distance(player.translation);
    //     dist < 2.0
    // }

    fn take_input(
        mut commands: Commands,
        mut q_input: Query<(Entity, &mut TextSprite, &mut TerminalInput)>,
        mut keystrokes: EventReader<ReceivedCharacter>,
        keys: Res<Input<KeyCode>>,
        mut terminal_command: EventWriter<TerminalCommand>,
        levels: Res<Levels>,
    ) {
        let (entity, mut text_sprite, mut term) = q_input.single_mut();
        let input = keystrokes
            .iter()
            .map(|ev| ev.char)
            .filter(|ch| conv_cp437::index_of(*ch).is_some())
            .collect::<String>();
        text_sprite.add_str(&input, &mut commands, entity, |_| {});

        if keys.just_pressed(KeyCode::Back) && text_sprite.len() > term.user_inp_start {
            text_sprite.pop(&mut commands);
        }

        if keys.just_pressed(KeyCode::Return) {
            let cmd = text_sprite
                .text
                .lines()
                .last()
                .unwrap()
                .strip_prefix(">>")
                .unwrap()
                .trim();
            let term_cmd = TerminalCommand::from_str(cmd);
            if let Some(cmd) = term_cmd.clone() {
                terminal_command.send(cmd)
            }
            use TerminalCommand::*;
            let message = format!(
                "\n{response}\n{prompt}",
                response = match term_cmd.clone() {
                    Some(Restart) => "restarting...".to_owned(),
                    Some(ShowCode) =>
                        format!("code: \n{}", levels.code_text[levels.current].trim_end()),
                    Some(Send) => "sending off completed code".to_owned(),
                    Some(Exit) => "goodbye git".to_owned(),
                    None => format!("command {cmd} not recognised"),
                },
                prompt = PROPMPT,
            );

            text_sprite.add_multiline_str(&message, &mut commands, entity);
            term.user_inp_start = text_sprite.len();
        }

        let ln_count = text_sprite.text.lines().count();
        let max_count = (TERM_H / ATLAS_CHAR_H).floor() as usize;
        if ln_count > max_count {
            text_sprite.remove_top_lines(&mut commands, entity, ln_count - max_count);
            term.user_inp_start = text_sprite.len();
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum TerminalCommand {
    Restart,
    ShowCode,
    Send,
    Exit,
}

impl TerminalCommand {
    pub fn from_str(s: &str) -> Option<Self> {
        Some(match s.to_ascii_lowercase().as_str() {
            "r" | "restart" => Self::Restart,
            "c" | "show" | "code" | "show code" => Self::ShowCode,
            "s" | "send" => Self::Send,
            "e" | "exit" => Self::Exit,
            _ => return None,
        })
    }
}
