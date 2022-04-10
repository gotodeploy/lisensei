# lisensei

[![CI](https://github.com/gotodeploy/lisensei/workflows/CI/badge.svg)](https://github.com/gotodeploy/lisensei/actions)
[![Coverage Status](https://coveralls.io/repos/github/gotodeploy/lisensei/badge.svg?branch=main)](https://coveralls.io/github/gotodeploy/lisensei?branch=main)

## About

注音(Bopomofo) typing training tool for traditional Chinese language learners. You can play [this tool](https://gotodeploy.github.io/lisensei/) in major web browsers supporting [WebAssembly](https://webassembly.org/).

## Build instructions

```sh
# e.g. Ubuntu
sudo apt-get update && sudo apt-get install -y pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev sqlite3
git clone https://github.com/gotodeploy/lisensei.git
cd lisensei
rustup target add x86_64-unknown-linux-gnu
cargo build --target x86_64-unknown-linux-gnu --release
```

## License

 Apache License, Version 2.0
   ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)
 

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
