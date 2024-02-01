use stm32g0b0::Peripherals;


pub(crate) fn setup_clocks(_peripherals: &mut Peripherals) {
    // default:
    // HSI16 16MHz --[/HSIDIV=1]--> HSISYS --> SYSCLK
    // SYSCLK --[/HPRE=1]--> HCLK --[/PPRE=1]--> PCLK
    // PCLK --> USART1 --[/USART_PRESC=1]--> baud rate generator
    // (different source for USART1 can be chosen in USART1SEL)
    //
    // 16 MHz / 9600 baud
    // = 16 MHz / 9 600 Hz
    // = 16 000 000 Hz / 9 600 Hz
    // ~ 1667 (0x0683) => fits into 16 bits
    //
    // => no prescaling or changes necessary
}
