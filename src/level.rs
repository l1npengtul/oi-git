use crate::{
    code::{Diff, LineOfCode, LoCBlock},
    prelude::*,
    tools::{SType, SensorEvent},
    ui::TimerText,
};
use std::time::Duration;

const LEVELS: &str = include_str!("../assets/code/code.txt");
const LEVEL_SEP: &str = "NEXT_LEVEL\n";

// Time given for each level in seconds
const LEVEL_TIMES: &[u64] = &[180, 120, 150, 250, 250, 300];

pub struct NewLevel {
    pub number: usize,
}

impl NewLevel {
    pub fn has_triggered(new: EventReader<NewLevel>) -> bool {
        let ret = !new.is_empty();
        new.clear();
        ret
    }
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewLevel>()
            .init_resource::<LevelTimer>()
            .init_resource::<Levels>()
            .init_resource::<Submitted>()
            .add_system(LevelTimer::tick.run_in_state(GameState::InOffice))
            .add_system(LevelTimer::update_ui.run_in_state(GameState::InOffice))
            .add_system(LevelTimer::new_level.run_in_state(GameState::InOffice))
            .add_system(Submitted::detect_submissions.run_in_state(GameState::InOffice))
            .add_system(LevelTimer::trigger_game_over_on_finish.run_in_state(GameState::InOffice))
            .add_enter_system(GameState::InOffice, start_first_level);
    }
}

fn start_first_level(mut next_level: EventWriter<NewLevel>, mut timer: ResMut<LevelTimer>) {
    next_level.send(NewLevel { number: 0 });
    timer.active = true;
}

#[derive(Debug, Clone)]
pub struct Levels {
    pub levels: Vec<CodeBlock>,
    pub code_text: Vec<&'static str>,
    pub current: usize,
}

impl Default for Levels {
    fn default() -> Self {
        Self::load()
    }
}

impl Levels {
    fn load() -> Self {
        let levels = LEVELS.split(LEVEL_SEP).map(CodeBlock::from_str).collect();
        Self {
            levels,
            code_text: LEVELS.split(LEVEL_SEP).collect(),
            current: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub code: Vec<LineOfCode>,
}

impl CodeBlock {
    fn from_str(s: &str) -> Self {
        let mut lines_of_code = Vec::new();
        for ln in s.lines() {
            let diff = Diff::from_line(ln);
            let code = ln.strip_prefix(&format!("{} ", diff.prefix())).unwrap();

            lines_of_code.push(LineOfCode {
                color: diff.to_color(),
                code: code.to_owned(),
                diff,
            })
        }
        CodeBlock {
            code: lines_of_code,
        }
    }
}

#[derive(Default)]
pub struct Submitted {
    pub last: Option<Vec<LoCBlock>>,
}

impl Submitted {
    fn detect_submissions(mut evs: EventReader<SensorEvent>, mut subs: ResMut<Submitted>) {
        for ev in evs.iter() {
            if matches!(ev.stype, SType::Scanner) {
                subs.last = Some(match ev.loc.clone() {
                    Some(v) => v,
                    None => return,
                });
            }
        }
        evs.clear();
    }
}

#[derive(Default)]
pub struct LevelTimer {
    time: Timer,
    pub active: bool,
}

impl LevelTimer {
    pub fn tick(mut timer: ResMut<LevelTimer>, time: Res<Time>) {
        if timer.active {
            timer.time.tick(time.delta());
        }
    }

    /// returns a string of the remaining time mm:ss
    pub fn remaining(&self) -> String {
        let rem = self.time.duration() - self.time.elapsed();
        let s = rem.as_secs();
        if s != 0 {
            format!("{:0>2}:{:0>2}", s / 60, s % 60)
        } else {
            "OUT OF TIME".to_string()
        }
    }

    pub fn duration(&self) -> Duration {
        self.time.duration()
    }

    pub fn time_left(&self) -> u128 {
        (self.duration() - self.time.elapsed()).as_millis()
    }

    pub fn update_ui(timer: Res<LevelTimer>, mut text: Query<&mut Text, With<TimerText>>) {
        text.single_mut().sections[0].value = format!("ASSEMBLE DIFFS: {}", timer.remaining());
    }

    pub fn new_level(mut timer: ResMut<LevelTimer>, mut new: EventReader<NewLevel>) {
        if let Some(n) = new.iter().next() {
            timer
                .time
                .set_duration(Duration::from_secs(LEVEL_TIMES[n.number]));
            timer.time.reset();
        }
    }

    // FIXME: Check if this is correct!
    pub fn trigger_game_over_on_finish(mut commands: Commands, mut timer: ResMut<LevelTimer>) {
        if timer.time.finished() && timer.active {
            timer.active = false;
            commands.insert_resource(NextState(GameState::GameOver));
        }
    }
}
