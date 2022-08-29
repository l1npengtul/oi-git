use crate::audio::channels::{
    BGFanHum, BGFlorescence, BGMusic, HammerSFX, InteractionsSFX, Music, PhysicsColliderSFX,
    ScannerSFX, ToolsSFX,
};
use crate::audio::events::{
    CollisionSoundEvent, HammerSoundEvent, InteractSoundEvent, InteractSoundType, ScannerSoundEvent,
};
use crate::prelude::*;
use crate::tools::{SType, SensorEvent};
use bevy_asset_loader::prelude::AssetCollection;
use bevy_kira_audio::AudioSource;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioControl};
use std::time::Duration;

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/sfx/325113__fisch12345__error.ogg")]
    pub scan_error: Handle<AudioSource>,
    #[asset(path = "audio/sfx/353240__korndeftones123__dell-dimension-l600r_terminal_boot.ogg")]
    pub terminal_boot: Handle<AudioSource>,
    #[asset(path = "audio/sfx/353240__korndeftones123__dell-dimension-l600r_shutdown.ogg")]
    pub terminal_shutdown: Handle<AudioSource>,
    #[asset(path = "audio/sfx/521973__kastenfrosch__error.ogg")]
    pub terminal_error: Handle<AudioSource>,
    #[asset(path = "audio/sfx/361564__matthewwong__ding-dong.ogg")]
    pub dingdong: Handle<AudioSource>,
    #[asset(path = "audio/sfx/442676__qubodup__tree-falls-and-burns-down.ogg")]
    pub deleter: Handle<AudioSource>,
    #[asset(path = "audio/sfx/453177__benege__nasal-spray-single-stereo.ogg")]
    pub paint: Handle<AudioSource>,
    #[asset(path = "audio/sfx/460509__florianreichelt__hitting-in-a-face_hammer_hit.ogg")]
    pub hammer_hit: Handle<AudioSource>,
    #[asset(path = "audio/sfx/478203__jonnyruss01__glass-hit-bowls_collision.ogg")]
    pub glass_collide: Handle<AudioSource>,
    #[asset(path = "audio/sfx/512683__wavjunction-com__wooshes_hammer_miss.ogg")]
    pub hammer_voosh: Handle<AudioSource>,
    #[asset(path = "audio/sfx/535359__eminyildirim__combat-whooshes_throw.ogg")]
    pub throw_item: Handle<AudioSource>,
    #[asset(path = "audio/sfx/537769__janbezouska__factory-fluorescent-light-buzz.ogg")]
    pub flourescent: Handle<AudioSource>,
    #[asset(path = "audio/sfx/572940__zach-ramirez__ceiling-fan-closeup-hum.ogg")]
    pub fan: Handle<AudioSource>,
    #[asset(path = "audio/sfx/keyhit_spaguetti.ogg")]
    pub keyhit: Handle<AudioSource>,
    #[asset(path = "audio/sfx/509879__slamaxu__pickup-generic.ogg")]
    pub pickup: Handle<AudioSource>,
    #[asset(path = "audio/sfx/637750__kyles__phone-pickup-hangup_attach.ogg")]
    pub attach: Handle<AudioSource>,
    #[asset(path = "audio/music/Crinoline Dreams [4a_bgjheZkE].ogg")]
    pub caroline: Handle<AudioSource>,
    #[asset(path = "audio/music/tether.ogg")]
    pub tether: Handle<AudioSource>,
}

mod channels {
    pub struct BGFlorescence;
    pub struct BGFanHum;
    pub struct BGMusic;
    pub struct ToolsSFX;
    pub struct HammerSFX;
    pub struct PhysicsColliderSFX;
    pub struct InteractionsSFX;
    pub struct ScannerSFX;
    pub struct Music;
}

pub mod events {
    pub struct CollisionSoundEvent;
    pub struct HammerSoundEvent {
        pub hit: bool,
    }
    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
    pub enum InteractSoundType {
        Pickup,
        Throw,
        Attach,
        TerminalEnter,
        TerminalType,
        TerminalCommandError,
        TerminalLeave,
    }
    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
    pub struct InteractSoundEvent {
        pub int_type: InteractSoundType,
    }
    pub struct ScannerSoundEvent {
        pub success: bool,
    }
}

pub struct SusdioPlugin;

impl SusdioPlugin {}

impl Plugin for SusdioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionSoundEvent>()
            .add_event::<HammerSoundEvent>()
            .add_event::<InteractSoundEvent>()
            .add_event::<ScannerSoundEvent>();
        app.add_audio_channel::<BGFanHum>()
            .add_audio_channel::<BGFlorescence>()
            .add_audio_channel::<BGMusic>()
            .add_audio_channel::<ToolsSFX>()
            .add_audio_channel::<HammerSFX>()
            .add_audio_channel::<PhysicsColliderSFX>()
            .add_audio_channel::<InteractionsSFX>()
            .add_audio_channel::<ScannerSFX>()
            .add_audio_channel::<Music>();
        app.add_enter_system(GameState::InOffice, setup_background_soundscapes)
            .add_enter_system(GameState::MainMenu, main_menu_music)
            .add_enter_system(GameState::InOffice, game_over_music)
            .add_system(
                sensor_event_sfx
                    .run_in_state(GameState::InOffice)
                    .run_on_event::<SensorEvent>(),
            )
            .add_system(
                collision_event_sfx
                    .run_in_state(GameState::InOffice)
                    .run_on_event::<CollisionSoundEvent>(),
            )
            .add_system(
                hammer_event_sfx
                    .run_in_state(GameState::InOffice)
                    .run_on_event::<HammerSoundEvent>(),
            )
            .add_system(
                interact_event_sfx
                    .run_in_state(GameState::InOffice)
                    .run_on_event::<InteractSoundEvent>(),
            )
            .add_system(
                scanner_event_sfx
                    .run_in_state(GameState::InOffice)
                    .run_on_event::<ScannerSoundEvent>(),
            )
            .add_enter_system(GameState::GameOver, game_over_music);
    }
}

pub fn setup_background_soundscapes(
    audio: Res<AudioAssets>,
    fan: Res<AudioChannel<BGFanHum>>,
    fl: Res<AudioChannel<BGFlorescence>>,
    msic: Res<AudioChannel<BGMusic>>,
    music: Res<AudioChannel<Music>>,
) {
    music.stop().linear_fade_out(Duration::ZERO);
    fan.play(audio.fan.clone()).with_volume(0.5).looped();
    fl.play(audio.flourescent.clone()).with_volume(0.5).looped();
    msic.play(audio.caroline.clone()).with_volume(0.1).looped();
}

pub fn sensor_event_sfx(
    mut events: EventReader<SensorEvent>,
    audio: Res<AudioAssets>,
    tools: Res<AudioChannel<ToolsSFX>>,
) {
    for event in events.iter() {
        match event.stype {
            SType::Painter => {
                tools.play(audio.paint.clone()).with_volume(0.7);
            }
            SType::Deleter => {
                tools.play(audio.deleter.clone()).with_volume(0.8);
            }
            _ => {}
        }
    }
}

pub fn collision_event_sfx(
    mut events: EventReader<CollisionSoundEvent>,
    audio: Res<AudioAssets>,
    collision: Res<AudioChannel<PhysicsColliderSFX>>,
) {
    for _ in events.iter() {
        collision.stop();
        collision.play(audio.glass_collide.clone()).with_volume(0.7);
    }
}

pub fn hammer_event_sfx(
    mut events: EventReader<HammerSoundEvent>,
    audio: Res<AudioAssets>,
    hammer: Res<AudioChannel<HammerSFX>>,
) {
    for hammersnd in events.iter() {
        match hammersnd.hit {
            true => {
                hammer.play(audio.hammer_hit.clone()).with_volume(0.6);
            }
            false => {
                hammer.play(audio.hammer_voosh.clone()).with_volume(1.0);
            }
        }
    }
}

pub fn interact_event_sfx(
    mut events: EventReader<InteractSoundEvent>,
    audio: Res<AudioAssets>,
    interact: Res<AudioChannel<InteractionsSFX>>,
) {
    for interaction in events.iter() {
        match interaction.int_type {
            InteractSoundType::Pickup => {
                interact.play(audio.pickup.clone()).with_volume(0.7);
            }
            InteractSoundType::Throw => {
                interact.play(audio.throw_item.clone()).with_volume(1.0);
            }
            InteractSoundType::Attach => {
                interact.play(audio.attach.clone()).with_volume(0.7);
            }
            InteractSoundType::TerminalEnter => {
                interact.play(audio.terminal_boot.clone()).with_volume(0.7);
            }
            InteractSoundType::TerminalType => {
                interact.play(audio.keyhit.clone()).with_volume(0.7);
            }
            InteractSoundType::TerminalCommandError => {
                interact.play(audio.terminal_error.clone()).with_volume(0.7);
            }
            InteractSoundType::TerminalLeave => {
                interact
                    .play(audio.terminal_shutdown.clone())
                    .with_volume(0.7);
            }
        }
    }
}

pub fn scanner_event_sfx(
    mut events: EventReader<ScannerSoundEvent>,
    audio: Res<AudioAssets>,
    scanner: Res<AudioChannel<ScannerSFX>>,
) {
    for scan in events.iter() {
        {
            match scan.success {
                true => {
                    scanner.play(audio.dingdong.clone()).with_volume(0.8);
                }
                false => {
                    scanner.play(audio.scan_error.clone()).with_volume(0.8);
                }
            }
        }
    }
}

pub fn game_over_music(
    audio: Res<AudioAssets>,
    scanner: Res<AudioChannel<Music>>,
    bgmusic: Res<AudioChannel<BGMusic>>,
) {
    bgmusic.stop();
    scanner
        .play(audio.tether.clone())
        .with_volume(0.7)
        .linear_fade_in(Duration::from_secs(1));
}

pub fn main_menu_music(audio: Res<AudioAssets>, scanner: Res<AudioChannel<Music>>) {
    scanner
        .play(audio.tether.clone())
        .with_volume(0.7)
        .linear_fade_in(Duration::from_secs(1));
}
