# what-the-dog-doin

Do you ever like to get interrupted by random sounds? Do you ever watch 10 hours of silence occasionally broken by what the dog doin? Then this is the PERFECT cli tool for you!

## Usage

For supported audio files, see [rodio](https://github.com/RustAudio/rodio)

```
> what-the-dog-doin --help
Usage: what-the-dog-doin [OPTIONS] <FILE>

Arguments:
  <FILE>  Audio file to be played

Options:
  -f, --from <FROM>     Minimum amount of delay [default: 0]
  -u, --until <UNTIL>   Maximum amount of delay [default: 1200]
  -r, --disable_random  Disables random delay
  -d, --delay <DELAY>   Delay that should be used if random is off. This is added to the length of the sound
  -h, --help            Print help
  -V, --version         Print version
```

## Installation

### Cargo

```bash
cargo install what-the-dog-doin
```

## License

This project is licensed under either of the following licenses, at your option:

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or [here](https://www.apache.org/licenses/LICENSE-2.0))
- MIT License ([LICENSE-MIT](./LICENSE-MIT) or [here](https://opensource.org/licenses/MIT))
