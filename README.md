```text
$ tw -h
textwrap 0.1.0

USAGE:
    tw [OPTIONS] <INPUT_FILES>...

ARGS:
    <INPUT_FILES>...    Input file(s); use `-` to read from standard input

OPTIONS:
    -w <WIDTH>        Width [default: 80]
    -e <EOL>          End of line string [default: \]
    -h, --help        Print help information
    -V, --version     Print version information
```

```text
$ echo abc def ghi jkl mno pqr stu vwx yz |tw -w 13
abc def ghi\
jkl mno pqr\
stu vwx yz
```

