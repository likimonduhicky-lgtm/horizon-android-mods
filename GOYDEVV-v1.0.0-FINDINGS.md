# Reverse-engineering notes – GoyDevv Horizon-Android v1.0.0

APK: `Horizon-Android-v1.0.0-arm64-v8a.apk`
Main native library: `lib/arm64-v8a/libhorizon_android.so` (~61 MB uncompressed)

## Confirmed strings related to mods

- Empty state: `"No mods: put .hmod files in the horizon-mods folder"`
- Spanish equivalent also present
- Tooltip: `"Mods made for Horizon, from the horizon-mods folder. Each runs in a sandbox and stays off until you turn it on."`
- CLI / help: `--dev-mod PATH          A content mod (.hmod or folder) to develop: always on`
- File type labels include both:
  - `Horizon Mod (.hmod)`
  - `Native Mod (.hnmod)`
- Game directory layout mentions `horizon-mods/` alongside `saves/`, `options.txt`, `resourcepacks/`, etc.

## Interpretation

Primary (and only fully functional) mod path on this build is the **content-mod / sandboxed `.hmod`** system living in the `horizon-mods` folder.

`.hnmod` is recognised as a file type (so the file manager / import UI can show the extension), but there is no evidence of the desktop-style mixin compilation or of loading prebuilt `aarch64-linux-android` libraries from inside a `.hnmod` the way some other Android ports document.

## Practical advice

- Only place real `.hmod` files in `horizon-mods/`.
- Source-only desktop `.hnmod` files (the ones containing `mixins/` + `horizon-native.json`) will not execute.
- If a feature can be expressed as pure data/assets, convert it to `.hmod`.
- For true native behaviour (HUD overlays that need code, freecam, veinminer logic, voice chat, etc.) this particular build is not the right target without further reverse-engineering or using a different Android port that documents a prebuilt native ABI.
