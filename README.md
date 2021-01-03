# tiwi
Tiwi is an interactive application which keeps track of timestamped events.  Tiwi is short for 'timey wimey'.

Tiwi is an interactive tool that writes to stdout, and optionally a log file.

# Installation
Install with:
```
cargo install tiwi
```

Run with:
```
tiwi
```
## Quickstart
Tiwi begins tracking events when it receives any input from stdin:
```
$ tiwi
2020-10-15T19:57:28-04:00	--	launched the server█
```

The entry is completed with Enter, and the cursor is hidden:
```
$ tiwi
2020-10-15T19:57:28-04:00	--	launched the server
```

With more input, tiwi will begin tracking a new event and print the previous event span:
```
$ tiwi
2020-10-15T19:57:28-04:00	2m	launched the server
2020-10-15T19:59:57-04:00	--	opened the ui█
```

## Options
Tiwi supports appending to a logfile, if given an argument:
```
tiwi output.txt
```

Tiwi also supports logging in UTC:
```
tiwi --utc
```