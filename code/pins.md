# eprom5vread pins (STM32G0B0KET)

| logical | physical | usage | alt func |
| ---- | --:| ------------- | ---- |
| PA9  | 19 | USART1 Tx     | AF1  |
| PA10 | 21 | USART1 Rx     | AF1  |
| PA13 | 24 | SWDIO         | AF0  |
| PA14 | 25 | SWCLK         | AF0  |
| PB1  | 16 | I2C_~{RESET}  | GPIO |
| PB6  | 30 | I2C1 SCL      | AF6  |
| PB7  | 31 | I2C1 SDA      | AF6  |

on bootup

* PA13+PA14 are set to AF0 (debug)
* all other pins are set to analog mode (GPIO)

## I/O expander (MCP23008) pins

The EPROM is assumed to be a model similar to the ST M27C800 or M27C322.

| I2C address (binary) | GPIO pin | DIP pin | EPROM pin |
| ---:| -:| --:| ---- |
| 000 | 0 |  8 | A2   |
| 000 | 1 |  7 | A3   |
| 000 | 2 |  6 | A4   |
| 000 | 3 |  5 | A5   |
| 000 | 4 |  4 | A6   |
| 000 | 5 |  3 | A7   |
| 000 | 6 |  2 | A17  |
| 000 | 7 |  1 | A18  |
| 001 | 0 | 17 | Q9   |
| 001 | 1 | 16 | Q1   |
| 001 | 2 | 15 | Q8   |
| 001 | 3 | 14 | Q0   |
| 001 | 4 | 13 | ~{G} |
| 001 | 5 | 11 | ~{E} |
| 001 | 6 | 10 | A0   |
| 001 | 7 |  9 | A1   |
| 010 | 0 | 26 | Q13  |
| 010 | 1 | 25 | Q5   |
| 010 | 2 | 24 | Q12  |
| 010 | 3 | 23 | Q4   |
| 010 | 4 | 21 | Q11  |
| 010 | 5 | 20 | Q3   |
| 010 | 6 | 19 | Q10  |
| 010 | 7 | 18 | Q2   |
| 011 | 0 | 35 | A14  |
| 011 | 1 | 34 | A15  |
| 011 | 2 | 33 | A16  |
| 011 | 3 | 32 | A20/~{BYTE}V_{PP} |
| 011 | 4 | 30 | Q15/A-1 |
| 011 | 5 | 29 | Q7   |
| 011 | 6 | 28 | Q14  |
| 011 | 7 | 27 | Q6   |
| 100 | 0 | -  | -    |
| 100 | 1 | 42 | A19  |
| 100 | 2 | 41 | A8   |
| 100 | 3 | 40 | A9   |
| 100 | 4 | 39 | A10  |
| 100 | 5 | 38 | A11  |
| 100 | 6 | 37 | A12  |
| 100 | 7 | 36 | A13  |
| -   | - | 12 | GND  |
| -   | - | 22 | +5V  |
| -   | - | 31 | GND  |
