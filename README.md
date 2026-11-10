# Horizon Android Mods

Notes and desktop sources for trying to use popular Horizon native mods on Android.

## Target launcher tested

**GoyDevv/Horizon-Android v1.0.0** (`Horizon-Android-v1.0.0-arm64-v8a.apk`)

Reverse-engineered from `lib/arm64-v8a/libhorizon_android.so`.

### What this launcher actually accepts

| Format | Supported? | Details |
|--------|------------|---------|
| **`.hmod`** | **Yes (primary)** | Put files in the `horizon-mods/` folder. Runs in a sandbox. Explicit message in the binary: *"No mods: put .hmod files in the horizon-mods folder"* |
| **`.hnmod`** | File-type only | Shown as "Native Mod (.hnmod)" in the picker, but **no compilation or native loading path**. Desktop source mixins do **not** run. |
| Folder with `horizon-mod.toml` | Yes | Treated as a content mod (same path as `.hmod`) |

UI tooltip from the binary:
> "Mods made for Horizon, from the horizon-mods folder. Each runs in a sandbox and stays off until you turn it on."

`--dev-mod PATH` also exists and is documented as accepting a content mod (`.hmod` or folder).

### Conclusion for the uploaded mods

All files originally provided (`appleskin`, `armor_status`, `fast_trading`, `fps_overlay`, `freecam_plus`, `freelook`, `treefall`, `veinminer`, `voicechat`) are **source `.hnmod`** packages containing `horizon-native.json` + `mixins/*.rs`.

They will **not** load on this GoyDevv v1.0.0 launcher.

You need either:
1. Real **`.hmod`** (data/assets only) versions of the features, or
2. Properly pre-built Android native libraries against a port that exposes the limited `horizon-native-abi` (this particular build does not appear to fully support that path the way some other forks do).

### How to install working content mods on this launcher

1. Install the APK.
2. Use the in-app File Manager / SAF or a file explorer that can reach the app’s storage.
3. Place real `.hmod` files into the **`horizon-mods`** folder inside the game directory.
4. Enable them in the Mods page (they stay off by default).

### Desktop sources

The original extracted desktop `.hnmod` contents are kept under `desktop-source/` for reference (see commits / files).

### Android scaffold (experimental)

`android-scaffolds/fps_overlay/` contains a minimal starting point written against the public `horizon-native-abi` style. It is **not** known to work with this exact GoyDevv binary without matching headers and a proper build.

---

**Bottom line:** This launcher is built for sandboxed **`.hmod`** content mods. The native source mods you have need to be rewritten or replaced with content-mod equivalents.
