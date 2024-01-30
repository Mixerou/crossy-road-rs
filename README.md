<h1 align="center">
    <span>Crossy Road</span>
    <br>
    <span>🐔 But Using Rust 🐔</span>
</h1>

## Content

- [Development](#development)
    - [Dependencies compatibility](#dependency-compatibility)
    - [Environment Variables](#environment-variables)
- [License](#license)

## Development

> It's highly recommended to develop in debug mode with `debug` feature

### Dependency Compatibility

Since Bevy is in the early stages of development, there may be many breaking changes.
Therefore, if you want to upgrade to the next version, you should check dependency compatibility.

[bevy-inspector-egui]: https://github.com/jakobhellermann/bevy-inspector-egui?tab=readme-ov-file#bevy-support-table
[bevy_tweening]: https://github.com/djeedai/bevy_tweening?tab=readme-ov-file#compatible-bevy-versions

| Dependency            | Description                                        |
|-----------------------|----------------------------------------------------|
| [bevy-inspector-egui] | Inspector plugin for the Bevy game engine          |
| [bevy_tweening]       | Tweening animation plugin for the Bevy game engine |

### Environment Variables

| Variable   | Default Value | Description                                                                                                                   |
|------------|:-------------:|-------------------------------------------------------------------------------------------------------------------------------|
| `RUST_LOG` |       -       | `env_logger` output controller. Module declarations take comma separated entries formatted like `path::to::module=log_level`. |

## License

This project is available under the MIT license.
See the [LICENSE](LICENSE) file for more info.
