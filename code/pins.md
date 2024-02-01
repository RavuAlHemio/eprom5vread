# eprom5vread pins (STM32G0B0CET)

| logical | physical | usage | alt func |
| ---- | --:| ------------- | ---- |
| PA0  | 11 | LSOE          | GPIO |
| PA1  | 12 | A8            | GPIO |
| PA2  | 13 | A9            | GPIO |
| PA3  | 14 | A10           | GPIO |
| PA4  | 15 | A11           | GPIO |
| PA5  | 16 | A12           | GPIO |
| PA6  | 17 | A13           | GPIO |
| PA7  | 18 | A14           | GPIO |
| PA8  | 28 | Q5            | GPIO |
| PA9  | 29 | USART1 Tx     | AF1  |
| PA10 | 32 | USART1 Rx     | AF1  |
| PA11 | 33 | Q10           | GPIO |
| PA12 | 34 | Q3            | GPIO |
| PA13 | 35 | SWDIO         | AF0  |
| PA14 | 36 | SWCLK         | AF0  |
| PA15 | 37 | Q11           | GPIO |
| PB0  | 19 | A15           | GPIO |
| PB1  | 20 | A16           | GPIO |
| PB2  | 21 | ~{BYTE}V_{PP} | GPIO |
| PB3  | 42 | ROM_~{OE}     | GPIO |
| PB4  | 43 | ROM_~{CS}     | GPIO |
| PB5  | 44 | A0            | GPIO |
| PB6  | 45 | A1            | GPIO |
| PB7  | 46 | A2            | GPIO |
| PB8  | 47 | A3            | GPIO |
| PB9  | 48 | A4            | GPIO |
| PB10 | 22 | Q15A-1        | GPIO |
| PB11 | 23 | Q7            | GPIO |
| PB12 | 24 | Q14           | GPIO |
| PB13 | 25 | Q6            | GPIO |
| PB14 | 26 | Q4            | GPIO |
| PB15 | 27 | Q12           | GPIO |
| PC6  | 30 | Q13           | GPIO |
| PC7  | 31 | Q2            | GPIO |
| PC13 |  1 | A5            | GPIO |
| PC14 |  2 | A6            | GPIO |
| PC15 |  3 | A7            | GPIO |
| PD0  | 38 | Q9            | GPIO |
| PD1  | 39 | Q1            | GPIO |
| PD2  | 40 | Q8            | GPIO |
| PD3  | 41 | Q0            | GPIO |
| PF0  |  8 | A17           | GPIO |
| PF1  |  9 | A18           | GPIO |

on bootup

* PA13+PA14 are set to AF0 (debug)
* all other pins are set to analog mode (GPIO)
