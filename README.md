**This feature wsa bundled to provlegisto as application trayer, the program was no longer maintain**

**See [Provlegisto](https://github.com/mslxl/provlegisto) for more information**


# ContestWatchman

ContestWatchman, the abbr of Recent Contest Night's Watchman

### Build

```
$ cargo build --release

# Bulid docker image
$ nu make.nu
```


### Usage

```
Usage: ContestWatchman [PORT] <COMMAND>

Commands:
  insert  Insert item to database manually
  pull    Update database from remote
  send    Send emails to subscriber
  serve   Start server
  help    Print this message or the help of the given subcommand(s)

Arguments:
  [PORT]  [default: 3000]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

There are 2 path to gain resource currently:
- `/data`: Recent contest in json format
- `/atom.xml`: Recent contest feed
