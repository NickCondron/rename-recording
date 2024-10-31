## Rename Recordings

Command line tool to automatically rename recording save files (.gci) from
[Training Mode](https://github.com/AlexanderHarrison/TrainingMode-More).

```
Usage: rename-recording [OPTIONS] <FILES>...

Arguments:
  <FILES>...
          File(s) to rename

Options:
  -f, --format <FORMAT_STR>
          File name format string

          Describes how to rename the recording file. Automatically appends the
          '.gci' extension. The string is interpreted literally except for the
          follwing codes:
          %n - Recording name (as seen in game import menu)
          %h - Human character name
          %c - CPU character name
          %d - Date as YYYY-MM-DD

          [default: %n]

  -i, --in-place
          Rename in place

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
