# TimeIt

A simple command-line utility written in Rust that measures how long a command takes to execute.

## Features

* Run any command from the command line.
* Measure total execution time.
* Automatically displays durations in:

  * Seconds (`s`)
  * Minutes (`m`)
  * Hours (`h`)
* Lightweight and dependency-free.

## Installation

### Build from Source

```bash
git clone <repository-url>
cd timeit
cargo build --release
```

The compiled executable will be available at:

```text
target/release/timeit.exe
```

## Usage

```bash
timeit <COMMAND> [ARGS...]
```

### Examples

Time a directory listing:

```bash
timeit dir
```

Time a ping command:

```bash
timeit ping localhost -n 5
```

Time another executable:

```bash
timeit myprogram.exe --verbose
```

### Example Output

For short-running commands:

```text
Execution time of `dir`: 125.7ms
```

For longer-running commands:

```text
Execution time of `ping`: 5.0123s
```

Or:

```text
Execution time of `backup`: 2.4831m
```

## How It Works

`timeit` records the current time before launching the target process and measures the elapsed duration after the process exits.

Internally it:

1. Collects command-line arguments.
2. Launches the command using the Windows command interpreter (`cmd /C`).
3. Waits for the process to complete.
4. Calculates and displays the elapsed execution time.

## Platform Support

This implementation currently targets **Windows** because it executes commands through:

```text
cmd /C
```

To support Linux or macOS, the process-launching logic would need to be adapted to use the appropriate shell (such as `sh -c`).

## Help

Running without arguments displays:

```text
Time a command line process

Usage: timeit <COMMANDS>
```

## License

MIT License
