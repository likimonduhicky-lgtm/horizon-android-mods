# Horizon Android Mods

Attempts / notes for running popular desktop Horizon `.hnmod` native mods on **Horizon Android**.

## Hard truth

Desktop `.hnmod` files are **source** (horizon-mixins patches).  
Android has **no compiler**. It only loads `.hnmod` files that contain a prebuilt `lib*.so` for `aarch64-linux-android` speaking the small `horizon-native-abi` C ABI.

Most of the mods in `desktop-source/` use deep mixins (world, inventory, camera, rendering, networking). Those hooks simply do not exist in the current public Android ABI. Full ports are not possible until the Android port expands the ABI / SDK.

## What is here

| Folder | Content |
|--------|---------|
| `desktop-source/` | Original extracted `.hnmod` contents (for reference) |
| `android-scaffolds/` | Minimal Rust crates that *could* be built with `cargo-ndk` + `horizon-native-abi` for the few features that *are* possible (HUD, keys, chat commands) |

## Feasibility

| Mod | Android feasibility | Notes |
|-----|---------------------|-------|
| fps_overlay | High | Pure HUD |
| armor_status | Medium | Mostly HUD |
| appleskin | Medium | Food / saturation overlay |
| freelook / freecam_plus | Low | Needs camera hooks |
| fast_trading | Low | Inventory UI |
| veinminer / treefall | Very low | World / mining hooks |
| voicechat | Extremely low | Full audio + net stack |

## How to build an Android mod (reference)

See the official docs from the Android port you are using:

- https://github.com/Rayan-XOO/horizon/blob/android-port/docs/android-mods.md

Rough steps:

```bash
rustup target add aarch64-linux-android
cargo install cargo-ndk
# set ANDROID_NDK_HOME
# clone the matching Horizon android-port branch
cargo install --path crates/cargo-horizon-mod

# then in a crate that depends on horizon-native-abi:
cargo horizon-mod android
```

## Your existing work

You already have a Horizon native mod scaffold here:  
https://github.com/likimonduhicky-lgtm/sodium-clump

That is the correct style for **desktop**. For Android you need a completely different crate type (`cdylib` + `horizon-native-abi`).

## Next steps if you want real Android ports

1. Pick the 1–2 mods that are actually useful on phone (probably fps_overlay + armor_status).
2. Rewrite them against the limited ABI.
3. Build the `.so` against the exact ABI version of the APK you run.
4. Package as `.hnmod` with the `binaries` field.

I can help write the minimal Android versions of the HUD mods if you tell me which Android port (Starling / Rayan-XOO / GoyDevv / etc.) and version you are on.
