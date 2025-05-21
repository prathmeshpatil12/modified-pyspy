# py-spy (Modified Version)

This is a modified version of [py-spy](https://github.com/benfred/py-spy), a sampling profiler for Python programs. The modifications add support for **timestamped call stacks**.

## Modifications

### 1. Timestamped Call Stack Capture

In addition to the default flamegraph output, this version records call stacks along with their respective timestamps in a new JSON file format:

```json
{
  "stack": [
    "main (Fibonnaci.py:14)",
    "<module> (Fibonnaci.py:18)"
  ],
  "timestamp": "2025-04-09T02:48:11.009374Z"
}
```
The file is saved as {output_file}_timestamps.json.

## Installation

To build and install from source:

```bash
cargo install py-spy
```

Dependencies:
On Linux and Windows, make sure to install libunwind before building:

```bash
sudo apt install libunwind-dev
```

## Output
- Flamegraph output (default behavior)
- Timestamped call stacks in {output_file}_timestamps.json.
Each entry in the JSON file looks like:
```json
{
  "stack": [
    "main (Fibonnaci.py:14)",
    "<module> (Fibonnaci.py:18)"
  ],
  "timestamp": "2025-04-09T02:48:11.009374Z"
}
```

## Technologies Used
- Rust – main language for the profiler
- Python – target programs being profiled
- libunwind – for native stack tracing on Linux and Windows
