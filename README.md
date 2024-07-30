# bidet

Small CLI utility for converting .VTT and .SRT subtitle files to .ASS subtitle files.

```
bidet 0.1.0
Small .ass subtitle conversion utility.

USAGE:
    bidet [FLAGS] <input> [output]

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      Disable output on successful conversion
    -V, --version    Prints version information

ARGS:
    <input>     Input subtitle file (can be either .srt or .vtt)
    <output>    Output .ass subtitle file
```

Most of the heavy lifting is done by [rsubs-lib](https://github.com/adracea/rsubs-lib).