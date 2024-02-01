#[doc = "Register `ISR_FIFO_ENABLED` reader"]
pub type R = crate::R<ISR_FIFO_ENABLED_SPEC>;
#[doc = "Field `PE` reader - Parity error This bit is set by hardware when a parity error occurs in receiver mode. It is cleared by software, writing 1 to the PECF in the USART_ICR register. An interrupt is generated if PEIE = 1 in the USART_CR1 register. Note: This error is associated with the character in the USART_RDR."]
pub type PE_R = crate::BitReader<PE_A>;
#[doc = "Parity error This bit is set by hardware when a parity error occurs in receiver mode. It is cleared by software, writing 1 to the PECF in the USART_ICR register. An interrupt is generated if PEIE = 1 in the USART_CR1 register. Note: This error is associated with the character in the USART_RDR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE_A {
    #[doc = "0: No parity error"]
    B_0X0 = 0,
    #[doc = "1: Parity error"]
    B_0X1 = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
impl PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::B_0X0,
            true => PE_A::B_0X1,
        }
    }
    #[doc = "No parity error"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PE_A::B_0X0
    }
    #[doc = "Parity error"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PE_A::B_0X1
    }
}
#[doc = "Field `FE` reader - Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE=1 in the USART_CR1 register. Note: This error is associated with the character in the USART_RDR."]
pub type FE_R = crate::BitReader<FE_A>;
#[doc = "Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE=1 in the USART_CR1 register. Note: This error is associated with the character in the USART_RDR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_A {
    #[doc = "0: No Framing error is detected"]
    B_0X0 = 0,
    #[doc = "1: Framing error or break character is detected"]
    B_0X1 = 1,
}
impl From<FE_A> for bool {
    #[inline(always)]
    fn from(variant: FE_A) -> Self {
        variant as u8 != 0
    }
}
impl FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FE_A {
        match self.bits {
            false => FE_A::B_0X0,
            true => FE_A::B_0X1,
        }
    }
    #[doc = "No Framing error is detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FE_A::B_0X0
    }
    #[doc = "Framing error or break character is detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FE_A::B_0X1
    }
}
#[doc = "Field `NE` reader - Noise detection flag This bit is set by hardware when noise is detected on a received frame. It is cleared by software, writing 1 to the NECF bit in the USART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXFNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. When the line is noise-free, the NE flag can be disabled by programming the ONEBIT bit to 1 to increase the USART tolerance to deviations (Refer to Tolerance of the USART receiver to clock deviation on page861). This error is associated with the character in the USART_RDR."]
pub type NE_R = crate::BitReader<NE_A>;
#[doc = "Noise detection flag This bit is set by hardware when noise is detected on a received frame. It is cleared by software, writing 1 to the NECF bit in the USART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXFNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. When the line is noise-free, the NE flag can be disabled by programming the ONEBIT bit to 1 to increase the USART tolerance to deviations (Refer to Tolerance of the USART receiver to clock deviation on page861). This error is associated with the character in the USART_RDR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NE_A {
    #[doc = "0: No noise is detected"]
    B_0X0 = 0,
    #[doc = "1: Noise is detected"]
    B_0X1 = 1,
}
impl From<NE_A> for bool {
    #[inline(always)]
    fn from(variant: NE_A) -> Self {
        variant as u8 != 0
    }
}
impl NE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NE_A {
        match self.bits {
            false => NE_A::B_0X0,
            true => NE_A::B_0X1,
        }
    }
    #[doc = "No noise is detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == NE_A::B_0X0
    }
    #[doc = "Noise is detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NE_A::B_0X1
    }
}
#[doc = "Field `ORE` reader - Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the USART_RDR register while RXFF = 1. It is cleared by a software, writing 1 to the ORECF, in the USART_ICR register. An interrupt is generated if RXFNEIE=1 or EIE = 1 in the USART_CR1 register. Note: When this bit is set, the USART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the USART_CR3 register."]
pub type ORE_R = crate::BitReader<ORE_A>;
#[doc = "Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the USART_RDR register while RXFF = 1. It is cleared by a software, writing 1 to the ORECF, in the USART_ICR register. An interrupt is generated if RXFNEIE=1 or EIE = 1 in the USART_CR1 register. Note: When this bit is set, the USART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the USART_CR3 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORE_A {
    #[doc = "0: No overrun error"]
    B_0X0 = 0,
    #[doc = "1: Overrun error is detected"]
    B_0X1 = 1,
}
impl From<ORE_A> for bool {
    #[inline(always)]
    fn from(variant: ORE_A) -> Self {
        variant as u8 != 0
    }
}
impl ORE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ORE_A {
        match self.bits {
            false => ORE_A::B_0X0,
            true => ORE_A::B_0X1,
        }
    }
    #[doc = "No overrun error"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ORE_A::B_0X0
    }
    #[doc = "Overrun error is detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ORE_A::B_0X1
    }
}
#[doc = "Field `IDLE` reader - Idle line detected This bit is set by hardware when an Idle Line is detected. An interrupt is generated if IDLEIE=1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the USART_ICR register. Note: The IDLE bit is not set again until the RXFNE bit has been set (i.e. a new idle line occurs). If Mute mode is enabled (MME=1), IDLE is set if the USART is not mute (RWU=0), whatever the Mute mode selected by the WAKE bit. If RWU=1, IDLE is not set."]
pub type IDLE_R = crate::BitReader<IDLE_A>;
#[doc = "Idle line detected This bit is set by hardware when an Idle Line is detected. An interrupt is generated if IDLEIE=1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the USART_ICR register. Note: The IDLE bit is not set again until the RXFNE bit has been set (i.e. a new idle line occurs). If Mute mode is enabled (MME=1), IDLE is set if the USART is not mute (RWU=0), whatever the Mute mode selected by the WAKE bit. If RWU=1, IDLE is not set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLE_A {
    #[doc = "0: No Idle line is detected"]
    B_0X0 = 0,
    #[doc = "1: Idle line is detected"]
    B_0X1 = 1,
}
impl From<IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDLE_A {
        match self.bits {
            false => IDLE_A::B_0X0,
            true => IDLE_A::B_0X1,
        }
    }
    #[doc = "No Idle line is detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IDLE_A::B_0X0
    }
    #[doc = "Idle line is detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IDLE_A::B_0X1
    }
}
#[doc = "Field `RXFNE` reader - RXFIFO not empty RXFNE bit is set by hardware when the RXFIFO is not empty, meaning that data can be read from the USART_RDR register. Every read operation from the USART_RDR frees a location in the RXFIFO. RXFNE is cleared when the RXFIFO is empty. The RXFNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXFNEIE=1 in the USART_CR1 register."]
pub type RXFNE_R = crate::BitReader<RXFNE_A>;
#[doc = "RXFIFO not empty RXFNE bit is set by hardware when the RXFIFO is not empty, meaning that data can be read from the USART_RDR register. Every read operation from the USART_RDR frees a location in the RXFIFO. RXFNE is cleared when the RXFIFO is empty. The RXFNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXFNEIE=1 in the USART_CR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFNE_A {
    #[doc = "0: Data is not received"]
    B_0X0 = 0,
    #[doc = "1: Received data is ready to be read."]
    B_0X1 = 1,
}
impl From<RXFNE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFNE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXFNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXFNE_A {
        match self.bits {
            false => RXFNE_A::B_0X0,
            true => RXFNE_A::B_0X1,
        }
    }
    #[doc = "Data is not received"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXFNE_A::B_0X0
    }
    #[doc = "Received data is ready to be read."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXFNE_A::B_0X1
    }
}
#[doc = "Field `TC` reader - Transmission complete This bit indicates that the last data written in the USART_TDR has been transmitted out of the shift register. It is set by hardware when the transmission of a frame containing data is complete and when TXFE is set. An interrupt is generated if TCIE=1 in the USART_CR1 register. TC bit is is cleared by software, by writing 1 to the TCCF in the USART_ICR register or by a write to the USART_TDR register. Note: If TE bit is reset and no transmission is on going, the TC bit is immediately set."]
pub type TC_R = crate::BitReader<TC_A>;
#[doc = "Transmission complete This bit indicates that the last data written in the USART_TDR has been transmitted out of the shift register. It is set by hardware when the transmission of a frame containing data is complete and when TXFE is set. An interrupt is generated if TCIE=1 in the USART_CR1 register. TC bit is is cleared by software, by writing 1 to the TCCF in the USART_ICR register or by a write to the USART_TDR register. Note: If TE bit is reset and no transmission is on going, the TC bit is immediately set.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC_A {
    #[doc = "0: Transmission is not complete"]
    B_0X0 = 0,
    #[doc = "1: Transmission is complete"]
    B_0X1 = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
impl TC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::B_0X0,
            true => TC_A::B_0X1,
        }
    }
    #[doc = "Transmission is not complete"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TC_A::B_0X0
    }
    #[doc = "Transmission is complete"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TC_A::B_0X1
    }
}
#[doc = "Field `TXFNF` reader - TXFIFO not full TXFNF is set by hardware when TXFIFO is not full meaning that data can be written in the USART_TDR. Every write operation to the USART_TDR places the data in the TXFIFO. This flag remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared indicating that data can not be written into the USART_TDR. An interrupt is generated if the TXFNFIE bit =1 in the USART_CR1 register. Note: The TXFNF is kept reset during the flush request until TXFIFO is empty. After sending the flush request (by setting TXFRQ bit), the flag TXFNF should be checked prior to writing in TXFIFO (TXFNF and TXFE are set at the same time). This bit is used during single buffer transmission."]
pub type TXFNF_R = crate::BitReader<TXFNF_A>;
#[doc = "TXFIFO not full TXFNF is set by hardware when TXFIFO is not full meaning that data can be written in the USART_TDR. Every write operation to the USART_TDR places the data in the TXFIFO. This flag remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared indicating that data can not be written into the USART_TDR. An interrupt is generated if the TXFNFIE bit =1 in the USART_CR1 register. Note: The TXFNF is kept reset during the flush request until TXFIFO is empty. After sending the flush request (by setting TXFRQ bit), the flag TXFNF should be checked prior to writing in TXFIFO (TXFNF and TXFE are set at the same time). This bit is used during single buffer transmission.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFNF_A {
    #[doc = "0: Transmit FIFO is full"]
    B_0X0 = 0,
    #[doc = "1: Transmit FIFO is not full"]
    B_0X1 = 1,
}
impl From<TXFNF_A> for bool {
    #[inline(always)]
    fn from(variant: TXFNF_A) -> Self {
        variant as u8 != 0
    }
}
impl TXFNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXFNF_A {
        match self.bits {
            false => TXFNF_A::B_0X0,
            true => TXFNF_A::B_0X1,
        }
    }
    #[doc = "Transmit FIFO is full"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXFNF_A::B_0X0
    }
    #[doc = "Transmit FIFO is not full"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXFNF_A::B_0X1
    }
}
#[doc = "Field `LBDF` reader - LIN break detection flag This bit is set by hardware when the LIN break is detected. It is cleared by software, by writing 1 to the LBDCF in the USART_ICR. An interrupt is generated if LBDIE = 1 in the USART_CR2 register. Note: If the USART does not support LIN mode, this bit is reserved and kept at reset value. Refer to ."]
pub type LBDF_R = crate::BitReader<LBDF_A>;
#[doc = "LIN break detection flag This bit is set by hardware when the LIN break is detected. It is cleared by software, by writing 1 to the LBDCF in the USART_ICR. An interrupt is generated if LBDIE = 1 in the USART_CR2 register. Note: If the USART does not support LIN mode, this bit is reserved and kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDF_A {
    #[doc = "0: LIN Break not detected"]
    B_0X0 = 0,
    #[doc = "1: LIN break detected"]
    B_0X1 = 1,
}
impl From<LBDF_A> for bool {
    #[inline(always)]
    fn from(variant: LBDF_A) -> Self {
        variant as u8 != 0
    }
}
impl LBDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LBDF_A {
        match self.bits {
            false => LBDF_A::B_0X0,
            true => LBDF_A::B_0X1,
        }
    }
    #[doc = "LIN Break not detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LBDF_A::B_0X0
    }
    #[doc = "LIN break detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LBDF_A::B_0X1
    }
}
#[doc = "Field `CTSIF` reader - CTS interrupt flag This bit is set by hardware when the nCTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the USART_ICR register. An interrupt is generated if CTSIE=1 in the USART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
pub type CTSIF_R = crate::BitReader<CTSIF_A>;
#[doc = "CTS interrupt flag This bit is set by hardware when the nCTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the USART_ICR register. An interrupt is generated if CTSIE=1 in the USART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSIF_A {
    #[doc = "0: No change occurred on the nCTS status line"]
    B_0X0 = 0,
    #[doc = "1: A change occurred on the nCTS status line"]
    B_0X1 = 1,
}
impl From<CTSIF_A> for bool {
    #[inline(always)]
    fn from(variant: CTSIF_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTSIF_A {
        match self.bits {
            false => CTSIF_A::B_0X0,
            true => CTSIF_A::B_0X1,
        }
    }
    #[doc = "No change occurred on the nCTS status line"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTSIF_A::B_0X0
    }
    #[doc = "A change occurred on the nCTS status line"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTSIF_A::B_0X1
    }
}
#[doc = "Field `CTS` reader - CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the nCTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
pub type CTS_R = crate::BitReader<CTS_A>;
#[doc = "CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the nCTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTS_A {
    #[doc = "0: nCTS line set"]
    B_0X0 = 0,
    #[doc = "1: nCTS line reset"]
    B_0X1 = 1,
}
impl From<CTS_A> for bool {
    #[inline(always)]
    fn from(variant: CTS_A) -> Self {
        variant as u8 != 0
    }
}
impl CTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTS_A {
        match self.bits {
            false => CTS_A::B_0X0,
            true => CTS_A::B_0X1,
        }
    }
    #[doc = "nCTS line set"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTS_A::B_0X0
    }
    #[doc = "nCTS line reset"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTS_A::B_0X1
    }
}
#[doc = "Field `RTOF` reader - Receiver timeout This bit is set by hardware when the timeout value, programmed in the RTOR register has lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in the USART_ICR register. An interrupt is generated if RTOIE=1 in the USART_CR2 register. In Smartcard mode, the timeout corresponds to the CWT or BWT timings. Note: If a time equal to the value programmed in RTOR register separates 2 characters, RTOF is not set. If this time exceeds this value + 2 sample times (2/16 or 2/8, depending on the oversampling method), RTOF flag is set. The counter counts even if RE = 0 but RTOF is set only when RE = 1. If the timeout has already elapsed when RE is set, then RTOF is set. If the USART does not support the Receiver timeout feature, this bit is reserved and kept at reset value."]
pub type RTOF_R = crate::BitReader<RTOF_A>;
#[doc = "Receiver timeout This bit is set by hardware when the timeout value, programmed in the RTOR register has lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in the USART_ICR register. An interrupt is generated if RTOIE=1 in the USART_CR2 register. In Smartcard mode, the timeout corresponds to the CWT or BWT timings. Note: If a time equal to the value programmed in RTOR register separates 2 characters, RTOF is not set. If this time exceeds this value + 2 sample times (2/16 or 2/8, depending on the oversampling method), RTOF flag is set. The counter counts even if RE = 0 but RTOF is set only when RE = 1. If the timeout has already elapsed when RE is set, then RTOF is set. If the USART does not support the Receiver timeout feature, this bit is reserved and kept at reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOF_A {
    #[doc = "0: Timeout value not reached"]
    B_0X0 = 0,
    #[doc = "1: Timeout value reached without any data reception"]
    B_0X1 = 1,
}
impl From<RTOF_A> for bool {
    #[inline(always)]
    fn from(variant: RTOF_A) -> Self {
        variant as u8 != 0
    }
}
impl RTOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTOF_A {
        match self.bits {
            false => RTOF_A::B_0X0,
            true => RTOF_A::B_0X1,
        }
    }
    #[doc = "Timeout value not reached"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTOF_A::B_0X0
    }
    #[doc = "Timeout value reached without any data reception"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTOF_A::B_0X1
    }
}
#[doc = "Field `EOBF` reader - End of block flag This bit is set by hardware when a complete block has been received (for example T=1 Smartcard mode). The detection is done when the number of received bytes (from the start of the block, including the prologue) is equal or greater than BLEN + 4. An interrupt is generated if the EOBIE=1 in the USART_CR2 register. It is cleared by software, writing 1 to the EOBCF in the USART_ICR register. Note: If Smartcard mode is not supported, this bit is reserved and kept at reset value. Refer to ."]
pub type EOBF_R = crate::BitReader<EOBF_A>;
#[doc = "End of block flag This bit is set by hardware when a complete block has been received (for example T=1 Smartcard mode). The detection is done when the number of received bytes (from the start of the block, including the prologue) is equal or greater than BLEN + 4. An interrupt is generated if the EOBIE=1 in the USART_CR2 register. It is cleared by software, writing 1 to the EOBCF in the USART_ICR register. Note: If Smartcard mode is not supported, this bit is reserved and kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOBF_A {
    #[doc = "0: End of Block not reached"]
    B_0X0 = 0,
    #[doc = "1: End of Block (number of characters) reached"]
    B_0X1 = 1,
}
impl From<EOBF_A> for bool {
    #[inline(always)]
    fn from(variant: EOBF_A) -> Self {
        variant as u8 != 0
    }
}
impl EOBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOBF_A {
        match self.bits {
            false => EOBF_A::B_0X0,
            true => EOBF_A::B_0X1,
        }
    }
    #[doc = "End of Block not reached"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOBF_A::B_0X0
    }
    #[doc = "End of Block (number of characters) reached"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOBF_A::B_0X1
    }
}
#[doc = "Field `UDR` reader - SPI slave underrun error flag In slave transmission mode, this flag is set when the first clock pulse for data transmission appears while the software has not yet loaded any value into USART_TDR. This flag is reset by setting UDRCF bit in the USART_ICR register. Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at reset value. Refer to ."]
pub type UDR_R = crate::BitReader<UDR_A>;
#[doc = "SPI slave underrun error flag In slave transmission mode, this flag is set when the first clock pulse for data transmission appears while the software has not yet loaded any value into USART_TDR. This flag is reset by setting UDRCF bit in the USART_ICR register. Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDR_A {
    #[doc = "0: No underrun error"]
    B_0X0 = 0,
    #[doc = "1: underrun error"]
    B_0X1 = 1,
}
impl From<UDR_A> for bool {
    #[inline(always)]
    fn from(variant: UDR_A) -> Self {
        variant as u8 != 0
    }
}
impl UDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UDR_A {
        match self.bits {
            false => UDR_A::B_0X0,
            true => UDR_A::B_0X1,
        }
    }
    #[doc = "No underrun error"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UDR_A::B_0X0
    }
    #[doc = "underrun error"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UDR_A::B_0X1
    }
}
#[doc = "Field `ABRE` reader - Auto baud rate error This bit is set by hardware if the baud rate measurement failed (baud rate out of range or character comparison failed) It is cleared by software, by writing 1 to the ABRRQ bit in the USART_CR3 register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value."]
pub type ABRE_R = crate::BitReader;
#[doc = "Field `ABRF` reader - Auto baud rate flag This bit is set by hardware when the automatic baud rate has been set (RXFNE is also set, generating an interrupt if RXFNEIE = 1) or when the auto baud rate operation was completed without success (ABRE=1) (ABRE, RXFNE and FE are also set in this case) It is cleared by software, in order to request a new auto baud rate detection, by writing 1 to the ABRRQ in the USART_RQR register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value."]
pub type ABRF_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not)."]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: USART is idle (no reception)"]
    B_0X0 = 0,
    #[doc = "1: Reception on going"]
    B_0X1 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::B_0X0,
            true => BUSY_A::B_0X1,
        }
    }
    #[doc = "USART is idle (no reception)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BUSY_A::B_0X0
    }
    #[doc = "Reception on going"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BUSY_A::B_0X1
    }
}
#[doc = "Field `CMF` reader - Character match flag This bit is set by hardware, when a the character defined by ADD\\[7:0\\]
is received. It is cleared by software, writing 1 to the CMCF in the USART_ICR register. An interrupt is generated if CMIE=1in the USART_CR1 register."]
pub type CMF_R = crate::BitReader<CMF_A>;
#[doc = "Character match flag This bit is set by hardware, when a the character defined by ADD\\[7:0\\]
is received. It is cleared by software, writing 1 to the CMCF in the USART_ICR register. An interrupt is generated if CMIE=1in the USART_CR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMF_A {
    #[doc = "0: No Character match detected"]
    B_0X0 = 0,
    #[doc = "1: Character Match detected"]
    B_0X1 = 1,
}
impl From<CMF_A> for bool {
    #[inline(always)]
    fn from(variant: CMF_A) -> Self {
        variant as u8 != 0
    }
}
impl CMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMF_A {
        match self.bits {
            false => CMF_A::B_0X0,
            true => CMF_A::B_0X1,
        }
    }
    #[doc = "No Character match detected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CMF_A::B_0X0
    }
    #[doc = "Character Match detected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CMF_A::B_0X1
    }
}
#[doc = "Field `SBKF` reader - Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission."]
pub type SBKF_R = crate::BitReader<SBKF_A>;
#[doc = "Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBKF_A {
    #[doc = "0: Break character transmitted"]
    B_0X0 = 0,
    #[doc = "1: Break character requested by setting SBKRQ bit in USART_RQR register"]
    B_0X1 = 1,
}
impl From<SBKF_A> for bool {
    #[inline(always)]
    fn from(variant: SBKF_A) -> Self {
        variant as u8 != 0
    }
}
impl SBKF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBKF_A {
        match self.bits {
            false => SBKF_A::B_0X0,
            true => SBKF_A::B_0X1,
        }
    }
    #[doc = "Break character transmitted"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SBKF_A::B_0X0
    }
    #[doc = "Break character requested by setting SBKRQ bit in USART_RQR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SBKF_A::B_0X1
    }
}
#[doc = "Field `RWU` reader - Receiver wakeup from Mute mode This bit indicates if the USART is in Mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the USART_CR1 register. When wakeup on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the USART_RQR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to ."]
pub type RWU_R = crate::BitReader<RWU_A>;
#[doc = "Receiver wakeup from Mute mode This bit indicates if the USART is in Mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the USART_CR1 register. When wakeup on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the USART_RQR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWU_A {
    #[doc = "0: Receiver in active mode"]
    B_0X0 = 0,
    #[doc = "1: Receiver in Mute mode"]
    B_0X1 = 1,
}
impl From<RWU_A> for bool {
    #[inline(always)]
    fn from(variant: RWU_A) -> Self {
        variant as u8 != 0
    }
}
impl RWU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RWU_A {
        match self.bits {
            false => RWU_A::B_0X0,
            true => RWU_A::B_0X1,
        }
    }
    #[doc = "Receiver in active mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RWU_A::B_0X0
    }
    #[doc = "Receiver in Mute mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RWU_A::B_0X1
    }
}
#[doc = "Field `WUF` reader - Wakeup from low-power mode flag This bit is set by hardware, when a wakeup event is detected. The event is defined by the WUS bitfield. It is cleared by software, writing a 1 to the WUCF in the USART_ICR register. An interrupt is generated if WUFIE=1 in the USART_CR3 register. Note: When UESM is cleared, WUF flag is also cleared. If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to ."]
pub type WUF_R = crate::BitReader;
#[doc = "Field `TEACK` reader - Transmit enable acknowledge flag This bit is set/reset by hardware, when the Transmit Enable value is taken into account by the USART. It can be used when an idle frame request is generated by writing TE=0, followed by TE=1 in the USART_CR1 register, in order to respect the TE=0 minimum period."]
pub type TEACK_R = crate::BitReader;
#[doc = "Field `REACK` reader - Receive enable acknowledge flag This bit is set/reset by hardware, when the Receive Enable value is taken into account by the USART. It can be used to verify that the USART is ready for reception before entering low-power mode. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to ."]
pub type REACK_R = crate::BitReader;
#[doc = "Field `TXFE` reader - TXFIFO empty This bit is set by hardware when TXFIFO is empty. When the TXFIFO contains at least one data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4) in the USART_RQR register. An interrupt is generated if the TXFEIE bit =1 (bit 30) in the USART_CR1 register."]
pub type TXFE_R = crate::BitReader<TXFE_A>;
#[doc = "TXFIFO empty This bit is set by hardware when TXFIFO is empty. When the TXFIFO contains at least one data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4) in the USART_RQR register. An interrupt is generated if the TXFEIE bit =1 (bit 30) in the USART_CR1 register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFE_A {
    #[doc = "0: TXFIFO not empty."]
    B_0X0 = 0,
    #[doc = "1: TXFIFO empty."]
    B_0X1 = 1,
}
impl From<TXFE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXFE_A {
        match self.bits {
            false => TXFE_A::B_0X0,
            true => TXFE_A::B_0X1,
        }
    }
    #[doc = "TXFIFO not empty."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXFE_A::B_0X0
    }
    #[doc = "TXFIFO empty."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXFE_A::B_0X1
    }
}
#[doc = "Field `RXFF` reader - RXFIFO full This bit is set by hardware when the number of received data corresponds to RXFIFOsize+1 (RXFIFO full + 1 data in the USART_RDR register. An interrupt is generated if the RXFFIE bit =1 in the USART_CR1 register."]
pub type RXFF_R = crate::BitReader<RXFF_A>;
#[doc = "RXFIFO full This bit is set by hardware when the number of received data corresponds to RXFIFOsize+1 (RXFIFO full + 1 data in the USART_RDR register. An interrupt is generated if the RXFFIE bit =1 in the USART_CR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFF_A {
    #[doc = "0: RXFIFO not full."]
    B_0X0 = 0,
    #[doc = "1: RXFIFO Full."]
    B_0X1 = 1,
}
impl From<RXFF_A> for bool {
    #[inline(always)]
    fn from(variant: RXFF_A) -> Self {
        variant as u8 != 0
    }
}
impl RXFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXFF_A {
        match self.bits {
            false => RXFF_A::B_0X0,
            true => RXFF_A::B_0X1,
        }
    }
    #[doc = "RXFIFO not full."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXFF_A::B_0X0
    }
    #[doc = "RXFIFO Full."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXFF_A::B_0X1
    }
}
#[doc = "Field `TCBGT` reader - Transmission complete before guard time flag This bit is set when the last data written in the USART_TDR has been transmitted correctly out of the shift register. It is set by hardware in Smartcard mode, if the transmission of a frame containing data is complete and if the smartcard did not send back any NACK. An interrupt is generated if TCBGTIE=1 in the USART_CR3 register. This bit is cleared by software, by writing 1 to the TCBGTCF in the USART_ICR register or by a write to the USART_TDR register. Note: If the USART does not support the Smartcard mode, this bit is reserved and kept at reset value. If the USART supports the Smartcard mode and the Smartcard mode is enabled, the TCBGT reset value is '1'. Refer to on page835."]
pub type TCBGT_R = crate::BitReader<TCBGT_A>;
#[doc = "Transmission complete before guard time flag This bit is set when the last data written in the USART_TDR has been transmitted correctly out of the shift register. It is set by hardware in Smartcard mode, if the transmission of a frame containing data is complete and if the smartcard did not send back any NACK. An interrupt is generated if TCBGTIE=1 in the USART_CR3 register. This bit is cleared by software, by writing 1 to the TCBGTCF in the USART_ICR register or by a write to the USART_TDR register. Note: If the USART does not support the Smartcard mode, this bit is reserved and kept at reset value. If the USART supports the Smartcard mode and the Smartcard mode is enabled, the TCBGT reset value is '1'. Refer to on page835.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCBGT_A {
    #[doc = "0: Transmission is not complete or transmission is complete unsuccessfully (i.e. a NACK is received from the card)"]
    B_0X0 = 0,
    #[doc = "1: Transmission is complete successfully (before Guard time completion and there is no NACK from the smart card)."]
    B_0X1 = 1,
}
impl From<TCBGT_A> for bool {
    #[inline(always)]
    fn from(variant: TCBGT_A) -> Self {
        variant as u8 != 0
    }
}
impl TCBGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCBGT_A {
        match self.bits {
            false => TCBGT_A::B_0X0,
            true => TCBGT_A::B_0X1,
        }
    }
    #[doc = "Transmission is not complete or transmission is complete unsuccessfully (i.e. a NACK is received from the card)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCBGT_A::B_0X0
    }
    #[doc = "Transmission is complete successfully (before Guard time completion and there is no NACK from the smart card)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCBGT_A::B_0X1
    }
}
#[doc = "Field `RXFT` reader - RXFIFO threshold flag This bit is set by hardware when the threshold programmed in RXFTCFG in USART_CR3 register is reached. This means that there are (RXFTCFG - 1) data in the Receive FIFO and one data in the USART_RDR register. An interrupt is generated if the RXFTIE bit =1 (bit 27) in the USART_CR3 register. Note: When the RXFTCFG threshold is configured to '101', RXFT flag is set if 16 data are available i.e. 15 data in the RXFIFO and 1 data in the USART_RDR. Consequently, the 17th received data does not cause an overrun error. The overrun error occurs after receiving the 18th data."]
pub type RXFT_R = crate::BitReader<RXFT_A>;
#[doc = "RXFIFO threshold flag This bit is set by hardware when the threshold programmed in RXFTCFG in USART_CR3 register is reached. This means that there are (RXFTCFG - 1) data in the Receive FIFO and one data in the USART_RDR register. An interrupt is generated if the RXFTIE bit =1 (bit 27) in the USART_CR3 register. Note: When the RXFTCFG threshold is configured to '101', RXFT flag is set if 16 data are available i.e. 15 data in the RXFIFO and 1 data in the USART_RDR. Consequently, the 17th received data does not cause an overrun error. The overrun error occurs after receiving the 18th data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFT_A {
    #[doc = "0: Receive FIFO does not reach the programmed threshold."]
    B_0X0 = 0,
    #[doc = "1: Receive FIFO reached the programmed threshold."]
    B_0X1 = 1,
}
impl From<RXFT_A> for bool {
    #[inline(always)]
    fn from(variant: RXFT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXFT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXFT_A {
        match self.bits {
            false => RXFT_A::B_0X0,
            true => RXFT_A::B_0X1,
        }
    }
    #[doc = "Receive FIFO does not reach the programmed threshold."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXFT_A::B_0X0
    }
    #[doc = "Receive FIFO reached the programmed threshold."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXFT_A::B_0X1
    }
}
#[doc = "Field `TXFT` reader - TXFIFO threshold flag This bit is set by hardware when the TXFIFO reaches the threshold programmed in TXFTCFG of USART_CR3 register i.e. the TXFIFO contains TXFTCFG empty locations. An interrupt is generated if the TXFTIE bit =1 (bit 31) in the USART_CR3 register."]
pub type TXFT_R = crate::BitReader<TXFT_A>;
#[doc = "TXFIFO threshold flag This bit is set by hardware when the TXFIFO reaches the threshold programmed in TXFTCFG of USART_CR3 register i.e. the TXFIFO contains TXFTCFG empty locations. An interrupt is generated if the TXFTIE bit =1 (bit 31) in the USART_CR3 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFT_A {
    #[doc = "0: TXFIFO does not reach the programmed threshold."]
    B_0X0 = 0,
    #[doc = "1: TXFIFO reached the programmed threshold."]
    B_0X1 = 1,
}
impl From<TXFT_A> for bool {
    #[inline(always)]
    fn from(variant: TXFT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXFT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXFT_A {
        match self.bits {
            false => TXFT_A::B_0X0,
            true => TXFT_A::B_0X1,
        }
    }
    #[doc = "TXFIFO does not reach the programmed threshold."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXFT_A::B_0X0
    }
    #[doc = "TXFIFO reached the programmed threshold."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXFT_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - Parity error This bit is set by hardware when a parity error occurs in receiver mode. It is cleared by software, writing 1 to the PECF in the USART_ICR register. An interrupt is generated if PEIE = 1 in the USART_CR1 register. Note: This error is associated with the character in the USART_RDR."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE=1 in the USART_CR1 register. Note: This error is associated with the character in the USART_RDR."]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise detection flag This bit is set by hardware when noise is detected on a received frame. It is cleared by software, writing 1 to the NECF bit in the USART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXFNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. When the line is noise-free, the NE flag can be disabled by programming the ONEBIT bit to 1 to increase the USART tolerance to deviations (Refer to Tolerance of the USART receiver to clock deviation on page861). This error is associated with the character in the USART_RDR."]
    #[inline(always)]
    pub fn ne(&self) -> NE_R {
        NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the USART_RDR register while RXFF = 1. It is cleared by a software, writing 1 to the ORECF, in the USART_ICR register. An interrupt is generated if RXFNEIE=1 or EIE = 1 in the USART_CR1 register. Note: When this bit is set, the USART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the USART_CR3 register."]
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Idle line detected This bit is set by hardware when an Idle Line is detected. An interrupt is generated if IDLEIE=1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the USART_ICR register. Note: The IDLE bit is not set again until the RXFNE bit has been set (i.e. a new idle line occurs). If Mute mode is enabled (MME=1), IDLE is set if the USART is not mute (RWU=0), whatever the Mute mode selected by the WAKE bit. If RWU=1, IDLE is not set."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXFIFO not empty RXFNE bit is set by hardware when the RXFIFO is not empty, meaning that data can be read from the USART_RDR register. Every read operation from the USART_RDR frees a location in the RXFIFO. RXFNE is cleared when the RXFIFO is empty. The RXFNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXFNEIE=1 in the USART_CR1 register."]
    #[inline(always)]
    pub fn rxfne(&self) -> RXFNE_R {
        RXFNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete This bit indicates that the last data written in the USART_TDR has been transmitted out of the shift register. It is set by hardware when the transmission of a frame containing data is complete and when TXFE is set. An interrupt is generated if TCIE=1 in the USART_CR1 register. TC bit is is cleared by software, by writing 1 to the TCCF in the USART_ICR register or by a write to the USART_TDR register. Note: If TE bit is reset and no transmission is on going, the TC bit is immediately set."]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXFIFO not full TXFNF is set by hardware when TXFIFO is not full meaning that data can be written in the USART_TDR. Every write operation to the USART_TDR places the data in the TXFIFO. This flag remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared indicating that data can not be written into the USART_TDR. An interrupt is generated if the TXFNFIE bit =1 in the USART_CR1 register. Note: The TXFNF is kept reset during the flush request until TXFIFO is empty. After sending the flush request (by setting TXFRQ bit), the flag TXFNF should be checked prior to writing in TXFIFO (TXFNF and TXFE are set at the same time). This bit is used during single buffer transmission."]
    #[inline(always)]
    pub fn txfnf(&self) -> TXFNF_R {
        TXFNF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LIN break detection flag This bit is set by hardware when the LIN break is detected. It is cleared by software, by writing 1 to the LBDCF in the USART_ICR. An interrupt is generated if LBDIE = 1 in the USART_CR2 register. Note: If the USART does not support LIN mode, this bit is reserved and kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn lbdf(&self) -> LBDF_R {
        LBDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS interrupt flag This bit is set by hardware when the nCTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the USART_ICR register. An interrupt is generated if CTSIE=1 in the USART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn ctsif(&self) -> CTSIF_R {
        CTSIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the nCTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiver timeout This bit is set by hardware when the timeout value, programmed in the RTOR register has lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in the USART_ICR register. An interrupt is generated if RTOIE=1 in the USART_CR2 register. In Smartcard mode, the timeout corresponds to the CWT or BWT timings. Note: If a time equal to the value programmed in RTOR register separates 2 characters, RTOF is not set. If this time exceeds this value + 2 sample times (2/16 or 2/8, depending on the oversampling method), RTOF flag is set. The counter counts even if RE = 0 but RTOF is set only when RE = 1. If the timeout has already elapsed when RE is set, then RTOF is set. If the USART does not support the Receiver timeout feature, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn rtof(&self) -> RTOF_R {
        RTOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - End of block flag This bit is set by hardware when a complete block has been received (for example T=1 Smartcard mode). The detection is done when the number of received bytes (from the start of the block, including the prologue) is equal or greater than BLEN + 4. An interrupt is generated if the EOBIE=1 in the USART_CR2 register. It is cleared by software, writing 1 to the EOBCF in the USART_ICR register. Note: If Smartcard mode is not supported, this bit is reserved and kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn eobf(&self) -> EOBF_R {
        EOBF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI slave underrun error flag In slave transmission mode, this flag is set when the first clock pulse for data transmission appears while the software has not yet loaded any value into USART_TDR. This flag is reset by setting UDRCF bit in the USART_ICR register. Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Auto baud rate error This bit is set by hardware if the baud rate measurement failed (baud rate out of range or character comparison failed) It is cleared by software, by writing 1 to the ABRRQ bit in the USART_CR3 register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn abre(&self) -> ABRE_R {
        ABRE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Auto baud rate flag This bit is set by hardware when the automatic baud rate has been set (RXFNE is also set, generating an interrupt if RXFNEIE = 1) or when the auto baud rate operation was completed without success (ABRE=1) (ABRE, RXFNE and FE are also set in this case) It is cleared by software, in order to request a new auto baud rate detection, by writing 1 to the ABRRQ in the USART_RQR register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn abrf(&self) -> ABRF_R {
        ABRF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not)."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Character match flag This bit is set by hardware, when a the character defined by ADD\\[7:0\\]
is received. It is cleared by software, writing 1 to the CMCF in the USART_ICR register. An interrupt is generated if CMIE=1in the USART_CR1 register."]
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission."]
    #[inline(always)]
    pub fn sbkf(&self) -> SBKF_R {
        SBKF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receiver wakeup from Mute mode This bit indicates if the USART is in Mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the USART_CR1 register. When wakeup on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the USART_RQR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wakeup from low-power mode flag This bit is set by hardware, when a wakeup event is detected. The event is defined by the WUS bitfield. It is cleared by software, writing a 1 to the WUCF in the USART_ICR register. An interrupt is generated if WUFIE=1 in the USART_CR3 register. Note: When UESM is cleared, WUF flag is also cleared. If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit enable acknowledge flag This bit is set/reset by hardware, when the Transmit Enable value is taken into account by the USART. It can be used when an idle frame request is generated by writing TE=0, followed by TE=1 in the USART_CR1 register, in order to respect the TE=0 minimum period."]
    #[inline(always)]
    pub fn teack(&self) -> TEACK_R {
        TEACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Receive enable acknowledge flag This bit is set/reset by hardware, when the Receive Enable value is taken into account by the USART. It can be used to verify that the USART is ready for reception before entering low-power mode. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn reack(&self) -> REACK_R {
        REACK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TXFIFO empty This bit is set by hardware when TXFIFO is empty. When the TXFIFO contains at least one data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4) in the USART_RQR register. An interrupt is generated if the TXFEIE bit =1 (bit 30) in the USART_CR1 register."]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RXFIFO full This bit is set by hardware when the number of received data corresponds to RXFIFOsize+1 (RXFIFO full + 1 data in the USART_RDR register. An interrupt is generated if the RXFFIE bit =1 in the USART_CR1 register."]
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmission complete before guard time flag This bit is set when the last data written in the USART_TDR has been transmitted correctly out of the shift register. It is set by hardware in Smartcard mode, if the transmission of a frame containing data is complete and if the smartcard did not send back any NACK. An interrupt is generated if TCBGTIE=1 in the USART_CR3 register. This bit is cleared by software, by writing 1 to the TCBGTCF in the USART_ICR register or by a write to the USART_TDR register. Note: If the USART does not support the Smartcard mode, this bit is reserved and kept at reset value. If the USART supports the Smartcard mode and the Smartcard mode is enabled, the TCBGT reset value is '1'. Refer to on page835."]
    #[inline(always)]
    pub fn tcbgt(&self) -> TCBGT_R {
        TCBGT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - RXFIFO threshold flag This bit is set by hardware when the threshold programmed in RXFTCFG in USART_CR3 register is reached. This means that there are (RXFTCFG - 1) data in the Receive FIFO and one data in the USART_RDR register. An interrupt is generated if the RXFTIE bit =1 (bit 27) in the USART_CR3 register. Note: When the RXFTCFG threshold is configured to '101', RXFT flag is set if 16 data are available i.e. 15 data in the RXFIFO and 1 data in the USART_RDR. Consequently, the 17th received data does not cause an overrun error. The overrun error occurs after receiving the 18th data."]
    #[inline(always)]
    pub fn rxft(&self) -> RXFT_R {
        RXFT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TXFIFO threshold flag This bit is set by hardware when the TXFIFO reaches the threshold programmed in TXFTCFG of USART_CR3 register i.e. the TXFIFO contains TXFTCFG empty locations. An interrupt is generated if the TXFTIE bit =1 (bit 31) in the USART_CR3 register."]
    #[inline(always)]
    pub fn txft(&self) -> TXFT_R {
        TXFT_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Interrupt &amp; status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr_fifo_enabled::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_FIFO_ENABLED_SPEC;
impl crate::RegisterSpec for ISR_FIFO_ENABLED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr_fifo_enabled::R`](R) reader structure"]
impl crate::Readable for ISR_FIFO_ENABLED_SPEC {}
#[doc = "`reset()` method sets ISR_FIFO_ENABLED to value 0x0080_00c0"]
impl crate::Resettable for ISR_FIFO_ENABLED_SPEC {
    const RESET_VALUE: u32 = 0x0080_00c0;
}
