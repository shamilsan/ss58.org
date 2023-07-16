# SS58 Address Format Online Converter

[![Build][build_badge]][build_href]
[![License][lic_badge]][lic_href]

[build_badge]: https://github.com/gear-tech/ss58.org/workflows/Build/badge.svg
[build_href]: https://github.com/gear-tech/ss58.org/actions/workflows/build.yml

[lic_badge]: https://img.shields.io/badge/License-MIT-success
[lic_href]: https://github.com/gear-tech/ss58.org/blob/master/LICENSE

🏹 https://ss58.org

## Build and Serve Locally

0. Install Rust using [rustup](https://rustup.rs/).

1. Install Rust nightly toolchain and Wasm target:

```
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown
```

2. Install [trunk](https://trunkrs.dev/#install) utility.

3. Clone the repo:

```sh
git clone https://github.com/gear-tech/ss58.org
cd ss58.org
```

4. Run `trunk` to build and serve the app:

```sh
trunk serve
```

3. Open http://localhost:8080

## License

Source code is licensed under the [MIT license](LICENSE).
