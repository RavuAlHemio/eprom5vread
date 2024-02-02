# eprom5vread pins (STM32G0B0CET)

assuming ROM is an M27C800-100F1

| logical | physical | usage | alt func |
| ---- | --:| ----------------- | ---- |
| PA0  | 11 | LS_OE             | GPIO |
| PA1  | 12 | ROM_A8            | GPIO |
| PA2  | 13 | ROM_A9            | GPIO |
| PA3  | 14 | ROM_A10           | GPIO |
| PA4  | 15 | ROM_A11           | GPIO |
| PA5  | 16 | ROM_A12           | GPIO |
| PA6  | 17 | ROM_A13           | GPIO |
| PA7  | 18 | ROM_A14           | GPIO |
| PA8  | 28 | ROM_Q5            | GPIO |
| PA9  | 29 | USART1 Tx         | AF1  |
| PA10 | 32 | USART1 Rx         | AF1  |
| PA11 | 33 | ROM_Q10           | GPIO |
| PA12 | 34 | ROM_Q3            | GPIO |
| PA13 | 35 | SWDIO             | AF0  |
| PA14 | 36 | SWCLK             | AF0  |
| PA15 | 37 | ROM_Q11           | GPIO |
| PB0  | 19 | ROM_A15           | GPIO |
| PB1  | 20 | ROM_A16           | GPIO |
| PB2  | 21 | ROM_~{BYTE}V_{PP} | GPIO |
| PB3  | 42 | ROM_~{OE}         | GPIO |
| PB4  | 43 | ROM_~{CS}         | GPIO |
| PB5  | 44 | ROM_A0            | GPIO |
| PB6  | 45 | ROM_A1            | GPIO |
| PB7  | 46 | ROM_A2            | GPIO |
| PB8  | 47 | ROM_A3            | GPIO |
| PB9  | 48 | ROM_A4            | GPIO |
| PB10 | 22 | ROM_Q15A-1        | GPIO |
| PB11 | 23 | ROM_Q7            | GPIO |
| PB12 | 24 | ROM_Q14           | GPIO |
| PB13 | 25 | ROM_Q6            | GPIO |
| PB14 | 26 | ROM_Q4            | GPIO |
| PB15 | 27 | ROM_Q12           | GPIO |
| PC6  | 30 | ROM_Q13           | GPIO |
| PC7  | 31 | ROM_Q2            | GPIO |
| PC13 |  1 | ROM_A5            | GPIO |
| PC14 |  2 | ROM_A6            | GPIO |
| PC15 |  3 | ROM_A7            | GPIO |
| PD0  | 38 | ROM_Q9            | GPIO |
| PD1  | 39 | ROM_Q1            | GPIO |
| PD2  | 40 | ROM_Q8            | GPIO |
| PD3  | 41 | ROM_Q0            | GPIO |
| PF0  |  8 | ROM_A17           | GPIO |
| PF1  |  9 | ROM_A18           | GPIO |

on bootup

* PA13+PA14 are set to AF0 (debug)
* all other pins are set to analog mode (GPIO)
