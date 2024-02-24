<div align="center">
  
![logo banner](https://github.com/muhammad-fiaz/logly-rs/assets/75434191/500975cc-b5f4-46df-abe8-2d03c687a1c1)

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

let logly = logly::new(); //intialize the logly

logly.start_logging("test_log.txt"); //start storing the log messages in txt, make sure to pass to create the log file

logly.info("Key1", "Value1", LogColor::Cyan); // message with custom color if you don't want just set it None
logly.warn("Key2", "Value2", LogColor::Yellow);

logly.stop_logging(); //this will stop storing the message from here but it will display
logly.warn("Key3", "Value3", LogColor::Yellow);


```
## Color Options:

### Default Color Options:

| Level    | Color Code      |
| -------- | --------------- |
| INFO     | CYAN            |
| WARNING  | YELLOW          |
| ERROR    | RED             |
| DEBUG    | BLUE            |
| CRITICAL | BRIGHT RED      |
| TRACE    | BLUE            |
| DEFAULT  | WHITE           |

### Custom Color Options:

You can use any of the following color codes for custom coloring:

| NAME     | Color Code      |
|----------| --------------- |
| CYAN      | CYAN            |
| YELLOW   | YELLOW          |
|  RED       | RED             |
|  BLUE      | BLUE            |
| BRIGHT RED | CRITICAL     |
|WHITE   | WHITE           |


## Contributing
Contributions are welcome! Before contributing, please read our [Contributing Guidelines](CONTRIBUTING.md) to ensure a smooth and collaborative development process.

## Issues

If you encounter any issues or have suggestions, feel free to [open an issue](https://github.com/muhammad-fiaz/logly-rs/issues) on GitHub!

## Code of Conduct

Please review our [Code of Conduct](CODE_OF_CONDUCT.md) to understand the standards of behavior we expect from contributors and users of this project.

## License
This project is licensed under the [MIT License](). See [LICENSE](LICENSE) for more details.

## Support the Project
<br>
<div align="center">

<h5> <strong> 💰 You can help this improve more by offering a little support! ❤️</strong></h5>

[![Sponsor muhammad-fiaz](https://img.shields.io/badge/Sponsor-%231EAEDB.svg?&style=for-the-badge&logo=GitHub-Sponsors&logoColor=white)](https://github.com/sponsors/muhammad-fiaz)

</div>


## Happy Coding ❤️
