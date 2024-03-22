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
