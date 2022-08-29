use crate::code::{CodeColor, Diff, LineOfCode, LoCBlock, LoCEntity, LocType};
use crate::level::{LevelTimer, Levels, NewLevel, Submitted};
use crate::prelude::*;
use crate::ui::UIRoot;

pub mod conv_cp437;
mod text_sprite;
pub use text_sprite::*;
mod screen;
use crate::audio::events::{InteractSoundEvent, InteractSoundType, ScannerSoundEvent};
use crate::player::fsm::{PlayerState, PlayerStateMachine};
pub use screen::TerminalScreenTarget;

mod spawn;

pub const PROPMPT: &str = ">>";
pub const TERM_DIM: (f32, f32) = (1280.0, 960.0);
pub const TERM_W: f32 = TERM_DIM.0;
pub const TERM_H: f32 = TERM_DIM.1;

pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TerminalCommand>()
            .add_event::<TermWrite>()
            .add_plugin(TextSpritePlugin)
            .add_enter_system(
                GameState::InOffice,
                TerminalInput::spawn.label("terminal_spawn"),
            )
            .add_enter_system(GameState::InOffice, TerminalScreenTarget::set_up_2d)
            .add_system(
                TerminalInput::take_input
                    .run_in_state(GameState::InOffice)
                    .run_if_resource_equals(PlayerStateMachine::INTERACTING),
            )
            .add_system(TerminalCommand::reset.run_in_state(GameState::InOffice))
            .add_system(TerminalCommand::submit.run_in_state(GameState::InOffice))
            .add_system(TerminalInput::take_write.run_in_state(GameState::InOffice))
            .add_system(TerminalInput::show_or_hide_ui.run_in_state(GameState::InOffice))
            .add_system(TerminalCommand::reset.run_in_state(GameState::InOffice));
    }
}

#[derive(Component)]
pub struct TerminalInput {
    pub user_inp_start: usize,
}

impl TerminalInput {
    fn take_input(
        mut commands: Commands,
        mut interact_sfx_event: EventWriter<InteractSoundEvent>,
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

        for _ in keys.get_just_pressed() {
            interact_sfx_event.send(InteractSoundEvent {
                int_type: InteractSoundType::TerminalType,
            });
        }

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
            } else {
                interact_sfx_event.send(InteractSoundEvent {
                    int_type: InteractSoundType::TerminalCommandError,
                });
            }
            use TerminalCommand::*;
            let message = format!(
                "\n{response}\n{prompt}",
                response = match term_cmd.clone() {
                    Some(Restart) => "restarting...".to_owned(),
                    Some(ShowCode) => format!("{}", levels.code_text[levels.current].trim_end()),
                    Some(Send) => "sending off completed code".to_owned(),
                    Some(Exit) => "goodbye git".to_owned(),
                    Some(Help) => "[c]ode | [r]estart | [e]xit | [f]inish".to_owned(),
                    None => format!("command {cmd} not recognised, use help for commands"),
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

    fn take_write(
        mut commands: Commands,
        mut q_input: Query<(Entity, &mut TextSprite, &mut TerminalInput)>,
        mut writes: EventReader<TermWrite>,
    ) {
        let (entity, mut text_sprite, mut term) = q_input.single_mut();
        for TermWrite { s } in writes.iter() {
            text_sprite.add_multiline_str(&s, &mut commands, entity);
            term.user_inp_start = text_sprite.len();
        }

        writes.clear();

        let ln_count = text_sprite.text.lines().count();
        let max_count = (TERM_H / ATLAS_CHAR_H).floor() as usize;
        if ln_count > max_count {
            text_sprite.remove_top_lines(&mut commands, entity, ln_count - max_count);
            term.user_inp_start = text_sprite.len();
        }
    }

    fn show_or_hide_ui(
        mut timer: ResMut<LevelTimer>,
        mut ui: Query<&mut Visibility, With<UIRoot>>,
        pstate: Res<PlayerStateMachine>,
    ) {
        let show = pstate.state() != PlayerState::Interacting;
        timer.active = show;
        ui.single_mut().is_visible = show;
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum TerminalCommand {
    Restart,
    ShowCode,
    Send,
    Exit,
    Help,
}

impl TerminalCommand {
    pub fn from_str(s: &str) -> Option<Self> {
        #[allow(unreachable_patterns)]
        Some(match s.to_ascii_lowercase().as_str() {
            "r" | "restart" => Self::Restart,
            "c" | "show" | "code" | "show code" => Self::ShowCode,
            "f" | "r" | "finish" | "finished" | "release" => Self::Send,
            "e" | "exit" => Self::Exit,
            "h" | "help" => Self::Help,
            _ => return None,
        })
    }

    pub fn reset(
        mut commands: Commands,
        mut term_cmds: EventReader<Self>,
        levels: Res<Levels>,
        mut new_level: EventWriter<NewLevel>,
        locs: Query<Entity, With<LoCEntity>>,
    ) {
        let reset = term_cmds.iter().any(|c| *c == Self::Restart);
        term_cmds.clear();
        if reset {
            new_level.send(NewLevel {
                number: levels.current,
            });
            locs.iter()
                .for_each(|e| commands.entity(e).despawn_recursive());
        }
    }

    pub fn submit(
        mut commands: Commands,
        mut term_cmds: EventReader<Self>,
        mut levels: ResMut<Levels>,
        mut new_level: EventWriter<NewLevel>,
        mut scanner_event: EventWriter<ScannerSoundEvent>,
        mut subs: ResMut<Submitted>,
        mut state: ResMut<State<GameState>>,
        mut term_write: EventWriter<TermWrite>,
        timer: Res<LevelTimer>,
        locs: Query<Entity, With<LoCEntity>>,
    ) {
        let submit = term_cmds.iter().any(|c| *c == Self::Send);
        term_cmds.clear();
        if !submit {
            return;
        }
        if subs.last.is_none() {
            state.set(GameState::GameOver).unwrap();
            return;
        }
        // showing the score
        let sub = subs
            .last
            .clone()
            .unwrap()
            .into_iter()
            .map(locblk_to_loc)
            .collect::<Vec<_>>();
        let cor = levels.levels[levels.current]
            .code
            .iter()
            .cloned()
            .filter(|loc| loc.diff != Diff::Rem)
            .collect::<Vec<_>>();
        // calculate time bonus
        let time_score = (timer.time_left() / 10) as f64;
        let code_score = crate::score::score(&cor, &sub);
        let score = time_score * code_score as f64;

        // calculate the possible total score of this level
        let possible_total_score = timer.duration().as_millis() as f64;
        // fail the player if the score is <50%

        if (score / possible_total_score) < 0.5 {
            term_write.send(TermWrite {
                s: format!(
                    "\ntime: {}\naccuracy: {:.2}%\ntotal: {}\nPASS. Loading next job...\n>>",
                    time_score as u64,
                    code_score * 100.0,
                    score as u64
                ),
            });

            subs.last = None;
            // advancing to next level
            let next_level = levels.current + 1;
            if next_level >= levels.levels.len() {
                state.set(GameState::GameOver).unwrap();
                return;
            }
            levels.current = next_level;
            new_level.send(NewLevel { number: next_level });
            scanner_event.send(ScannerSoundEvent { success: true });
        } else {
            // FIXME: copied over from reset()
            term_write.send(TermWrite {
                s: format!(
                    "\ntime: {}\naccuracy: {:.2}%\ntotal: {}\nFAIL. Resetting playfield...\n>>",
                    time_score as u64,
                    code_score * 100.0,
                    score as u64
                ),
            });

            new_level.send(NewLevel {
                number: levels.current,
            });
            scanner_event.send(ScannerSoundEvent { success: false });
        }

        // despawn all the stuff from the old level
        locs.iter()
            .for_each(|e| commands.entity(e).despawn_recursive());
    }
}

pub struct TermWrite {
    s: String,
}

fn locblk_to_loc(blk: LoCBlock) -> LineOfCode {
    let (color, diff) = match blk.loc_type {
        LocType::Green => (CodeColor::Green, Diff::Pos),
        LocType::Red => (CodeColor::Red, Diff::Neg),
        LocType::Neutral => (CodeColor::Normal, Diff::Eq),
    };
    LineOfCode {
        diff,
        color,
        code: blk.line_of_code,
    }
}
