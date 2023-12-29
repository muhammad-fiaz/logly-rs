<div align="center">

# logly.rs

[![Rust](https://github.com/muhammad-fiaz/logly-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/muhammad-fiaz/logly-rs/actions/workflows/rust.yml)
[![Crates.io Version](https://img.shields.io/crates/v/logly)](https://crates.io/crates/logly)
[![Crates.io Downloads](https://img.shields.io/crates/d/logly)](https://crates.io/crates/logly)
[![Crates.io License](https://img.shields.io/crates/l/logly)](https://opensource.org/licenses/MIT)
[![Crates.io Stability](https://img.shields.io/badge/Crates.io%20Stability-Stable-green)](https://crates.io/crates/logly)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![GitHub issues](https://img.shields.io/github/issues/muhammad-fiaz/logly-rs)](https://github.com/muhammad-fiaz/logly-rs/issues)
[![GitHub forks](https://img.shields.io/github/forks/muhammad-fiaz/logly-rs)](http://github.com/muhammad-fiaz/logly-rs/network)
[![GitHub stars](https://img.shields.io/github/stars/muhammad-fiaz/logly-rs)](http://github.com/muhammad-fiaz/logly-rs/stargazers)
[![GitHub license](https://img.shields.io/github/license/muhammad-fiaz/logly-rs)](https://github.com/muhammad-fiaz/logly-rs/blob/master/LICENSE)
[![GitHub contributors](https://img.shields.io/github/contributors/muhammad-fiaz/logly-rs)](https://github.com/muhammad-fiaz/logly-rs/graphs/contributors)
[![GitHub pull-requests](https://img.shields.io/github/issues-pr/muhammad-fiaz/logly-rs)](https://github.com/muhammad-fiaz/logly-rs/pulls)
[![Last Commit](https://img.shields.io/github/last-commit/muhammad-fiaz/logly-rs)](https://github.com/muhammad-fiaz/logly-rs)

[![Maintainer](https://img.shields.io/badge/Maintainer-muhammad--fiaz-blue)](https://github.com/muhammad-fiaz)
[![Sponsor on GitHub](https://img.shields.io/badge/Sponsor%20on%20GitHub-Become%20a%20Sponsor-blue)](https://github.com/sponsors/muhammad-fiaz)

</div>
Logly is a simple logging utility for rust that provides an easy way to log messages with different levels, colors, and options. It is designed to be flexible, allowing you to customize the log messages based on your application's needs. Logly supports logging to both the console and a file, and it comes with built-in color-coded log levels for better visibility.

if you like this project make sure to star 🌟 it in the [repository](https://github.com/muhammad-fiaz/logly-rs/) and if you want to contribute make sure to fork this repository❤✨.

## Installation
```shell
cargo add logly

```
### Usage

```rust
use logly::logly::*;
let logly = logly::new();

logly.start_logging("test_log.txt");

logly.info("Key1", "Value1", LogColor::Cyan);
logly.warn("Key2", "Value2", LogColor::Yellow);

logly.stop_logging();
logly.warn("Key3", "Value3", LogColor::Yellow);


```
## Contributing
Contributions are welcome! Before contributing, please read our [Contributing Guidelines](CONTRIBUTING.md) to ensure a smooth and collaborative development process.

## Code of Conduct

Please review our [Code of Conduct](CODE_OF_CONDUCT.md) to understand the standards of behavior we expect from contributors and users of this project.

## License
This project is licensed under the [MIT License](). See [LICENSE](LICENSE) for more details.

## Support the Project
<br>
<div align="center">

<h5> <strong> 💰 You can help me improve more by offering a little support on any platform❤️</strong></h5>

[![BuyMeACoffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-ffdd00?style=for-the-badge&logo=buy-me-a-coffee&logoColor=black)](https://buymeacoffee.com/muhammadfiaz) [![Patreon](https://img.shields.io/badge/Patreon-F96854?style=for-the-badge&logo=patreon&logoColor=white)](https://patreon.com/muhammadfiaz) [![Ko-Fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/muhammadfiaz)
[![Sponsor muhammad-fiaz](https://img.shields.io/badge/Sponsor-%231EAEDB.svg?&style=for-the-badge&logo=GitHub-Sponsors&logoColor=white)](https://github.com/sponsors/muhammad-fiaz)
[![Open Collective Backer](https://img.shields.io/badge/Open%20Collective-Backer-%238CC84B?style=for-the-badge&logo=open-collective&logoColor=white)](https://opencollective.com/muhammadfiaz)
</div>



## Happy Coding ❤️
