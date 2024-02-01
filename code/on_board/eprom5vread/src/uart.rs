use stm32g0b0::Peripherals;


pub(crate) fn setup(peripherals: &mut Peripherals) {
    // enable clock for relevant GPIO (PORTA)
    peripherals.RCC.iopenr().modify(|_, w| w
        .gpioaen().set_bit()
    );

    // configure PA9 and PA10 as Alternative Function 1
    peripherals.GPIOA.moder().modify(|_, w| w
        .moder9().alternate_function()
        .moder10().alternate_function()
    );
    peripherals.GPIOA.afrh().modify(|_, w| w
        .afsel9().af1()
        .afsel10().af1()
    );

    // enable clock for USART1
    peripherals.RCC.apbenr2().modify(|_, w| w
        .usart1en().set_bit()
    );

    // disable, configure, then enable USART1
    peripherals.USART1.cr1_fifo_disabled().modify(|_, w| w
        .ue().clear_bit()
    );
    peripherals.USART1.cr1_fifo_disabled().modify(|_, w| w
        .uesm().clear_bit() // do not enable in low-power mode
        .re().set_bit() // enable receiver
        .te().set_bit() // enable transmitter
        .idleie().clear_bit() // disable idle interrupt
        .rxneie().clear_bit() // disable "receive data register not empty" interrupt
        .tcie().clear_bit() // disable "transmission complete" interrupt
        .txeie().clear_bit() // disable "transmit data register empty" interrupt
        .peie().clear_bit() // disable "parity error" interrupt
        .pce().clear_bit() // disable parity calculation (tx) and verification (rx)
        .m0().clear_bit() // 8 data bits
        .m1().clear_bit() // 8 data bits (part 2)
        .mme().clear_bit() // disable mute mode
        .cmie().clear_bit() // disable "character match" interrupt
        .over8().over16() // oversample by 16 bits
        .fifoen().clear_bit() // disable FIFO mode
    );
}
