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
    peripherals.USART1.cr2().modify(|_, w| w
        .slven().clear_bit() // disable synchronous slave mode
        .lbdie().clear_bit() // disable "LIN break detection" interrupt
        .clken().clear_bit() // disable SCLK pin
        .stop().one() // one stop bit
        .linen().clear_bit() // disable LIN mode
        .swap().clear_bit() // do not swap Tx and Rx
        .rxinv().clear_bit() // do not invert logic levels on Rx pin
        .txinv().clear_bit() // do not invert logic levels on Tx pin
        .datainv().clear_bit() // do not invert logic levels on data
        .msbfirst().clear_bit() // send data LSB-first (RS232)
        .abren().clear_bit() // disable auto baud rate detection
        .rtoen().clear_bit() // disable receiver timeout
    );
    peripherals.USART1.cr3().modify(|_, w| w
        .eie().clear_bit() // disable error interrupt
        .iren().clear_bit() // disable IrDA mode
        .hdsel().clear_bit() // disable half-duplex mode
        .scen().clear_bit() // disable smart card mode
        .dmar().clear_bit() // disable DMA mode on reception
        .dmat().clear_bit() // disable DMA mode on transmission
        .rtse().clear_bit() // disable Ready-to-Send hardware flow control
        .ctse().clear_bit() // disable Clear-to-Send hardware flow control
        .ctsie().clear_bit() // disable Clear-to-Send interrupt
        .onebit().clear_bit() // sample three bits, not just one
        .ovrdis().clear_bit() // enable overrun detection
        .dem().clear_bit() // disable driver-enable mode
        .wufie().clear_bit() // disable "wake up from low-power mode" interrupt
        .txftie().clear_bit() // disable "TXFIFO threshold reached" interrupt
        .tcbgtie().clear_bit() // disable "transmission complete before guard time" interrupt
        .rxftie().clear_bit() // disable "RXFIFO threshold reached" interrupt
    );
    peripherals.USART1.presc().modify(|_, w| w
        .prescaler().div1() // do not divide baud clock (= divide baud clock by 1)
    );
    peripherals.USART1.brr().modify(|_, w| w
        .brr().variant(1667) // 9600 baud at 16 MHz
    );
    peripherals.USART1.cr1_fifo_disabled().modify(|_, w| w
        .ue().set_bit() // enable USART
    );
}

pub(crate) fn write<'a, I: IntoIterator<Item = &'a u8>>(peripherals: &mut Peripherals, data: I) {
    for &b in data.into_iter() {
        // wait until Tx buffer is empty
        while peripherals.USART1.isr_fifo_disabled().read().txe().bit_is_clear() {
        }

        peripherals.USART1.tdr().modify(|_, w| w
            .tdr().variant(b.into())
        );
    }

    // wait until Tx buffer is empty
    while peripherals.USART1.isr_fifo_disabled().read().txe().bit_is_clear() {
    }

    // wait until transmission has completed
    while peripherals.USART1.isr_fifo_disabled().read().tc().bit_is_clear() {
    }
}
