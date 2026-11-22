//! simple-music-control (Horizon Rust port), the mod API entry.
//!
//! 1:1 port of pajic's simple-music-control (MIT,
//! https://github.com/pajicadvance/simple-music-control, port by
//! sparticle): the full mod core - the config model, the situational music
//! picker, the jukebox tracker, the keybind model, the music-note color
//! tick and the sound-engine pause control. All in `runtime`.
//!
//! This root carries the mod API wiring on the pinned SDK (Horizon
//! f00125f61f, mixins e59da909, mod api 0.2): the launcher-facing entry.
//! It registers nothing and hooks nothing yet: the input/keybind hook, the
//! sound-engine and music-player surfaces and the client tick hook have no
//! surfaces in the client mod API, so no music control happens in game.
//! The logic runs its tests; the seams stay recorded in the README and get
//! wired the day those surfaces land. Upstream licence: MIT, carried
//! verbatim in LICENSE.md. Source:
//! https://github.com/Sparticle62ops/horizon-simple-music-control

pub mod runtime;

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl mod_api::Hooks for Mod {}
