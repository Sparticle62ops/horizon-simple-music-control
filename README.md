# horizon-simple-music-control

Simple Music Control ported to Horizon (the Rust Minecraft client) as a native mod.

- Upstream: https://github.com/pajicadvance/simple-music-control, commit `d59f4b9` ("Neo 26.3", 2026-10-26)
- License: MIT (c) 2026 pajic. `LICENSE.md` is the upstream license verbatim.
- Modrinth: https://modrinth.com/mod/simple-music-control (471,620 downloads)
- Port policy: 1:1 behavior port, no optimizations or behavior changes.

## Slice 1 (this repo)

- `config/ModConfig.java` -> `Config`: all 13 options with upstream defaults (stopMusicOnJukeboxUse, creativeMusicInSurvival, situationalMusicInCreative, musicMinDelay 300 [0,2400], musicMaxDelay 600 [10,2400], playMusicWhenPaused, nowPlayingWidget, nowPlayingWidgetDuration 6 [1,30], showNowPlayingWidgetInPauseMenu, showNowPlayingWidgetOnTitleScreen, pauseWidgetPosition/titleWidgetPosition TOP_LEFT, unlockSituationalMusic false, situationalMusicChance 70 [0,100]) and fzzy_config's validateAndSet clamping; broken file falls back to defaults.
- `gui/WidgetPosition.java` -> `WidgetPosition` (4 variants, upstream translation key).
- `util/Constants.java` -> the 20-entry overworld and 5-entry nether situational music lists, in upstream order.
- `util/JukeboxTracker.java` -> `JukeboxTracker` (Object2BooleanOpenHashMap semantics, onJukeboxPlay/onJukeboxStop/noJukeboxesInRange).
- `util/ModUtil.java` -> `pickRandomSituationalMusic` (full overworld/creative/survival/nether branch tree) and `onClientTick` (64-block jukebox sweep at center-of-block distance squared vs 4096, keybind consumes, music-note tick).
- `keybind/ModKeybinds.java` -> NEXT_MUSIC_TRACK (KEY_M), TOGGLE_MUSIC (unbound), onNextMusicTrack/onToggleMusic semantics; `consumeClick` click-count behavior.
- `mixin/client/SoundEngineMixin.java` (>=26.1 path) -> `SoundEngine::smc_set_music_paused` (loaded gate, playMusicWhenPaused gate, MUSIC-source-only channels) and the pauseAllExcept wrap that drops MUSIC from the ignored sources.
- `gui/MusicNoteColorManager.java` -> the tick state machine with the upstream 25 ms gate and -1 initial color.
- `gui/NowPlayingWidget.java` -> the toggle-notification state slice (`displayToggleNotification`: pause-screen gate, centered = level != null, duration * 1000 ms). Rendering is a later slice.
- `SMC.java` -> mod id, `id(path)`, dev-env-only `debugLog`.

39 tests cover the config model + clamping, enum serde names, keybind defaults and click counting, the situational picker's full branch tree, the jukebox sweep including the 64-block boundary, toggle and next-track behavior, the sound-engine pause rules, and the 25 ms color gate.

## Degradations (tracked on horizon-rust/horizon)

Issue **#11150**: (1) the music manager is write-only for native mods - API 119 `native_music(music, volume)` can set the track but a mod cannot read the current track/translation key or vanilla's situational pick, so next-track and the widget can't run fully 1:1; (2) no per-source sound-engine pause control (the mod pauses only MUSIC channels and wraps pauseAllExcept); (3) no MUSIC_NOTE ColorLerper for the cycling note color (Horizon implements the same pattern internally for sheep). Widget HUD rendering additionally waits on #10125, keybind consumeClick on #11123.

## Remaining for full 1:1

- `NowPlayingWidget` rendering half (action-bar/corner text, fade, music note) - needs #10125 + #11150.
- The client mixins: `MusicManagerMixin` (delay re-read, pause handling, situational override), `BackgroundMusicMixin` (situational music in creative), `LevelEventHandlerMixin`/`LevelRendererMixin` (jukebox tracking events), `MinecraftMixin`, `ScreenMixin`, `TitleScreenMixin`, `PauseScreenMixin`, `NowPlayingToastMixin`, `GameRendererMixin`, `SoundManagerAccessor` wiring.
- Sodium options-page port (`ModSodiumConfig`).
- Registry submission once the render half lands.
