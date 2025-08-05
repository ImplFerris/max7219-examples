# ESP32 LED Matrix Examples

This directory contains a set of example projects for the MAX7219 LED matrix display. Projects in this directory are built to run on the **ESP32 DevKit v1** board.

Each subfolder is a standalone example demonstrating different LED matrix operations such as displaying characters, patterns, or scrolling text.

## Usage

Each example is a separate Cargo package. You can build and flash any example directly to your ESP32 DevKit board.

### Prerequisites

- Install [esp-hal development tools](https://docs.espressif.com/projects/rust/book/installation/index.html/)
- Set up your environment with the appropriate toolchain
- Connect your ESP32 DevKit v1 via USB

## Hardware Setup

### Required Components

- ESP32 DevKit v1
- 8x8 LED matrix module(s) with MAX7219 driver
- Connecting wires

### Wiring Connections

| ESP32 Pin | MAX7219 Pin | Description    |
|-----------|-------------|----------------|
| GPIO18    | CLK         | Clock signal   |
| GPIO23    | DIN         | Data input     |
| GPIO21    | CS          | Chip select    |

> **Note**: These pin assignments can be modified in each example's source code.

## Running an Example

To build and flash an individual example to your ESP32 board, run:

```sh
# Ensure you are inside max7219-examples/led-matrix/esp32 folder when you run this command

cargo run --package print-char
```

Replace print-char with any other package name such as scroll-text, graphics, etc.
