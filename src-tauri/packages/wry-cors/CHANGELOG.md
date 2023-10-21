# Changelog

## \[0.23.1]

- Fixes usage of the `linux-headers` feature.
  - [64a72ff](https://github.com/tauri-apps/wry/commit/64a72ffd2369f51d36bdb00973f71326e8395016) fix(wry): correctly use the linux-headers feature on 2022-12-05

## \[0.23.0]

- Properly parse the content type header for the `android.webkit.WebResourceResponse` mime type.
  - [1db5ea6](https://github.com/tauri-apps/wry/commit/1db5ea68c2028db77788ec8c78ee0ab75a7a5f7f) fix(android): properly parse content-type for response mime type ([#772](https://github.com/tauri-apps/wry/pull/772)) on 2022-11-27
- Change typo in `WebViewBuilderExtWindows::with_additionl_browser_args`. to `WebViewBuilderExtWindows::with_additional_browser_args`.
  - [db1c290](https://github.com/tauri-apps/wry/commit/db1c290c0d8b58f6612ef9bef244a06261fb2a6e) fix(windows): Fix typo in method name of `WebViewBuilderExtWindows` ([#781](https://github.com/tauri-apps/wry/pull/781)) on 2022-12-02
- Add `Webiew::load_url`.
  - [a2b9531](https://github.com/tauri-apps/wry/commit/a2b9531b0e8397dcf74c049ccf6c7fa125288ca8) feat: add `Webiew::navigate_to_url`, closes [#776](https://github.com/tauri-apps/wry/pull/776) ([#777](https://github.com/tauri-apps/wry/pull/777)) on 2022-11-30
- Change the type of `WebViewBuilderExtWindows::with_additional_browser_args` argument from `AsRef<str>` to `Into<String>` to reduce extra allocation.
  - [b0ff06a](https://github.com/tauri-apps/wry/commit/b0ff06aba5aea77f067aee1e9bf8ac8c245ac5e8) perf: reduce extra allocation at `WebViewBuilderExtWindows::with_additional_browser_args` argument ([#783](https://github.com/tauri-apps/wry/pull/783)) on 2022-12-03
- Validate custom protocol response status code on Android.
  - [7f585c7](https://github.com/tauri-apps/wry/commit/7f585c7dc947936387faf565f3f5cbe62148daaf) feat(android): validate custom protocol response status code ([#779](https://github.com/tauri-apps/wry/pull/779)) on 2022-11-30

## \[0.22.5]

- On macOS, fix arrow keys misprint text on textarea.
  - [3005e54](https://github.com/tauri-apps/wry/commit/3005e5450339c6c3fbc1c7c67ab8008ed39ec864) On macOS, fix arrow keys misprint texts ([#769](https://github.com/tauri-apps/wry/pull/769)) on 2022-11-25

## \[0.22.4]

- On Linux, add `linux-headers` feature flag to fix version regression. The minimum webkit2gtk version remains v2.22.
  - [cf447f6](https://github.com/tauri-apps/wry/commit/cf447f64451fd8345f21440df31601265e0fde86) On Linux, add header feature flag to fix version regression ([#766](https://github.com/tauri-apps/wry/pull/766)) on 2022-11-24

## \[0.22.3]

- On macOS, fix keyinput missing by calling superclass methods.
  - [e40e55a](https://github.com/tauri-apps/wry/commit/e40e55a41d8d65ceda5e182c8915d37b5698c7b0) On macOS, fix keyinput missing by calling super class methods ([#764](https://github.com/tauri-apps/wry/pull/764)) on 2022-11-21

## \[0.22.2]

- On macOS, add an API to enable or disable backward and forward navigation gestures.
  - [487dff0](https://github.com/tauri-apps/wry/commit/487dff03a103df999e5e0c6286f75b4d1f419d25) Add the ability to navigate with swipe gesture ([#757](https://github.com/tauri-apps/wry/pull/757)) on 2022-11-16
  - [1a0ec19](https://github.com/tauri-apps/wry/commit/1a0ec19fd533c853b744c5e2346542d2e1e5805d) Update gesture change file to patch ([#763](https://github.com/tauri-apps/wry/pull/763)) on 2022-11-21
- On macOS, pass key event to menu if we have one on key press.
  - [2e5e138](https://github.com/tauri-apps/wry/commit/2e5e1381789c332654a5ffee47d578042a9be87b) On macOS, pass key event to menu on key press ([#760](https://github.com/tauri-apps/wry/pull/760)) on 2022-11-21

## \[0.22.1]

- Fix `WebViewBuilder::with_accept_first_mouse` taking behavior of first initalized webview.
  - [0647c0e](https://github.com/tauri-apps/wry/commit/0647c0efe131566ffbab0729e9d74355155c3c32) fix(macos): fix `acceptFirstMouse` for subsequent webviews, closes [#751](https://github.com/tauri-apps/wry/pull/751) ([#752](https://github.com/tauri-apps/wry/pull/752)) on 2022-11-13
- Fix download implementation on macOS older than 11.3.
  - [e69ddc6](https://github.com/tauri-apps/wry/commit/e69ddc6943770aa8baa02431bb037bbdcb3cbd80) fix(macos): download breaking app on macOS older than 11.3, closes [#755](https://github.com/tauri-apps/wry/pull/755) ([#756](https://github.com/tauri-apps/wry/pull/756)) on 2022-11-15
- On macOS, remove webview from window's NSView before dropping.
  - [3d3ea80](https://github.com/tauri-apps/wry/commit/3d3ea80808a327c546d8bbd97e06ef4b8feb32d0) On macOS, remove webview from window's NSView before dropping ([#754](https://github.com/tauri-apps/wry/pull/754)) on 2022-11-14

## \[0.22.0]

- Added `WebViewAttributes::with_accept_first_mouse` method for macOS.
  - [2c23440](https://github.com/tauri-apps/wry/commit/2c23440f9c194064caa907650df39bf9c96ed99c) feat(macos): add `accept_first_mouse` option, closes [#714](https://github.com/tauri-apps/wry/pull/714) ([#715](https://github.com/tauri-apps/wry/pull/715)) on 2022-10-04
- **Breaking change** Custom protocol now takes `Request` and returns `Response` types from `http` crate.
  - [1510e45](https://github.com/tauri-apps/wry/commit/1510e452547a95af2e42ff5199640877beecdbd7) refactor: use `http` crate primitives instead of a custom impl ([#706](https://github.com/tauri-apps/wry/pull/706)) on 2022-09-29
- Enabled devtools in debug mode by default.
  - [fea0638](https://github.com/tauri-apps/wry/commit/fea0638d9ad100c00b95468aa16fc44d6517ac0d) feat: enable devtools in debug mode by default ([#741](https://github.com/tauri-apps/wry/pull/741)) on 2022-10-27
- On Desktop, add `download_started_handler` and `download_completed_handler`. See `blob_download` and `download_event` example for their usages.
  - [3691c4f](https://github.com/tauri-apps/wry/commit/3691c4f6c88fe43e92597caf3003c8d57b447a7b) feat: Add download started and download completed callbacks ([#530](https://github.com/tauri-apps/wry/pull/530)) on 2022-10-19
- Fix double permission dialog on macOS 12+ and iOS 15+.
  - [8aa7d61](https://github.com/tauri-apps/wry/commit/8aa7d61cdc9fc584805b46c3ffd700aabb633649) Fix: Remove extra soft prompt asking for media permission on every app launch in macOS ([#694](https://github.com/tauri-apps/wry/pull/694)) on 2022-09-29
- Focus webview when window starts moving or resizing on Windows to automatically close `<select>` dropdowns. Also notify webview2 whenever the window position/size changes which fixes the `<select>` dropdown position
  - [a1001dd](https://github.com/tauri-apps/wry/commit/a1001dd6361a0629cd1ce2f8063b7c983bf29616) fix(windows): focus webview on `WM_ENTERSIZEMOVE` and call `NotifyParentChanged` on `WM_WINDOWPOSCHANGED`. ([#695](https://github.com/tauri-apps/wry/pull/695)) on 2022-09-16
- On Windows, hide the webview when the window is minimized to reduce memory and cpu usage.
  - [51b49c5](https://github.com/tauri-apps/wry/commit/51b49c54e41c71d1c5f03b568094d43fb9dc32ac) feat(webview2): hide the webview when the window is minimized ([#702](https://github.com/tauri-apps/wry/pull/702)) on 2022-09-27
- Internally return with error from custom protocol if an invalid uri was requseted such as `wry://` which doesn't contain a host.
  - [818ce99](https://github.com/tauri-apps/wry/commit/818ce9989d816bf970ebcf93009b2d693384e436) fix: don't panic on invalid uri ([#712](https://github.com/tauri-apps/wry/pull/712)) on 2022-09-30
- Support cross compiling ios on a non macos host.
  - [cd08410](https://github.com/tauri-apps/wry/commit/cd08410bce326c42e8fc25a74290d254468724fe) Fix cross compilation. ([#731](https://github.com/tauri-apps/wry/pull/731)) on 2022-10-29
- On Linux, Improve custom protocol with http headers / method added to request, and status code / http headers added to response. This feature is 2.36 only, version below it will fallback to previous implementation.
  - [2944d91](https://github.com/tauri-apps/wry/commit/2944d91c763ff105288aa6c1370ba42a54fa8caf) feat(linux): add headers to URL scheme request ([#721](https://github.com/tauri-apps/wry/pull/721)) on 2022-10-17
- On macOS, add WKWebview as subview of existing NSView directly.
  - [008eca8](https://github.com/tauri-apps/wry/commit/008eca871155f393e5de1053bb1a9f63e1eafe82) On macOS, add WKWebview as subview of existing NSView directly ([#745](https://github.com/tauri-apps/wry/pull/745)) on 2022-11-07
- Keypress on non-input element no longer triggers unsupported key feedback sound.
  - [51c7f12](https://github.com/tauri-apps/wry/commit/51c7f12d80e2b51a188fb644a323abaf5df1b3d1) fix(macos): do not trigger unsupported key feedback sound on keypress ([#742](https://github.com/tauri-apps/wry/pull/742)) on 2022-10-30
- Remove the IPC script message handler when the WebView is dropped on macOS.
  - [818ce99](https://github.com/tauri-apps/wry/commit/818ce9989d816bf970ebcf93009b2d693384e436) fix: don't panic on invalid uri ([#712](https://github.com/tauri-apps/wry/pull/712)) on 2022-09-30
- **Breaking change** Removed http error variants from `wry::Error` and replaced with generic `HttpError` variant that can be used to convert `http` crate errors.
  - [1510e45](https://github.com/tauri-apps/wry/commit/1510e452547a95af2e42ff5199640877beecdbd7) refactor: use `http` crate primitives instead of a custom impl ([#706](https://github.com/tauri-apps/wry/pull/706)) on 2022-09-29
- Disabled Microsoft SmartScreen by default on Windows.
  - [a617c5b](https://github.com/tauri-apps/wry/commit/a617c5b29da3d173d43aa814106e1c7ace08d27f) feat(webview2): disable smartscreen & allow disabling internal webview2 args, closes [#704](https://github.com/tauri-apps/wry/pull/704) ([#705](https://github.com/tauri-apps/wry/pull/705)) on 2022-09-28
- Add `WebView::url` to get the current url.
  - [38e49bd](https://github.com/tauri-apps/wry/commit/38e49bd5f1e26e9f9507d1f2af8b0b290aa515ad) feat: add `WebView::url()` to access the current url ([#732](https://github.com/tauri-apps/wry/pull/732)) on 2022-10-25
- **Breaking change** Removed `http` module and replaced with re-export of `http` crate.
  - [1510e45](https://github.com/tauri-apps/wry/commit/1510e452547a95af2e42ff5199640877beecdbd7) refactor: use `http` crate primitives instead of a custom impl ([#706](https://github.com/tauri-apps/wry/pull/706)) on 2022-09-29
- Add `WebviewBuilderExtWindows::with_additionl_browser_args` method to pass additional browser args to Webview2 On Windows. By default wry passes `--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection` so if you use this method, you also need to disable these components by yourself if you want.
  - [683f866](https://github.com/tauri-apps/wry/commit/683f86665366bb333cb03e05a503a69d0f8eb734) feat(webview2): add method to pass additional args, closes [#415](https://github.com/tauri-apps/wry/pull/415) ([#711](https://github.com/tauri-apps/wry/pull/711)) on 2022-09-29
- On Windows, fix canonical reason for custom protocol response.
  - [9d5595c](https://github.com/tauri-apps/wry/commit/9d5595c9c723b3f8046d9582ac086ccebf460a83) fix(webview2): set response reason correctly, closes [#733](https://github.com/tauri-apps/wry/pull/733) ([#734](https://github.com/tauri-apps/wry/pull/734)) on 2022-10-24
- On macOS, make the webview first responder.
  - [e64ad21](https://github.com/tauri-apps/wry/commit/e64ad21ad5ab9bf0b7fb15aec0065c20b61a5a80) fix(wkwebview): make webview first responder ([#740](https://github.com/tauri-apps/wry/pull/740)) on 2022-10-28

## \[0.21.1]

- Fix transparency on Windows
  - [e31cd0a](https://github.com/tauri-apps/wry/commit/e31cd0adf4ba881a35dcccd9b5ee78bb5af8828a) fix: fix transparency on Windows, closes [#692](https://github.com/tauri-apps/wry/pull/692) on 2022-09-16

## \[0.21.0]

- Implement `<input type="file">` on Android.
  - [bf39d9d](https://github.com/tauri-apps/wry/commit/bf39d9de1e997170e9efb3bb7392710b57c2ae1f) feat(android): implement dialogs and permissions ([#685](https://github.com/tauri-apps/wry/pull/685)) on 2022-09-05
- Add `WebviewExtAndroid::handle` which can be used to execute some code using JNI context.
  - [2bfc6c3](https://github.com/tauri-apps/wry/commit/2bfc6c3d2e0cc6c3922d125f678ab30c00b89483) feat(android): JNI execution handle ([#689](https://github.com/tauri-apps/wry/pull/689)) on 2022-09-07
- Enable JS alert, confirm, prompt on Android.
  - [bf39d9d](https://github.com/tauri-apps/wry/commit/bf39d9de1e997170e9efb3bb7392710b57c2ae1f) feat(android): implement dialogs and permissions ([#685](https://github.com/tauri-apps/wry/pull/685)) on 2022-09-05
- Prompt for permissions on Android when needed.
  - [bf39d9d](https://github.com/tauri-apps/wry/commit/bf39d9de1e997170e9efb3bb7392710b57c2ae1f) feat(android): implement dialogs and permissions ([#685](https://github.com/tauri-apps/wry/pull/685)) on 2022-09-05
- Implement `webview_version` on Android.
  - [9183de4](https://github.com/tauri-apps/wry/commit/9183de4f9d3129e7cba332eebca2afc846f727d0) feat(android): implement webview_version ([#687](https://github.com/tauri-apps/wry/pull/687)) on 2022-09-05
- Enable storage, geolocation, media playback, `window.open`.
  - [9dfffcf](https://github.com/tauri-apps/wry/commit/9dfffcfe12199d7f28bf4b8a837e28253958ac17) feat(android): enable storage, geolocation, media playback, window.open ([#684](https://github.com/tauri-apps/wry/pull/684)) on 2022-09-04
- Improve Android initialization script implementation.
  - [1b26d60](https://github.com/tauri-apps/wry/commit/1b26d605d6e33f5417eb6566a7381d8feb239c8b) feat(android): improve initialization scripts implementation ([#670](https://github.com/tauri-apps/wry/pull/670)) on 2022-08-24
- WRY will now generate the needed kotlin files at build time but you need to set `WRY_ANDROID_REVERSED_DOMAIN`, `WRY_ANDROID_APP_NAME_SNAKE_CASE` and `WRY_ANDROID_KOTLIN_FILES_OUT_DIR` env vars.
  - [b478903](https://github.com/tauri-apps/wry/commit/b4789034dc4d10ab83f6acce6b4152d79f702940) feat(android): generate kotlin files at build time ([#671](https://github.com/tauri-apps/wry/pull/671)) on 2022-08-24
  - [103f255](https://github.com/tauri-apps/wry/commit/103f255903bdf728bf5124fb323293d172c8dd12) chore: change bump to patch on 2022-08-25
- **Breaking change** Removed `WebView::focus`.
  - [f338df7](https://github.com/tauri-apps/wry/commit/f338df7a2716cbbde357b81d9baa108ce679eaa5) feat(windows): auto-focus the webview ([#676](https://github.com/tauri-apps/wry/pull/676)) on 2022-08-27
- Updated tao to `0.14`
  - [483bad0](https://github.com/tauri-apps/wry/commit/483bad0fc7e7564500f7183547c15604fa387258) feat: tao as window dependency ([#230](https://github.com/tauri-apps/wry/pull/230)) on 2021-05-03
  - [51430e9](https://github.com/tauri-apps/wry/commit/51430e97dfb6589c5ff71e5078438be67293d044) publish new versions ([#221](https://github.com/tauri-apps/wry/pull/221)) on 2021-05-09
  - [0cf0089](https://github.com/tauri-apps/wry/commit/0cf0089b6d49aa9e1a8c791ec8883fce48a0dfd1) Update tao to v0.2.6 ([#271](https://github.com/tauri-apps/wry/pull/271)) on 2021-05-18
  - [a76206c](https://github.com/tauri-apps/wry/commit/a76206c11fa0a4ba1d041aa0f25452dd80941ee9) publish new versions ([#272](https://github.com/tauri-apps/wry/pull/272)) on 2021-05-18
  - [3c4f8b8](https://github.com/tauri-apps/wry/commit/3c4f8b8b2bd42e7634b889aa5317d909bfce593c) Update tao to v0.5 ([#365](https://github.com/tauri-apps/wry/pull/365)) on 2021-08-09
  - [44aa1dc](https://github.com/tauri-apps/wry/commit/44aa1dc8fcc20cc5826697d69f763118d45f724a) publish new versions ([#351](https://github.com/tauri-apps/wry/pull/351)) on 2021-08-09
  - [935cc5f](https://github.com/tauri-apps/wry/commit/935cc5fe8b73055279dc107e71a10f2701ea8b3d) Update tao to 0.13 ([#642](https://github.com/tauri-apps/wry/pull/642)) on 2022-07-27
  - [657888a](https://github.com/tauri-apps/wry/commit/657888aac13830d97d2970bdf1c87319dadb2ffa) Publish New Versions ([#632](https://github.com/tauri-apps/wry/pull/632)) on 2022-07-27
  - [3a91376](https://github.com/tauri-apps/wry/commit/3a91376fa2c04783a32804e6f123722749ad595e) chore(deps): update tao to 0.14 ([#691](https://github.com/tauri-apps/wry/pull/691)) on 2022-09-13
- Allow setting the webview background color.
  - [eb1b723](https://github.com/tauri-apps/wry/commit/eb1b7234f731759b5e091f7c88ac18ce4b507017) feat: allow setting webview bg color, closes [#197](https://github.com/tauri-apps/wry/pull/197) ([#682](https://github.com/tauri-apps/wry/pull/682)) on 2022-09-05
- Added the `RustWebView` class on Android.
  - [b1e8560](https://github.com/tauri-apps/wry/commit/b1e8560c3f13f2674528f6ca440ba476ddbef7c2) feat(android): define WebView class in kotlin ([#672](https://github.com/tauri-apps/wry/pull/672)) on 2022-08-24
- Update the `windows` crate to the latest 0.39.0 release and `webview2-com` to 0.19.1 to match.
  - [c7d7e1f](https://github.com/tauri-apps/wry/commit/c7d7e1f9c85a5db9c98aa5ded1e0eaf7fe697817) Update windows to 0.39.0 and webview2-com to 0.19.1 to match ([#679](https://github.com/tauri-apps/wry/pull/679)) on 2022-08-31
- On Windows, automatically focus the webview when the window gains focus to match other platforms.
  - [f338df7](https://github.com/tauri-apps/wry/commit/f338df7a2716cbbde357b81d9baa108ce679eaa5) feat(windows): auto-focus the webview ([#676](https://github.com/tauri-apps/wry/pull/676)) on 2022-08-27

## \[0.20.2]

- Implement custom protocol on Android.
  - [dc68289](https://github.com/tauri-apps/wry/commit/dc68289169196419b8c9cda73c73b139ea1301f9) feat(android): implement custom protocol ([#656](https://github.com/tauri-apps/wry/pull/656)) on 2022-08-13
- Implement `WebView::eval` on Android.
  - [690fd26](https://github.com/tauri-apps/wry/commit/690fd26a3b9bd47f9d7b1b5d2aa3dcb1c018a771) feat(android): implement eval ([#658](https://github.com/tauri-apps/wry/pull/658)) on 2022-08-13
- On iOS, add webview as subview instead of replacing original view.
  - [74391e0](https://github.com/tauri-apps/wry/commit/74391e0769d0e0f4be015147ddfa39bf25c90928) fix(ios): addSubview instead of setContentView ([#655](https://github.com/tauri-apps/wry/pull/655)) on 2022-08-13
- Move WebView logic from tao to wry.
  - [aba1ae5](https://github.com/tauri-apps/wry/commit/aba1ae5afcf96c88b1215ef66f38a5a635ecf7c3) refactor(android): move WebView logic from tao to wry ([#659](https://github.com/tauri-apps/wry/pull/659)) on 2022-08-14

## \[0.20.1]

- Add android support
  - [3218091](https://github.com/tauri-apps/wry/commit/3218091aa393dca9451840d3baa44bc9371f2e1d) Add real android support [#577](https://github.com/tauri-apps/wry/pull/577)
- Enable private picture-in-picture on macos.
  - [3cfd8c9](https://github.com/tauri-apps/wry/commit/3cfd8c9e7a43f6c35a1ea61358521bd62fc70633) fix: add feature flag to enable private picture-in-picture flag on macos ([#645](https://github.com/tauri-apps/wry/pull/645)) on 2022-08-05
- On macOS, fix devtool warning
  - [2eba8c9](https://github.com/tauri-apps/wry/commit/2eba8c9c26ff5f9512b0039ac04bc7fd27a5256f) fix: devtool warning by adding parent view

## \[0.20.0]

- Add `WebViewBuilder::with_clipboard`.
  - [c798700](https://github.com/tauri-apps/wry/commit/c7987004eaaf5cb7da830d574d81bd96dace0112) fix: Add `WebViewBuilder::with_clipboard`([#631](https://github.com/tauri-apps/wry/pull/631)) on 2022-07-05
- Fix typos in several files.
  - [4466250](https://github.com/tauri-apps/wry/commit/44662506ab01846c7e8767eb2f13bf0bbca7fe9a) Fix typos ([#635](https://github.com/tauri-apps/wry/pull/635)) on 2022-07-11
- Set webview2 language to match the OS language. This makes i18n functions like `new Date().toLocaleStrin()` behave correctly.
  - [e9f04d7](https://github.com/tauri-apps/wry/commit/e9f04d7e7bea576d0283d97e25faf7b356c5e959) fix: set system language to webview on windows, closes [#442](https://github.com/tauri-apps/wry/pull/442) ([#640](https://github.com/tauri-apps/wry/pull/640)) on 2022-07-26
- Update tao to 0.13.0.
  - [935cc5f](https://github.com/tauri-apps/wry/commit/935cc5fe8b73055279dc107e71a10f2701ea8b3d) Update tao to 0.13 ([#642](https://github.com/tauri-apps/wry/pull/642)) on 2022-07-27

## \[0.19.0]

- - Automatically resize the webview on Windows to align with other platforms.
- **Breaking change**: Removed `WebView::resize`
- [d7c9097](https://github.com/tauri-apps/wry/commit/d7c9097256d76de7400032cf27acd7a1874da5cd) feat: auto resize webview on Windows ([#628](https://github.com/tauri-apps/wry/pull/628)) on 2022-06-27
- Implement new window requested handler
  - [fa5456c](https://github.com/tauri-apps/wry/commit/fa5456c6abe16be17073e75f4a0205966be266b2) feat: Implement new window requested event, closes [#527](https://github.com/tauri-apps/wry/pull/527) ([#526](https://github.com/tauri-apps/wry/pull/526)) on 2022-06-19
- Re-export `url::Url`.
  - [0cb6961](https://github.com/tauri-apps/wry/commit/0cb696119b5e25292af9595fd89856116520c049) fix: re-export `url::Url` ([#612](https://github.com/tauri-apps/wry/pull/612)) on 2022-06-17
- Update tao to 0.12
  - [448837e](https://github.com/tauri-apps/wry/commit/448837e795a8f7f8dc4ac5f34b27063b108fc1f2) Update tao to 0.12 ([#629](https://github.com/tauri-apps/wry/pull/629)) on 2022-06-28

## \[0.18.3]

- Update tao to 0.11
  - [f4b42fb](https://github.com/tauri-apps/wry/commit/f4b42fb412fa557188f20b72ef6c4314d1d6bb91) Update tao to v0.12 ([#609](https://github.com/tauri-apps/wry/pull/609)) on 2022-06-15

## \[0.18.2]

- Fix NSString can not be released.
  - [95ca52f](https://github.com/tauri-apps/wry/commit/95ca52f5d8ca86b64f8587a0f96cf0fb7dc22125) fix: NSString isn't released ([#604](https://github.com/tauri-apps/wry/pull/604)) on 2022-06-07

## \[0.18.1]

- Remove unused tray from doc features.
  - [5eecb00](https://github.com/tauri-apps/wry/commit/5eecb0074397efa40351b3caa8fd4a6d972c4c85) Remove unused tray from doc features ([#602](https://github.com/tauri-apps/wry/pull/602)) on 2022-05-31

## \[0.18.0]

- Remove trivial tray features.
  - [a3fea48](https://github.com/tauri-apps/wry/commit/a3fea48d2d78ebe4fa3f08b40d2c3c8c8135bb12) Remove trivial tray features ([#599](https://github.com/tauri-apps/wry/pull/599)) on 2022-05-31

## \[0.17.0]

- Add option to enable/disable zoom shortcuts for WebView2, disabled by default.
  - [494a110](https://github.com/tauri-apps/wry/commit/494a11057f9ddd2bf4bcecdc96b43ed95c5bd08e) WebView2: Enable/disable platform default zooming shortcuts, closes [#569](https://github.com/tauri-apps/wry/pull/569) ([#574](https://github.com/tauri-apps/wry/pull/574)) on 2022-05-15
- Prevent memory leak on macOS.
  - [16d1924](https://github.com/tauri-apps/wry/commit/16d192450ed639f94cf8b7137fa5fea1a319f8b5) fix: prevent memory leak on macOS, closes [#536](https://github.com/tauri-apps/wry/pull/536) ([#587](https://github.com/tauri-apps/wry/pull/587)) on 2022-05-20
- Update the `windows` crate to the latest 0.37.0 release and `webview2-com` to 0.16.0 to match.

The `#[implement]` macro in `windows-implement` and the `implement` feature in `windows` depend on some `const` generic features which stabilized in `rustc` 1.61. The MSRV on Windows targets is effectively 1.61, but other targets do not require these features.

The `webview2-com` crate specifies `rust-version = "1.61"`, so `wry` will inherit that MSRV and developers on Windows should get a clear error message telling them to update their toolchain when building `wry` or anything that depends on `wry`. Developers targeting other platforms should be able to continue using whatever toolchain they were using before.

- [9d9d9d8](https://github.com/tauri-apps/wry/commit/9d9d9d8f3d37a283bbb707d39c3aac090325a63e) Update windows-rs to 0.37.0 and webview2-com to 0.16.0 to match ([#592](https://github.com/tauri-apps/wry/pull/592)) on 2022-05-23

## \[0.16.2]

- Fixed build on macos.
  - [17ab12d](https://github.com/tauri-apps/wry/commit/17ab12ded27949474f687640faebb5cc376327c5) fix: fix build on macos, closes [#580](https://github.com/tauri-apps/wry/pull/580) ([#581](https://github.com/tauri-apps/wry/pull/581)) on 2022-05-10

## \[0.16.1]

- Fixes a crash on macOS below Big Sur due to `titlebarSeparatorStyle` (11+ API) usage.
  - [eb2dddb](https://github.com/tauri-apps/wry/commit/eb2dddb611f7fadf35bf7d7c32cb6d054da9fe9e) fix(macos): only use APIs when supported on 2022-05-08
- Only run `WebView::print` on macOS on v11+. This prevents a crash on older versions.
  - [eb2dddb](https://github.com/tauri-apps/wry/commit/eb2dddb611f7fadf35bf7d7c32cb6d054da9fe9e) fix(macos): only use APIs when supported on 2022-05-08

## \[0.16.0]

- Fixes a typo in the `WebviewExtMacOS` conditional compilation.
  - [10d7f03](https://github.com/tauri-apps/wry/commit/10d7f03f403e9c373fe80897308393e0bb67a06d) fix(macos): typo in the WebviewExtMacOS conditional compilation ([#568](https://github.com/tauri-apps/wry/pull/568)) on 2022-05-02
- Fixes a crash when the custom protocol response is empty on macOS.
  - [67809f4](https://github.com/tauri-apps/wry/commit/67809f4d8abe1a042b2cdb616b03f6a2c50652b8) fix(macos): crash when custom protocol response is empty ([#567](https://github.com/tauri-apps/wry/pull/567)) on 2022-05-01
- Add `WebView::zoom` method.
  - [34b6cbc](https://github.com/tauri-apps/wry/commit/34b6cbca76811966cedf8050ae0d0fa18c84aa34) feat: add feature to zoom webview contents, closes [#388](https://github.com/tauri-apps/wry/pull/388) ([#564](https://github.com/tauri-apps/wry/pull/564)) on 2022-05-02
- Set the titlebar separator style in macOS to `none`.
  - [9776fc4](https://github.com/tauri-apps/wry/commit/9776fc466b5f3a6ef47956ec5c9cdd9c5164046a) fix(macos): set titlebar style to `none` ([#566](https://github.com/tauri-apps/wry/pull/566)) on 2022-05-01
- Disable webview2 mini menu
  - [ed0b223](https://github.com/tauri-apps/wry/commit/ed0b2230c285991b7a4588c8045111f04a67a16f) fix: disable WebView2 mini menu ("OOUI"), closes [#535](https://github.com/tauri-apps/wry/pull/535) ([#559](https://github.com/tauri-apps/wry/pull/559)) on 2022-04-29

## \[0.15.1]

- Update how android handles url
  - [427cf92](https://github.com/tauri-apps/wry/commit/427cf9222d7152f911aa70eb778eb7aa90c83fac) Unify custom protocol across Android/iOS ([#546](https://github.com/tauri-apps/wry/pull/546)) on 2022-04-11
- Add devtools support on Android/iOS.
  - [1c5d77a](https://github.com/tauri-apps/wry/commit/1c5d77a8ce79e75705a71c659af86541d50c5007) Add devtools support on Android/iOS ([#548](https://github.com/tauri-apps/wry/pull/548)) on 2022-04-11
- Fix to reset process on MacOS when webview is closed, closes #536.
  - [fd1dcc3](https://github.com/tauri-apps/wry/commit/fd1dcc3cc5a290bfe4ae8de04064074109902432) fix: reset background process when webview is closed, closes [#536](https://github.com/tauri-apps/wry/pull/536) ([#556](https://github.com/tauri-apps/wry/pull/556)) on 2022-04-24

## \[0.15.0]

- On Windows and Linux, disable resizing maximized borderless windows.
  - [313eaea](https://github.com/tauri-apps/wry/commit/313eaea0ff123bddbc8b5c337ded05d464d3dfaa) fix(win,linux): disable resizing maximized borderless windows ([#533](https://github.com/tauri-apps/wry/pull/533)) on 2022-03-30
- Fixes a memory leak on the custom protocol response body on macOS.
  - [36b985e](https://github.com/tauri-apps/wry/commit/36b985e939f4769f9835b4865ee1013229ec7539) fix(macos): custom protocol memory leak ([#539](https://github.com/tauri-apps/wry/pull/539)) on 2022-04-03
- Update tao to v0.8.0.
  - [1c540b0](https://github.com/tauri-apps/wry/commit/1c540b01fa08e84c199b8ded726b6ec77b40f015) feat: update tao to 0.8, refactor tray features ([#541](https://github.com/tauri-apps/wry/pull/541)) on 2022-04-07
- The `tray` and `ayatana-tray` Cargo features are not enabled by default.
  - [1c540b0](https://github.com/tauri-apps/wry/commit/1c540b01fa08e84c199b8ded726b6ec77b40f015) feat: update tao to 0.8, refactor tray features ([#541](https://github.com/tauri-apps/wry/pull/541)) on 2022-04-07
- **Breaking change:** Renamed the `ayatana` Cargo feature to `ayatana-tray` and added the `gtk-tray` feature. The default tray on Linux is now `libayatana-appindicator`.
  - [1c540b0](https://github.com/tauri-apps/wry/commit/1c540b01fa08e84c199b8ded726b6ec77b40f015) feat: update tao to 0.8, refactor tray features ([#541](https://github.com/tauri-apps/wry/pull/541)) on 2022-04-07

## \[0.14.0]

- Added `close_devtools` function to `Webview`.
  - [bf3b710](https://github.com/tauri-apps/wry/commit/bf3b7107631f14567b0b5ff1947c2bff1ffa2603) feat: add function to close the devtool and check if it is opened ([#529](https://github.com/tauri-apps/wry/pull/529)) on 2022-03-28
- Hide the devtool functions behind the `any(debug_assertions, feature = "devtools")` flag.
  - [bf3b710](https://github.com/tauri-apps/wry/commit/bf3b7107631f14567b0b5ff1947c2bff1ffa2603) feat: add function to close the devtool and check if it is opened ([#529](https://github.com/tauri-apps/wry/pull/529)) on 2022-03-28
- **Breaking change:** Renamed the `devtool` function to `open_devtools`.
  - [bf3b710](https://github.com/tauri-apps/wry/commit/bf3b7107631f14567b0b5ff1947c2bff1ffa2603) feat: add function to close the devtool and check if it is opened ([#529](https://github.com/tauri-apps/wry/pull/529)) on 2022-03-28
- Enable tab navigation on macOS.
  - [28ebedc](https://github.com/tauri-apps/wry/commit/28ebedc41f9017fed3fe1dc3a6d021c69f88ef5d) fix(macOS): enable tab navigation on all elements, fixes [#406](https://github.com/tauri-apps/wry/pull/406) ([#512](https://github.com/tauri-apps/wry/pull/512)) on 2022-03-03
- Added `is_devtools_open` function to `Webview`.
  - [bf3b710](https://github.com/tauri-apps/wry/commit/bf3b7107631f14567b0b5ff1947c2bff1ffa2603) feat: add function to close the devtool and check if it is opened ([#529](https://github.com/tauri-apps/wry/pull/529)) on 2022-03-28
- - Expose methods to access the underlying native handles of the webview.
- **Breaking change**: `WebviewExtWindows::controller` now returns the controller directly and not wrapped in an `Option`
- [e54afec](https://github.com/tauri-apps/wry/commit/e54afec43b767ffdb43debbd526d249c3c5b5490) feat: expose webview native handles, closes [#495](https://github.com/tauri-apps/wry/pull/495) ([#513](https://github.com/tauri-apps/wry/pull/513)) on 2022-03-03
- Add navigation handler to decide if an url is allowed to navigate.
  - [aa8af02](https://github.com/tauri-apps/wry/commit/aa8af020ab9d88ad762f2facbfa368effb04f570) feat: Implement navigation event and cancellation, closes [#456](https://github.com/tauri-apps/wry/pull/456) ([#519](https://github.com/tauri-apps/wry/pull/519)) on 2022-03-18
- **Breaking change**: Renamed the `devtool` feature to `devtools`.
  - [bf3b710](https://github.com/tauri-apps/wry/commit/bf3b7107631f14567b0b5ff1947c2bff1ffa2603) feat: add function to close the devtool and check if it is opened ([#529](https://github.com/tauri-apps/wry/pull/529)) on 2022-03-28
- **Breaking change:** Renamed the `with_dev_tool` function to `with_devtools`.
  - [bf3b710](https://github.com/tauri-apps/wry/commit/bf3b7107631f14567b0b5ff1947c2bff1ffa2603) feat: add function to close the devtool and check if it is opened ([#529](https://github.com/tauri-apps/wry/pull/529)) on 2022-03-28

## \[0.13.3]

- Fix rustdoc generation of Windows and Mac on docs.rs.
  - [327a019](https://github.com/tauri-apps/wry/commit/327a019a07fd10ca3a42ebfb8d9d626e3b91fd05) Fix rustdoc generation of Windows and Mac on docs.rs, fix [#503](https://github.com/tauri-apps/wry/pull/503) ([#507](https://github.com/tauri-apps/wry/pull/507)) on 2022-02-27

## \[0.13.2]

- Fix cross compilation from `macOS`.
  - [c97499f](https://github.com/tauri-apps/wry/commit/c97499fb078c7c65508bf2fa3502ef95c8114ef4) fix: cross compilation from macOS ([#498](https://github.com/tauri-apps/wry/pull/498)) on 2022-02-15
- Update `webview2-com` to 0.13.0, which bumps the WebView2 SDK to 1.0.1108.44 and improves cross-compilation support.

Targeting \*-pc-windows-gnu works now, but it has some [limitations](https://github.com/wravery/webview2-rs#cross-compilation).

- [24a443c](https://github.com/tauri-apps/wry/commit/24a443ca1d90ef091eaceb0ec61bcc648499b743) Add /.changes/webview2-com-0.13.0.md on 2022-02-14

## \[0.13.1]

- Add `devtool` feature flag and configuration option.
  - [d0f307b](https://github.com/tauri-apps/wry/commit/d0f307b218c3913520efbb378e9c01a526137fdd) feat: implement `devtools` API, closes [#287](https://github.com/tauri-apps/wry/pull/287) ([#486](https://github.com/tauri-apps/wry/pull/486)) on 2022-02-07

- Update the `webview2-com` crate 0.11.0:

- Fix silent build script errors related to unconfigured nuget in https://github.com/wravery/webview2-rs/pull/4

- Update the WebView2 SDK (not the runtime, just the API bindings) to the latest 1.0.1072.54 version

- [7d4eeb7](https://github.com/tauri-apps/wry/commit/7d4eeb744bf008e43c034e865b383ee4a330e77a) Update webview2-com to 0.11.0 ([#488](https://github.com/tauri-apps/wry/pull/488)) on 2022-02-06

## \[0.13.0]

- Update gtk to 0.15
  - [219d20c](https://github.com/tauri-apps/wry/commit/219d20ce66a6bdf6c3e1af6156c9f2a74f2eed29) Merge next back to dev branch ([#477](https://github.com/tauri-apps/wry/pull/477)) on 2022-02-05
- Add clipboard field in WebViewAttributes.
  - [219d20c](https://github.com/tauri-apps/wry/commit/219d20ce66a6bdf6c3e1af6156c9f2a74f2eed29) Merge next back to dev branch ([#477](https://github.com/tauri-apps/wry/pull/477)) on 2022-02-05
- Ignore transparency on Windows 7 to prevent application crash.
  - [219d20c](https://github.com/tauri-apps/wry/commit/219d20ce66a6bdf6c3e1af6156c9f2a74f2eed29) Merge next back to dev branch ([#477](https://github.com/tauri-apps/wry/pull/477)) on 2022-02-05
- Remove clipboard property for consistency across platforms.
  - [219d20c](https://github.com/tauri-apps/wry/commit/219d20ce66a6bdf6c3e1af6156c9f2a74f2eed29) Merge next back to dev branch ([#477](https://github.com/tauri-apps/wry/pull/477)) on 2022-02-05
- Enable cookie persistence on Linux if the `data_directory` is provided.
  - [219d20c](https://github.com/tauri-apps/wry/commit/219d20ce66a6bdf6c3e1af6156c9f2a74f2eed29) Merge next back to dev branch ([#477](https://github.com/tauri-apps/wry/pull/477)) on 2022-02-05
- Enable objc's exception features so they can be treated as panic message.
  - [219d20c](https://github.com/tauri-apps/wry/commit/219d20ce66a6bdf6c3e1af6156c9f2a74f2eed29) Merge next back to dev branch ([#477](https://github.com/tauri-apps/wry/pull/477)) on 2022-02-05
- Add inner size method for webview. This can reflect correct size of webview on macOS.
  - [219d20c](https://github.com/tauri-apps/wry/commit/219d20ce66a6bdf6c3e1af6156c9f2a74f2eed29) Merge next back to dev branch ([#477](https://github.com/tauri-apps/wry/pull/477)) on 2022-02-05
- Add "transparent" and "fullscreen" feature flags on macOS to toggle private API.
  - [219d20c](https://github.com/tauri-apps/wry/commit/219d20ce66a6bdf6c3e1af6156c9f2a74f2eed29) Merge next back to dev branch ([#477](https://github.com/tauri-apps/wry/pull/477)) on 2022-02-05
- Implement WebContextImpl on mac to extend several callback lifetimes.
  - [219d20c](https://github.com/tauri-apps/wry/commit/219d20ce66a6bdf6c3e1af6156c9f2a74f2eed29) Merge next back to dev branch ([#477](https://github.com/tauri-apps/wry/pull/477)) on 2022-02-05
- The only thing that private mod shared does is re-export http mod to public,
  we can just pub mod http.
  - [219d20c](https://github.com/tauri-apps/wry/commit/219d20ce66a6bdf6c3e1af6156c9f2a74f2eed29) Merge next back to dev branch ([#477](https://github.com/tauri-apps/wry/pull/477)) on 2022-02-05
- - Fix hovering over an edge of undecorated window on Linux won't change cursor.
- Undecorated window can be resized using touch on Linux.
- [219d20c](https://github.com/tauri-apps/wry/commit/219d20ce66a6bdf6c3e1af6156c9f2a74f2eed29) Merge next back to dev branch ([#477](https://github.com/tauri-apps/wry/pull/477)) on 2022-02-05
- Update webkit2gtk to 0.15
  - [219d20c](https://github.com/tauri-apps/wry/commit/219d20ce66a6bdf6c3e1af6156c9f2a74f2eed29) Merge next back to dev branch ([#477](https://github.com/tauri-apps/wry/pull/477)) on 2022-02-05
- Add `with_user_agent(&str)` to `WebViewBuilder`.
  - [219d20c](https://github.com/tauri-apps/wry/commit/219d20ce66a6bdf6c3e1af6156c9f2a74f2eed29) Merge next back to dev branch ([#477](https://github.com/tauri-apps/wry/pull/477)) on 2022-02-05
- Replace all of the `winapi` crate references with the `windows` crate, and replace `webview2` and `webview2-sys` with `webview2-com` and `webview2-com-sys` built with the `windows` crate. The replacement bindings are in the `webview2-com-sys` crate, with `pub use` in the `webview2-com` crate. They can be shared with TAO.
  - [219d20c](https://github.com/tauri-apps/wry/commit/219d20ce66a6bdf6c3e1af6156c9f2a74f2eed29) Merge next back to dev branch ([#477](https://github.com/tauri-apps/wry/pull/477)) on 2022-02-05
- Fix null pointer crash on `get_content` of web resource request. This is a temporary fix.
  We will switch it back once upstream is updated.
  - [219d20c](https://github.com/tauri-apps/wry/commit/219d20ce66a6bdf6c3e1af6156c9f2a74f2eed29) Merge next back to dev branch ([#477](https://github.com/tauri-apps/wry/pull/477)) on 2022-02-05
- Update the `windows` crate to 0.25.0, which comes with pre-built libraries. WRY and Tao can both reference the same types directly from the `windows` crate instead of sharing bindings in `webview2-com-sys`.
  - [219d20c](https://github.com/tauri-apps/wry/commit/219d20ce66a6bdf6c3e1af6156c9f2a74f2eed29) Merge next back to dev branch ([#477](https://github.com/tauri-apps/wry/pull/477)) on 2022-02-05
- Update the `windows` crate to 0.29.0 and `webview2-com` to 0.9.0.
  - [219d20c](https://github.com/tauri-apps/wry/commit/219d20ce66a6bdf6c3e1af6156c9f2a74f2eed29) Merge next back to dev branch ([#477](https://github.com/tauri-apps/wry/pull/477)) on 2022-02-05
- Update the `windows` crate to 0.30.0 and `webview2-com` to 0.10.0.
  - [219d20c](https://github.com/tauri-apps/wry/commit/219d20ce66a6bdf6c3e1af6156c9f2a74f2eed29) Merge next back to dev branch ([#477](https://github.com/tauri-apps/wry/pull/477)) on 2022-02-05

## \[0.12.2]

- Fixed a Linux multi-window issue where the internal url loader didn't unlock when flushed while empty
  - [5377821](https://github.com/tauri-apps/wry/commit/5377821f43c0e7556ec46f0aaf4d6b0637512493) Fix async multiwindow deadlock ([#382](https://github.com/tauri-apps/wry/pull/382)) on 2021-08-16

- The custom protocol now returns a `Request` and expects a `Response`.

- This allows us to get the complete request from the Webview. (Method, GET, POST, PUT etc..)
  Read the complete header.

- And allow us to be more flexible in the future without bringing breaking changes.

- [d202573](https://github.com/tauri-apps/wry/commit/d202573c2c68a2ff0411c1aa797ecc10f727e93b) refactor: Custom protocol request/response ([#387](https://github.com/tauri-apps/wry/pull/387)) on 2021-08-22

- On Linux, automation callbacks now use the first created webview as the return value
  - [f9d7049](https://github.com/tauri-apps/wry/commit/f9d7049978bbad389c99d7a7cce9903a528d871d) Use the first created webview for webkit2gtk automation callbacks ([#383](https://github.com/tauri-apps/wry/pull/383)) on 2021-08-16

## \[0.12.1]

- Add html attributes as another method to load the page. This can provide some other origin header and make CORS request
  possible.
  - [02ad372](https://github.com/tauri-apps/wry/commit/02ad37219a1f6e5e6ed8e4da61e6a5ac021d410e) feat: html string attributes ([#368](https://github.com/tauri-apps/wry/pull/368)) on 2021-08-12
- Shorter protocol name on Windows. This can make origin be shorter too.
  - [2d9f5c9](https://github.com/tauri-apps/wry/commit/2d9f5c95e3805911d12803122fd1e83be758a769) Shorter protocol name on Windows ([#367](https://github.com/tauri-apps/wry/pull/367)) on 2021-08-12

## \[0.12.0]

- Custom Protocol handlers no longer take a `&Window` parameter.
  - [0e2574c](https://github.com/tauri-apps/wry/commit/0e2574c420f778c59bafc164ddee2bc0b7705ee9) Remove `&Window` parameter from Custom Protocol handlers ([#361](https://github.com/tauri-apps/wry/pull/361)) on 2021-07-28
- Update gtk to version 0.14. This also remove requirement of `clang`.
  - [251a80b](https://github.com/tauri-apps/wry/commit/251a80bab49d42f742a3ae6b3ca2cbfc97de98bb) Update gtk to version 0.14 ([#364](https://github.com/tauri-apps/wry/pull/364)) on 2021-08-06
- Update tao to v0.5. Please see release notes on tao for more information.
  - [483bad0](https://github.com/tauri-apps/wry/commit/483bad0fc7e7564500f7183547c15604fa387258) feat: tao as window dependency ([#230](https://github.com/tauri-apps/wry/pull/230)) on 2021-05-03
  - [51430e9](https://github.com/tauri-apps/wry/commit/51430e97dfb6589c5ff71e5078438be67293d044) publish new versions ([#221](https://github.com/tauri-apps/wry/pull/221)) on 2021-05-09
  - [0cf0089](https://github.com/tauri-apps/wry/commit/0cf0089b6d49aa9e1a8c791ec8883fce48a0dfd1) Update tao to v0.2.6 ([#271](https://github.com/tauri-apps/wry/pull/271)) on 2021-05-18
  - [a76206c](https://github.com/tauri-apps/wry/commit/a76206c11fa0a4ba1d041aa0f25452dd80941ee9) publish new versions ([#272](https://github.com/tauri-apps/wry/pull/272)) on 2021-05-18
  - [3c4f8b8](https://github.com/tauri-apps/wry/commit/3c4f8b8b2bd42e7634b889aa5317d909bfce593c) Update tao to v0.5 ([#365](https://github.com/tauri-apps/wry/pull/365)) on 2021-08-09
- Add flags to support all other possible unix systems.
  - [c0d0a78](https://github.com/tauri-apps/wry/commit/c0d0a78b893eecdc45c6cda71264020d6ae17bda) Add flags to support all other unix systems. ([#352](https://github.com/tauri-apps/wry/pull/352)) on 2021-07-21
- Support having multiple webkit2gtk `WebView`s on a single `WebContext`.
  - [3f03d6b](https://github.com/tauri-apps/wry/commit/3f03d6b5ea4e9ba81950245de156f09e72ab40a1) Support multiple webviews on a single WebContext (webkit2gtk) ([#359](https://github.com/tauri-apps/wry/pull/359)) on 2021-07-28
- On Windows, Fix cursor flickering when Tao window is without decorations
  - [e28bcce](https://github.com/tauri-apps/wry/commit/e28bcce0884937365013fda3098f64f9956d569f) fix(windows): fix mouse style flicker when `decorations: false` ([#350](https://github.com/tauri-apps/wry/pull/350)) on 2021-07-20
- Remove winrt support since it's outdated for a long time. We will reimplement it again once `windws-rs` is stable!
  - [c37973e](https://github.com/tauri-apps/wry/commit/c37973e47318e9cff2712eb4a394c07734f58d54) chore(windows): remove winrt support ([#356](https://github.com/tauri-apps/wry/pull/356)) on 2021-07-24

## \[0.11.0]

- Allow resizing of borderless window on Windows
  - [bd10b8e](https://github.com/tauri-apps/wry/commit/bd10b8e5fe517edd6234ed03170741f1a51768bf) feat(Windows): resize borderless window ([#333](https://github.com/tauri-apps/wry/pull/333)) on 2021-07-15
- Mark enums as `#[non_exhaustive]` to prevent breaking changes on enum update.
  - [f07ae14](https://github.com/tauri-apps/wry/commit/f07ae144197933c28f8302105b313c2a2afc62af) refactor: add `#[non_exhaustive]` attributes to enums ([#304](https://github.com/tauri-apps/wry/pull/304)) on 2021-07-08
- Bump tao to `0.4`. Please refer to `tao` changelog for more details.
  - [6eb10d4](https://github.com/tauri-apps/wry/commit/6eb10d4e10ce86c8403c80fb41ba5e37072dc61e) bump `tao` to 0.4 and fix examples ([#329](https://github.com/tauri-apps/wry/pull/329)) on 2021-07-14
- - Add `focus` method to `Webview`
- Add `WebviewExtWindows` trait with `controller` method
- [621ed1f](https://github.com/tauri-apps/wry/commit/621ed1fff35d9389d88664d8084e1a678dfbfc36) feat: add `.focus()` to `Webview` ([#325](https://github.com/tauri-apps/wry/pull/325)) on 2021-07-05
- [96b7b94](https://github.com/tauri-apps/wry/commit/96b7b943da34ab81872553e65d2f2cd138531a62) Add controller method instead ([#326](https://github.com/tauri-apps/wry/pull/326)) on 2021-07-07
- macOS: Remove handler in the webview as it should be handled with the menu.
  - [5a9df15](https://github.com/tauri-apps/wry/commit/5a9df156f04789d4c89fdb8edf72b301667df127) fix(macos): Remove keypress handler in the webview for copy/paste/cut ([#328](https://github.com/tauri-apps/wry/pull/328)) on 2021-07-07
- Fixes multiple custom protocols registration on Windows.
  - [923d346](https://github.com/tauri-apps/wry/commit/923d3461ce93846af8dd548d4e43ebd0fd6111a3) fix(windows): multiple custom protocols, closes [#323](https://github.com/tauri-apps/wry/pull/323) ([#324](https://github.com/tauri-apps/wry/pull/324)) on 2021-07-02

## \[0.10.3]

- [#315](https://github.com/tauri-apps/wry/pull/315) fixed Webview2 runtime performance issues.
  - [d3c9b16](https://github.com/tauri-apps/wry/commit/d3c9b169d81fd8b79e6695d91b3a1d0e8042a81f) Fix Webview2 runtime performance issues ([#316](https://github.com/tauri-apps/wry/pull/316)) on 2021-06-29

## \[0.10.2]

- Fix file explorer getting blocked by automation.
  - [0c5cdd8](https://github.com/tauri-apps/wry/commit/0c5cdd8f2a6f4d07d87c6c4d1c51540ff9abfd97) Fix file explorer getting blocked by automation ([#310](https://github.com/tauri-apps/wry/pull/310)) on 2021-06-23

## \[0.10.1]

- `WebContext::set_allows_automation` is now available to specify if the context should allow automation (e.g. WebDriver).
  It is only enforced on Linux, but may expand platforms in the future.
  - [4ad0bf1](https://github.com/tauri-apps/wry/commit/4ad0bf12d186b3c313131060316aef371f45d455) move set_allows_automation to WebContext method ([#302](https://github.com/tauri-apps/wry/pull/302)) on 2021-06-21

## \[0.10.0]

- Add WebViewAttributes
  - [81f3218](https://github.com/tauri-apps/wry/commit/81f3218d9ac55a987b050f574774afcaa0b5c2f7) Add WebViewAttributes ([#286](https://github.com/tauri-apps/wry/pull/286)) on 2021-06-04
- Add `with_web_context` method that can work well with builder pattern.
  - [48f53a3](https://github.com/tauri-apps/wry/commit/48f53a3393b0c016a972a72dec45691959ac9e3b) Add `with_web_context` method ([#292](https://github.com/tauri-apps/wry/pull/292)) on 2021-06-13
- Change the custom protocol handler on macOS so it returns a response on error and a status code on success.
  - [6b869b1](https://github.com/tauri-apps/wry/commit/6b869b1ad5de9c8e9f36c1fc1b7040e10b033b52) fix(macos): custom protocol response with status code + error response ([#279](https://github.com/tauri-apps/wry/pull/279)) on 2021-05-20
- Update signature of custom protocol closure. It should return a mime type string now.
  - [cc9fc4b](https://github.com/tauri-apps/wry/commit/cc9fc4b43df79834c1b8f2c1347accba50356604) Add mimetype to return type of custom protocol ([#296](https://github.com/tauri-apps/wry/pull/296)) on 2021-06-13
- Fix webview creation when using new_any_thread of event loop.
  - [4d62cf5](https://github.com/tauri-apps/wry/commit/4d62cf5a3ddcbed06afb93d9503424a9b8110d57) Fix webview creation when using new_any_thread on Windows ([#298](https://github.com/tauri-apps/wry/pull/298)) on 2021-06-18
- Remove `Dispatcher`, `dispatch_script` and `dispatcher` in the `webview` module and add a `js` parameter to `evaluate_script`.
  - [de4a5fa](https://github.com/tauri-apps/wry/commit/de4a5fa820b1938532223677913e73720885cb54) refactor: remove `Dispatcher` and related methods, closes [#290](https://github.com/tauri-apps/wry/pull/290) ([#291](https://github.com/tauri-apps/wry/pull/291)) on 2021-06-09
- Removes the `image` dependency.
  - [1d5cc59](https://github.com/tauri-apps/wry/commit/1d5cc590856e1be1428f8516595ace6d8099f41f) chore(deps): remove `image` dependency ([#274](https://github.com/tauri-apps/wry/pull/274)) on 2021-05-19
- Bump tao to `0.3` and add more examples.

*For more details, please refer to `tao` changelog.*

- [cd4697e](https://github.com/tauri-apps/wry/commit/cd4697ebdb8eb955f0ed2be4aefea82d2c263a52) bump `tao` to 0.3 with examples ([#294](https://github.com/tauri-apps/wry/pull/294)) on 2021-06-21
- Add `wry::webview::WebContext`. It's now a required argument on `WebViewBuilder::build`.
  - [761b2b5](https://github.com/tauri-apps/wry/commit/761b2b59fe0434b3458d99ed599394af0e1e3962) webdriver support ([#281](https://github.com/tauri-apps/wry/pull/281)) on 2021-06-08

## \[0.9.4]

- Update tao to v0.2.6
  - [483bad0](https://github.com/tauri-apps/wry/commit/483bad0fc7e7564500f7183547c15604fa387258) feat: tao as window dependency ([#230](https://github.com/tauri-apps/wry/pull/230)) on 2021-05-03
  - [51430e9](https://github.com/tauri-apps/wry/commit/51430e97dfb6589c5ff71e5078438be67293d044) publish new versions ([#221](https://github.com/tauri-apps/wry/pull/221)) on 2021-05-09
  - [0cf0089](https://github.com/tauri-apps/wry/commit/0cf0089b6d49aa9e1a8c791ec8883fce48a0dfd1) Update tao to v0.2.6 ([#271](https://github.com/tauri-apps/wry/pull/271)) on 2021-05-18

## \[0.9.3]

- Expose `webview_version` function in the `webview` module.
  - [4df310e](https://github.com/tauri-apps/wry/commit/4df310e6bb508854ffc17ec915b3d0ab7c11f03d) feat: get webview version ([#259](https://github.com/tauri-apps/wry/pull/259)) on 2021-05-12
- Add print method on Linux and Windows.
  - [54c5ec7](https://github.com/tauri-apps/wry/commit/54c5ec7ae6166da5ce670ccd2ceaa108233bb845) Implement print method on Linux and Windows ([#264](https://github.com/tauri-apps/wry/pull/264)) on 2021-05-17
- Disable smooth scrolling on Linux to match behaviour on browsers.
  - [3e786bb](https://github.com/tauri-apps/wry/commit/3e786bb28793e939c00ebf0c6758d4f6cf4d3b28) Disable smooth scrolling on Linux ([#268](https://github.com/tauri-apps/wry/pull/268)) on 2021-05-17

## \[0.9.2]

- Add `tray` feature flag from tao.
  - [093c25e](https://github.com/tauri-apps/wry/commit/093c25ee68d51849b95a1a3b9341e5ad6021cecf) feat: expose tray feature flag ([#256](https://github.com/tauri-apps/wry/pull/256)) on 2021-05-10

## \[0.9.1]

- Correctly set visibility when building `Window` on gtk-backend
  - [4395ad1](https://github.com/tauri-apps/wry/commit/4395ad147b799e67f9802c499346d0ad53554317) fix: only call `show_all` when needed ([#227](https://github.com/tauri-apps/wry/pull/227)) on 2021-05-02
- Fix `macOS` cursors and other minors UI glitch.
  - [d550b2f](https://github.com/tauri-apps/wry/commit/d550b2f0a1c708747537e3a5e6d880fea00e651d) fix(macOS): Window layers ([#220](https://github.com/tauri-apps/wry/pull/220)) on 2021-04-28
- Expose `print()` function to the webview. Work only on macOS for now.
  - [5206db6](https://github.com/tauri-apps/wry/commit/5206db6ca599fe0e146d72b04c908330e3045838) fix(macOS): Printing ([#235](https://github.com/tauri-apps/wry/pull/235)) ([#236](https://github.com/tauri-apps/wry/pull/236)) on 2021-05-06
- Fix macOS windows order for tray (statusbar) applications.
  - [229275f](https://github.com/tauri-apps/wry/commit/229275f106371d79800e0ca1cbc7b6c1827bc2ac) fix: macOS windows order ([#242](https://github.com/tauri-apps/wry/pull/242)) on 2021-05-07
- Add `request_redraw` method of `Window` on Linux
  - [03abfa0](https://github.com/tauri-apps/wry/commit/03abfa06019a78a182c7cd29dc63bf3d9df10e44) Add request_redraw method on Linux ([#222](https://github.com/tauri-apps/wry/pull/222)) on 2021-04-30
- Add tao as window dependency.
  - [483bad0](https://github.com/tauri-apps/wry/commit/483bad0fc7e7564500f7183547c15604fa387258) feat: tao as window dependency ([#230](https://github.com/tauri-apps/wry/pull/230)) on 2021-05-03
- Close the window when the instance is dropped on Linux and Windows.
  - [3f2cc28](https://github.com/tauri-apps/wry/commit/3f2cc28b4fbfcf54c97000a6541e9356440838e8) fix: close window when the instance is dropped ([#228](https://github.com/tauri-apps/wry/pull/228)) on 2021-05-02
- Remove winit dependency on Linux
  - [fa15076](https://github.com/tauri-apps/wry/commit/fa15076207d9e678db4149210aba929044d0ff45) feat: winit interface for gtk ([#163](https://github.com/tauri-apps/wry/pull/163)) on 2021-04-19
  - [39d6f59](https://github.com/tauri-apps/wry/commit/39d6f595d81c857e92aef31cc2559b402e64edd3) publish new versions ([#166](https://github.com/tauri-apps/wry/pull/166)) on 2021-04-29
  - [4ef8330](https://github.com/tauri-apps/wry/commit/4ef8330d856e07d34bf86d1f2903c82c37042556) Remove winit dependency on Linux ([#226](https://github.com/tauri-apps/wry/pull/226)) on 2021-04-30

## \[0.9.0]

- Refactor signatures of most closure types
  - [b8823fe](https://github.com/tauri-apps/wry/commit/b8823fe14ee5f95d07cd2cb1f9f673b964c9dc83) refactor: signature of closure types ([#167](https://github.com/tauri-apps/wry/pull/167)) on 2021-04-19
- Drop handler closures properly on macOS.
  - [f905503](https://github.com/tauri-apps/wry/commit/f905503c4a010ed4219c6ad36d14c0dbf0b6e122) fix: [#160](https://github.com/tauri-apps/wry/pull/160) drop handler closures properly ([#211](https://github.com/tauri-apps/wry/pull/211)) on 2021-04-27
- Fix `history.pushState` in webview2.
  - [dd0fa46](https://github.com/tauri-apps/wry/commit/dd0fa46494c1ab8536bcc7ea1dd16341b12856b4) Use http instead of file for windows custom protocol workaround ([#173](https://github.com/tauri-apps/wry/pull/173)) on 2021-04-20
- The `data_directory` field now affects the IndexedDB and LocalStorage directories on Linux.
  - [1a6c821](https://github.com/tauri-apps/wry/commit/1a6c8216ee6865ca14025c229b37342496b38f26) feat(linux): implement custom user data path ([#188](https://github.com/tauri-apps/wry/pull/188)) on 2021-04-22
- Fix runtime panic on macOS, when no file handler are defined.
  - [22a4991](https://github.com/tauri-apps/wry/commit/22a4991aa8ca7c75aa52150a90379c40bcc34d07) bug(macOS): Runtime panic when no file_drop_handler ([#177](https://github.com/tauri-apps/wry/pull/177)) on 2021-04-20
- Add position field on WindowAttribute
  - [2b3be7a](https://github.com/tauri-apps/wry/commit/2b3be7a4db2cbc1612c7105cb698c1f21a05da77) Add position field on WindowAttribute ([#219](https://github.com/tauri-apps/wry/pull/219)) on 2021-04-28
- Fix panic on multiple custom protocols registration.
  - [01647a2](https://github.com/tauri-apps/wry/commit/01647a2a5b769bc192754c2d3806a55112d58d33) Fix custom protocol registry on mac ([#205](https://github.com/tauri-apps/wry/pull/205)) on 2021-04-26
- Fix SVG render with the custom protocol.
  - [890cfe5](https://github.com/tauri-apps/wry/commit/890cfe527996c181d643c9f8e5fc3e79ff0841a0) fix(custom-protocol): SVG mime type - close [#168](https://github.com/tauri-apps/wry/pull/168) ([#169](https://github.com/tauri-apps/wry/pull/169)) on 2021-04-19
- Initial custom WindowExtWindows trait.
  - [1ef1f58](https://github.com/tauri-apps/wry/commit/1ef1f58efb6afa6c6b9eda3a43ee83fc79c3b78e) feat: custom WindowExtWindow trait ([#191](https://github.com/tauri-apps/wry/pull/191)) on 2021-04-23
- Fix transparency on Windows
  - [e278556](https://github.com/tauri-apps/wry/commit/e2785566c69d43f003896b7b5da79b29d2966c13) fix: transparency on Windows  ([#217](https://github.com/tauri-apps/wry/pull/217)) on 2021-04-28
- Add platform module and WindowExtUnix trait on Linux
  - [004e298](https://github.com/tauri-apps/wry/commit/004e298e0198e6576a11e6e84fdf6b7c2f66b6ae) feat: WindowExtUnix trait ([#192](https://github.com/tauri-apps/wry/pull/192)) on 2021-04-23
- Make sure custom protocol on Windows is over HTTPS.
  - [c36db35](https://github.com/tauri-apps/wry/commit/c36db35b2b8704eb36bc341cd99abac01abfab87) fix(custom-protocol): Make sure custom protocol on Windows is over HTTPS. ([#179](https://github.com/tauri-apps/wry/pull/179)) on 2021-04-20
- Initial winit interface for gtk backend
  - [fa15076](https://github.com/tauri-apps/wry/commit/fa15076207d9e678db4149210aba929044d0ff45) feat: winit interface for gtk ([#163](https://github.com/tauri-apps/wry/pull/163)) on 2021-04-19

## \[0.8.0]

- Wry now accepts multiple custom protocol registrations.
  - [db64fc6](https://github.com/tauri-apps/wry/commit/db64fc69c48a728184fcef001688b94f0294edab) feat/licenses ([#155](https://github.com/tauri-apps/wry/pull/155)) on 2021-04-14
- Apply license header for SPDX compliance.
  - [05e0218](https://github.com/tauri-apps/wry/commit/05e02180c9fe929d3e691185df44257654546935) feat: multiple custom protocols ([#151](https://github.com/tauri-apps/wry/pull/151)) on 2021-04-11
  - [db64fc6](https://github.com/tauri-apps/wry/commit/db64fc69c48a728184fcef001688b94f0294edab) feat/licenses ([#155](https://github.com/tauri-apps/wry/pull/155)) on 2021-04-14
- Remove bindings crate and use windows-webview2 as dependency instead.
  - [c2156a4](https://github.com/tauri-apps/wry/commit/c2156a45d7fbfead956b6d03b2594962e3455e6d) Move to windows-webview2 as dependency for winrt impl ([#144](https://github.com/tauri-apps/wry/pull/144)) on 2021-04-03

## \[0.7.0]

- Add old win32 implementation on windows as default feature flag.
  - [1a88cd2](https://github.com/tauri-apps/wry/commit/1a88cd267f2a29c1dd35d7197250972718081847) refactor: Add win32 implementation and feature flag for both backends ([#139](https://github.com/tauri-apps/wry/pull/139)) on 2021-04-02
- Adds a `WindowProxy` to the file drop handler closure - `WindowFileDropHandler`.
  - [20cb051](https://github.com/tauri-apps/wry/commit/20cb051aba28009c70dad838b2a9b1575cb5363a) feat: add WindowProxy to file drop handler closure ([#140](https://github.com/tauri-apps/wry/pull/140)) on 2021-04-01

## \[0.6.2]

- Add pipe back to version check for covector config. This prevents the CI failure on publish if it exists already. The issue was patched in covector (and tests in place so it doesn't break in the future).
  - [a32829c](https://github.com/tauri-apps/wry/commit/a32829c527f02b228fa1da45e9710941c5415bfc) chore: add pipe for publish check back in ([#131](https://github.com/tauri-apps/wry/pull/131)) on 2021-03-28
- Fix messages to the webview from the backend being delayed on Linux/GTK when the user is not actively engaged with the UI.
  - [d2a2a9f](https://github.com/tauri-apps/wry/commit/d2a2a9f473d2588b27a95bf627d125caea1b979d) fix: spawn async event loop on gtk to prevent delayed messages ([#135](https://github.com/tauri-apps/wry/pull/135)) on 2021-03-31
- Add draggable regions, just add `drag-region` class to the html element.
  - [b2a0bfc](https://github.com/tauri-apps/wry/commit/b2a0bfc289786d0a23dac0c8d9543771e70e3427) feat/ draggable-region ([#92](https://github.com/tauri-apps/wry/pull/92)) on 2021-03-25
- Add event listener in application proxy
  - [c49846c](https://github.com/tauri-apps/wry/commit/c49846cfc41bb548a685edeac5f8036501f7dcec) feat: event listener ([#129](https://github.com/tauri-apps/wry/pull/129)) on 2021-03-26
- Better result error handling
  - [485035f](https://github.com/tauri-apps/wry/commit/485035f17d28560966b07b512935821814f0e951) chore: better result error handling ([#124](https://github.com/tauri-apps/wry/pull/124)) on 2021-03-21
- Fix visibility on webview2 when window was invisible previously and then shown.
  - [6d31706](https://github.com/tauri-apps/wry/commit/6d31706a6bff43e9b28100675cf8fc12f29db248) Fix visibility on webview2 when window was invisible previously ([#128](https://github.com/tauri-apps/wry/pull/128)) on 2021-03-24

## \[0.6.1]

- Add attribute option to allow WebView on Windows use user_data folder
  - [8dd58ee](https://github.com/tauri-apps/wry/commit/8dd58eec77d4c89491b1af427d06c4ee6cfa8e58) feat/ allow webview2 (windows) to use optional user_data folder provided by the attributes ([#120](https://github.com/tauri-apps/wry/pull/120)) on 2021-03-21

## \[0.6.0]

- Initialize covector!
  - [33b64ed](https://github.com/tauri-apps/wry/commit/33b64ed5c208b778d03dbb5f3f2808bb417c9f52) chore: covector init ([#55](https://github.com/tauri-apps/wry/pull/55)) on 2021-02-21
- Support Windows 7, 8, and 10
  - [fbf0d17](https://github.com/tauri-apps/wry/commit/fbf0d17164da455400aaa44104c3925eded09393) Adopt Webview2 on Windows ([#48](https://github.com/tauri-apps/wry/pull/48)) on 2021-02-20
- Dev tools are enabled on debug build
- Add skip task bar option
  - [395b6fb](https://github.com/tauri-apps/wry/commit/395b6fbcd66f6cbd0457cb609bea4afe734fadd4) feat: `skip_taskbar` for windows ([#49](https://github.com/tauri-apps/wry/pull/49)) on 2021-02-20
- Add custom protocol option
  - [a492806](https://github.com/tauri-apps/wry/commit/7a492806d716a30abe15a2104b64152c1ca370bb) Add custom protocol ([#65](https://github.com/tauri-apps/wry/pull/65)) on 2021-02-23
- Add transparent option to mac and linux
- Error type has Send/Sync traits
  - [3536b83](https://github.com/tauri-apps/wry/commit/3536b831ec30ee7436616ba4b262bbdd1e6279c8) Add .changes file in prepare of v0.6 on 2021-02-24
- Replace Callback with RPC handler
  - [e215157](https://github.com/tauri-apps/wry/commit/e215157146f0eab8ee6beab0628b036c68eea108) Implement draft RPC API ([#95](https://github.com/tauri-apps/wry/pull/95)) on 2021-03-04
- Add File drop handlers
  - [fed0ee7](https://github.com/tauri-apps/wry/commit/fed0ee772100ad19a344a85266618c7bcf7cb649) File drop handlers ([#96](https://github.com/tauri-apps/wry/pull/96)) on 2021-03-09
