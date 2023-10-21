# Changelog

## \[0.1.2]

- Expose `native-host` option in tauri-driver and set default to `127.0.0.1`.
  - [cd9dfc7b](https://www.github.com/tauri-apps/tauri/commit/cd9dfc7b9a3fe0e04e40d9b0f9be674aefd0d725) fix(driver): expose native-host option and set default to 127.0.0.1 ([#3816](https://www.github.com/tauri-apps/tauri/pull/3816)) on 2022-03-31

## \[0.1.1]

- The minimum Rust version is now `1.56`.
  - [a9dfc015](https://www.github.com/tauri-apps/tauri/commit/a9dfc015505afe91281c2027954ffcc588b1a59c) feat: update to edition 2021 and set minimum rust to 1.56 ([#2789](https://www.github.com/tauri-apps/tauri/pull/2789)) on 2021-10-22
- Add `args` field (array of application CLI arguments) to the `tauri:options` capabilities.
  - [d0970e34](https://www.github.com/tauri-apps/tauri/commit/d0970e3499297a6c102a36f2dc479d3d657bfaf3) feat(driver): add `args` to `tauri:options` ([#3154](https://www.github.com/tauri-apps/tauri/pull/3154)) on 2022-01-03

## \[0.1.0]

- Initial release including Linux and Windows support.
  - [be76fb1d](https://www.github.com/tauri-apps/tauri/commit/be76fb1dfe73a1605cc2ad246418579f4c2e1999) WebDriver support ([#1972](https://www.github.com/tauri-apps/tauri/pull/1972)) on 2021-06-23
  - [c22e5a7c](https://www.github.com/tauri-apps/tauri/commit/c22e5a7c2ebede41657973b80eff6b68106817fc) fix(covector): keep `tauri-driver` version as alpha on 2021-06-23
  - [b4426eda](https://www.github.com/tauri-apps/tauri/commit/b4426eda9e64fcdd25a2d72e548b8b0fbfa09619) Revert "WebDriver support ([#1972](https://www.github.com/tauri-apps/tauri/pull/1972))" on 2021-06-23
  - [4b2aa356](https://www.github.com/tauri-apps/tauri/commit/4b2aa35684632ed2afd7dec4ad848df5704868e4) Add back WebDriver support ([#2324](https://www.github.com/tauri-apps/tauri/pull/2324)) on 2021-08-01
