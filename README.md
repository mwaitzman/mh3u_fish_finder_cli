A rewrite in Rust of [AthenaADP's Expat-licensed MH3U Fish Finder](https://github.com/AthenaADP/MH3U-Fish-Finder/) as a Linux-first command line program.

Example usage:
```zsh
cargo rr <<<'
Sushifish x3, Sleepyfish x2, Popfish x5, ShiningStarfish x1, Whetfish x4
Sushifish x3, Sleepyfish x2, Sushifish x3, Sleepyfish x2
Sushifish x2, ShiningStarfish x1, Sushifish x2, ShiningStarfish x1
Sleepyfish x2, ShiningStarfish x1, HumspunConch x1, PinTuna x2'
```
The input parser is rather lenient: while row items are comma-delimited, and rows newline-delimited, whitespace, letter case, and trailing and consecutive commas are all ignored.
