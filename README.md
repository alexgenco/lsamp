# `lsamp`

Sample stdin at a fixed rate.

For when you have a high volume source (e.g. live application logs) and you just want the gist.

## Installation

```sh
cargo install lsamp
```

## Usage

```sh
# Print 1 line every second
... | lsamp

# Print 10 lines every second
... | lsamp --rate 10

# Print 100 lines every 5.2 seconds
... | lsamp -r 100 --period 5.2s
```
