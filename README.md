# VCVRack-rs

A VCV Rack clone written in Rust.

## Description

In the current state it's not usable for users. Currently it's a way for me to test whether I can get it to resemble VCV Rack written in Rust simply by prompting Cursor.
Hopefully this will help me learn Rust and Sound Synthesis along the way.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- cargo-make (install with `cargo install cargo-make`)
- [List any other dependencies]

### Installation

1. Install cargo-make if you haven't already:

```bash
cargo install cargo-make
```

2. Run the application:

```bash
cargo make run
```

### Development Commands

- `cargo make coverage` - Generate and view code coverage report
- `cargo make test-watch` - Run tests in watch mode
- `cargo make fmt` - Format code
- `cargo make lint` - Run linter
