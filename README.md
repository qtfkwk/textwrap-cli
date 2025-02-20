```text
$ tw -h
Command line interface for textwrap

Usage: tw [OPTIONS] <INPUT_FILES>...

Arguments:
  <INPUT_FILES>...  Input file(s); use `-` to read from standard input

Options:
  -w <WIDTH>      Width [default: 80]
  -e <EOL>        End of line string [default: \]
  -h, --help      Print help
  -V, --version   Print version
```

```text
$ tw -V
tw 0.2.3
```

```text
$ echo abc def ghi jkl mno pqr stu vwx yz |tw -w 13 -
abc def ghi\
jkl mno pqr\
stu vwx yz
```

