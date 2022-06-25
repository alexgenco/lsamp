# `lsamp`

Sample stdin at a fixed rate.

For when you have a high volume source (e.g. live application logs) but you just want the gist.

## Installation

```sh
cargo install lsamp
```

## Examples

```sh
# Print 1 line every second
... | lsamp

# Print 10 lines every second
... | lsamp --rate 10

# Print 3 lines every 800 milliseconds
... | lsamp -r 3 --period 800ms
```
