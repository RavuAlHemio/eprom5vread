#[doc = "Register `CR1_FIFO_DISABLED` reader"]
pub type R = crate::R<CR1_FIFO_DISABLED_SPEC>;
#[doc = "Register `CR1_FIFO_DISABLED` writer"]
pub type W = crate::W<CR1_FIFO_DISABLED_SPEC>;
#[doc = "Field `UE` reader - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. In Smartcard mode, (SCEN = 1), the SCLK is always available when CLKEN = 1, regardless of the UE bit value."]
pub type UE_R = crate::BitReader<UE_A>;
#[doc = "USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. In Smartcard mode, (SCEN = 1), the SCLK is always available when CLKEN = 1, regardless of the UE bit value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UE_A {
    #[doc = "0: USART prescaler and outputs disabled, low-power mode"]
    B_0X0 = 0,
    #[doc = "1: USART enabled"]
    B_0X1 = 1,
}
impl From<UE_A> for bool {
    #[inline(always)]
    fn from(variant: UE_A) -> Self {
        variant as u8 != 0
    }
}
impl UE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UE_A {
        match self.bits {
            false => UE_A::B_0X0,
            true => UE_A::B_0X1,
        }
    }
    #[doc = "USART prescaler and outputs disabled, low-power mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UE_A::B_0X0
    }
    #[doc = "USART enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UE_A::B_0X1
    }
}
#[doc = "Field `UE` writer - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. In Smartcard mode, (SCEN = 1), the SCLK is always available when CLKEN = 1, regardless of the UE bit value."]
pub type UE_W<'a, REG> = crate::BitWriter<'a, REG, UE_A>;
impl<'a, REG> UE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART prescaler and outputs disabled, low-power mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UE_A::B_0X0)
    }
    #[doc = "USART enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UE_A::B_0X1)
    }
}
#[doc = "Field `UESM` reader - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type UESM_R = crate::BitReader<UESM_A>;
#[doc = "USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UESM_A {
    #[doc = "0: USART not able to wake up the MCU from low-power mode."]
    B_0X0 = 0,
    #[doc = "1: USART able to wake up the MCU from low-power mode."]
    B_0X1 = 1,
}
impl From<UESM_A> for bool {
    #[inline(always)]
    fn from(variant: UESM_A) -> Self {
        variant as u8 != 0
    }
}
impl UESM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UESM_A {
        match self.bits {
            false => UESM_A::B_0X0,
            true => UESM_A::B_0X1,
        }
    }
    #[doc = "USART not able to wake up the MCU from low-power mode."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UESM_A::B_0X0
    }
    #[doc = "USART able to wake up the MCU from low-power mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UESM_A::B_0X1
    }
}
#[doc = "Field `UESM` writer - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type UESM_W<'a, REG> = crate::BitWriter<'a, REG, UESM_A>;
impl<'a, REG> UESM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART not able to wake up the MCU from low-power mode."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UESM_A::B_0X0)
    }
    #[doc = "USART able to wake up the MCU from low-power mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UESM_A::B_0X1)
    }
}
#[doc = "Field `RE` reader - Receiver enable This bit enables the receiver. It is set and cleared by software."]
pub type RE_R = crate::BitReader<RE_A>;
#[doc = "Receiver enable This bit enables the receiver. It is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE_A {
    #[doc = "0: Receiver is disabled"]
    B_0X0 = 0,
    #[doc = "1: Receiver is enabled and begins searching for a start bit"]
    B_0X1 = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
impl RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::B_0X0,
            true => RE_A::B_0X1,
        }
    }
    #[doc = "Receiver is disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RE_A::B_0X0
    }
    #[doc = "Receiver is enabled and begins searching for a start bit"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RE_A::B_0X1
    }
}
#[doc = "Field `RE` writer - Receiver enable This bit enables the receiver. It is set and cleared by software."]
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG, RE_A>;
impl<'a, REG> RE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RE_A::B_0X0)
    }
    #[doc = "Receiver is enabled and begins searching for a start bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RE_A::B_0X1)
    }
}
#[doc = "Field `TE` reader - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ('0' followed by '1') sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to '1'. To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts."]
pub type TE_R = crate::BitReader<TE_A>;
#[doc = "Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ('0' followed by '1') sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to '1'. To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE_A {
    #[doc = "0: Transmitter is disabled"]
    B_0X0 = 0,
    #[doc = "1: Transmitter is enabled"]
    B_0X1 = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
impl TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::B_0X0,
            true => TE_A::B_0X1,
        }
    }
    #[doc = "Transmitter is disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TE_A::B_0X0
    }
    #[doc = "Transmitter is enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TE_A::B_0X1
    }
}
#[doc = "Field `TE` writer - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ('0' followed by '1') sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to '1'. To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts."]
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG, TE_A>;
impl<'a, REG> TE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmitter is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TE_A::B_0X0)
    }
    #[doc = "Transmitter is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TE_A::B_0X1)
    }
}
#[doc = "Field `IDLEIE` reader - IDLE interrupt enable This bit is set and cleared by software."]
pub type IDLEIE_R = crate::BitReader<IDLEIE_A>;
#[doc = "IDLE interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLEIE_A {
    #[doc = "0: Interrupt inhibited"]
    B_0X0 = 0,
    #[doc = "1: USART interrupt generated whenever IDLE = 1 in the USART_ISR register"]
    B_0X1 = 1,
}
impl From<IDLEIE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDLEIE_A {
        match self.bits {
            false => IDLEIE_A::B_0X0,
            true => IDLEIE_A::B_0X1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IDLEIE_A::B_0X0
    }
    #[doc = "USART interrupt generated whenever IDLE = 1 in the USART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IDLEIE_A::B_0X1
    }
}
#[doc = "Field `IDLEIE` writer - IDLE interrupt enable This bit is set and cleared by software."]
pub type IDLEIE_W<'a, REG> = crate::BitWriter<'a, REG, IDLEIE_A>;
impl<'a, REG> IDLEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEIE_A::B_0X0)
    }
    #[doc = "USART interrupt generated whenever IDLE = 1 in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEIE_A::B_0X1)
    }
}
#[doc = "Field `RXNEIE` reader - Receive data register not empty This bit is set and cleared by software."]
pub type RXNEIE_R = crate::BitReader<RXNEIE_A>;
#[doc = "Receive data register not empty This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNEIE_A {
    #[doc = "0: Interrupt inhibited"]
    B_0X0 = 0,
    #[doc = "1: USART interrupt generated whenever ORE = 1 or RXNE = 1 in the USART_ISR register"]
    B_0X1 = 1,
}
impl From<RXNEIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXNEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXNEIE_A {
        match self.bits {
            false => RXNEIE_A::B_0X0,
            true => RXNEIE_A::B_0X1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXNEIE_A::B_0X0
    }
    #[doc = "USART interrupt generated whenever ORE = 1 or RXNE = 1 in the USART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXNEIE_A::B_0X1
    }
}
#[doc = "Field `RXNEIE` writer - Receive data register not empty This bit is set and cleared by software."]
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG, RXNEIE_A>;
impl<'a, REG> RXNEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE_A::B_0X0)
    }
    #[doc = "USART interrupt generated whenever ORE = 1 or RXNE = 1 in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE_A::B_0X1)
    }
}
#[doc = "Field `TCIE` reader - Transmission complete interrupt enable This bit is set and cleared by software."]
pub type TCIE_R = crate::BitReader<TCIE_A>;
#[doc = "Transmission complete interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE_A {
    #[doc = "0: Interrupt inhibited"]
    B_0X0 = 0,
    #[doc = "1: USART interrupt generated whenever TC = 1 in the USART_ISR register"]
    B_0X1 = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::B_0X0,
            true => TCIE_A::B_0X1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIE_A::B_0X0
    }
    #[doc = "USART interrupt generated whenever TC = 1 in the USART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIE_A::B_0X1
    }
}
#[doc = "Field `TCIE` writer - Transmission complete interrupt enable This bit is set and cleared by software."]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE_A>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE_A::B_0X0)
    }
    #[doc = "USART interrupt generated whenever TC = 1 in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE_A::B_0X1)
    }
}
#[doc = "Field `TXEIE` reader - Transmit data register empty This bit is set and cleared by software."]
pub type TXEIE_R = crate::BitReader<TXEIE_A>;
#[doc = "Transmit data register empty This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEIE_A {
    #[doc = "0: Interrupt inhibited"]
    B_0X0 = 0,
    #[doc = "1: USART interrupt generated whenever TXE =1 in the USART_ISR register"]
    B_0X1 = 1,
}
impl From<TXEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXEIE_A {
        match self.bits {
            false => TXEIE_A::B_0X0,
            true => TXEIE_A::B_0X1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXEIE_A::B_0X0
    }
    #[doc = "USART interrupt generated whenever TXE =1 in the USART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXEIE_A::B_0X1
    }
}
#[doc = "Field `TXEIE` writer - Transmit data register empty This bit is set and cleared by software."]
pub type TXEIE_W<'a, REG> = crate::BitWriter<'a, REG, TXEIE_A>;
impl<'a, REG> TXEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE_A::B_0X0)
    }
    #[doc = "USART interrupt generated whenever TXE =1 in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE_A::B_0X1)
    }
}
#[doc = "Field `PEIE` reader - PE interrupt enable This bit is set and cleared by software."]
pub type PEIE_R = crate::BitReader<PEIE_A>;
#[doc = "PE interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEIE_A {
    #[doc = "0: Interrupt inhibited"]
    B_0X0 = 0,
    #[doc = "1: USART interrupt generated whenever PE = 1 in the USART_ISR register"]
    B_0X1 = 1,
}
impl From<PEIE_A> for bool {
    #[inline(always)]
    fn from(variant: PEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEIE_A {
        match self.bits {
            false => PEIE_A::B_0X0,
            true => PEIE_A::B_0X1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PEIE_A::B_0X0
    }
    #[doc = "USART interrupt generated whenever PE = 1 in the USART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PEIE_A::B_0X1
    }
}
#[doc = "Field `PEIE` writer - PE interrupt enable This bit is set and cleared by software."]
pub type PEIE_W<'a, REG> = crate::BitWriter<'a, REG, PEIE_A>;
impl<'a, REG> PEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PEIE_A::B_0X0)
    }
    #[doc = "USART interrupt generated whenever PE = 1 in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PEIE_A::B_0X1)
    }
}
#[doc = "Field `PS` reader - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type PS_R = crate::BitReader<PS_A>;
#[doc = "Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PS_A {
    #[doc = "0: Even parity"]
    B_0X0 = 0,
    #[doc = "1: Odd parity"]
    B_0X1 = 1,
}
impl From<PS_A> for bool {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as u8 != 0
    }
}
impl PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PS_A {
        match self.bits {
            false => PS_A::B_0X0,
            true => PS_A::B_0X1,
        }
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PS_A::B_0X0
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PS_A::B_0X1
    }
}
#[doc = "Field `PS` writer - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type PS_W<'a, REG> = crate::BitWriter<'a, REG, PS_A>;
impl<'a, REG> PS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PS_A::B_0X0)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PS_A::B_0X1)
    }
}
#[doc = "Field `PCE` reader - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE=0)."]
pub type PCE_R = crate::BitReader<PCE_A>;
#[doc = "Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCE_A {
    #[doc = "0: Parity control disabled"]
    B_0X0 = 0,
    #[doc = "1: Parity control enabled"]
    B_0X1 = 1,
}
impl From<PCE_A> for bool {
    #[inline(always)]
    fn from(variant: PCE_A) -> Self {
        variant as u8 != 0
    }
}
impl PCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCE_A {
        match self.bits {
            false => PCE_A::B_0X0,
            true => PCE_A::B_0X1,
        }
    }
    #[doc = "Parity control disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PCE_A::B_0X0
    }
    #[doc = "Parity control enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PCE_A::B_0X1
    }
}
#[doc = "Field `PCE` writer - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE=0)."]
pub type PCE_W<'a, REG> = crate::BitWriter<'a, REG, PCE_A>;
impl<'a, REG> PCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity control disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PCE_A::B_0X0)
    }
    #[doc = "Parity control enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PCE_A::B_0X1)
    }
}
#[doc = "Field `WAKE` reader - Receiver wakeup method This bit determines the USART wakeup method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type WAKE_R = crate::BitReader<WAKE_A>;
#[doc = "Receiver wakeup method This bit determines the USART wakeup method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKE_A {
    #[doc = "0: Idle line"]
    B_0X0 = 0,
    #[doc = "1: Address mark"]
    B_0X1 = 1,
}
impl From<WAKE_A> for bool {
    #[inline(always)]
    fn from(variant: WAKE_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAKE_A {
        match self.bits {
            false => WAKE_A::B_0X0,
            true => WAKE_A::B_0X1,
        }
    }
    #[doc = "Idle line"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WAKE_A::B_0X0
    }
    #[doc = "Address mark"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WAKE_A::B_0X1
    }
}
#[doc = "Field `WAKE` writer - Receiver wakeup method This bit determines the USART wakeup method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
pub type WAKE_W<'a, REG> = crate::BitWriter<'a, REG, WAKE_A>;
impl<'a, REG> WAKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Idle line"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WAKE_A::B_0X0)
    }
    #[doc = "Address mark"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WAKE_A::B_0X1)
    }
}
#[doc = "Field `M0` reader - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UE=0)."]
pub type M0_R = crate::BitReader;
#[doc = "Field `M0` writer - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UE=0)."]
pub type M0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MME` reader - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software."]
pub type MME_R = crate::BitReader<MME_A>;
#[doc = "Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MME_A {
    #[doc = "0: Receiver in active mode permanently"]
    B_0X0 = 0,
    #[doc = "1: Receiver can switch between Mute mode and active mode."]
    B_0X1 = 1,
}
impl From<MME_A> for bool {
    #[inline(always)]
    fn from(variant: MME_A) -> Self {
        variant as u8 != 0
    }
}
impl MME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MME_A {
        match self.bits {
            false => MME_A::B_0X0,
            true => MME_A::B_0X1,
        }
    }
    #[doc = "Receiver in active mode permanently"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MME_A::B_0X0
    }
    #[doc = "Receiver can switch between Mute mode and active mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MME_A::B_0X1
    }
}
#[doc = "Field `MME` writer - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software."]
pub type MME_W<'a, REG> = crate::BitWriter<'a, REG, MME_A>;
impl<'a, REG> MME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver in active mode permanently"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MME_A::B_0X0)
    }
    #[doc = "Receiver can switch between Mute mode and active mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MME_A::B_0X1)
    }
}
#[doc = "Field `CMIE` reader - Character match interrupt enable This bit is set and cleared by software."]
pub type CMIE_R = crate::BitReader<CMIE_A>;
#[doc = "Character match interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMIE_A {
    #[doc = "0: Interrupt inhibited"]
    B_0X0 = 0,
    #[doc = "1: USART interrupt generated when the CMF bit is set in the USART_ISR register."]
    B_0X1 = 1,
}
impl From<CMIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMIE_A {
        match self.bits {
            false => CMIE_A::B_0X0,
            true => CMIE_A::B_0X1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CMIE_A::B_0X0
    }
    #[doc = "USART interrupt generated when the CMF bit is set in the USART_ISR register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CMIE_A::B_0X1
    }
}
#[doc = "Field `CMIE` writer - Character match interrupt enable This bit is set and cleared by software."]
pub type CMIE_W<'a, REG> = crate::BitWriter<'a, REG, CMIE_A>;
impl<'a, REG> CMIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CMIE_A::B_0X0)
    }
    #[doc = "USART interrupt generated when the CMF bit is set in the USART_ISR register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CMIE_A::B_0X1)
    }
}
#[doc = "Field `OVER8` reader - Oversampling mode This bit can only be written when the USART is disabled (UE=0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared."]
pub type OVER8_R = crate::BitReader<OVER8_A>;
#[doc = "Oversampling mode This bit can only be written when the USART is disabled (UE=0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVER8_A {
    #[doc = "0: Oversampling by 16"]
    OVER16 = 0,
    #[doc = "1: Oversampling by 8"]
    OVER8 = 1,
}
impl From<OVER8_A> for bool {
    #[inline(always)]
    fn from(variant: OVER8_A) -> Self {
        variant as u8 != 0
    }
}
impl OVER8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVER8_A {
        match self.bits {
            false => OVER8_A::OVER16,
            true => OVER8_A::OVER8,
        }
    }
    #[doc = "Oversampling by 16"]
    #[inline(always)]
    pub fn is_over16(&self) -> bool {
        *self == OVER8_A::OVER16
    }
    #[doc = "Oversampling by 8"]
    #[inline(always)]
    pub fn is_over8(&self) -> bool {
        *self == OVER8_A::OVER8
    }
}
#[doc = "Field `OVER8` writer - Oversampling mode This bit can only be written when the USART is disabled (UE=0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared."]
pub type OVER8_W<'a, REG> = crate::BitWriter<'a, REG, OVER8_A>;
impl<'a, REG> OVER8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oversampling by 16"]
    #[inline(always)]
    pub fn over16(self) -> &'a mut crate::W<REG> {
        self.variant(OVER8_A::OVER16)
    }
    #[doc = "Oversampling by 8"]
    #[inline(always)]
    pub fn over8(self) -> &'a mut crate::W<REG> {
        self.variant(OVER8_A::OVER8)
    }
}
#[doc = "Field `DEDT` reader - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type DEDT_R = crate::FieldReader;
#[doc = "Field `DEDT` writer - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type DEDT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEAT` reader - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type DEAT_R = crate::FieldReader;
#[doc = "Field `DEAT` writer - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type DEAT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RTOIE` reader - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. ."]
pub type RTOIE_R = crate::BitReader<RTOIE_A>;
#[doc = "Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOIE_A {
    #[doc = "0: Interrupt inhibited"]
    B_0X0 = 0,
    #[doc = "1: USART interrupt generated when the RTOF bit is set in the USART_ISR register."]
    B_0X1 = 1,
}
impl From<RTOIE_A> for bool {
    #[inline(always)]
    fn from(variant: RTOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTOIE_A {
        match self.bits {
            false => RTOIE_A::B_0X0,
            true => RTOIE_A::B_0X1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTOIE_A::B_0X0
    }
    #[doc = "USART interrupt generated when the RTOF bit is set in the USART_ISR register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTOIE_A::B_0X1
    }
}
#[doc = "Field `RTOIE` writer - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. ."]
pub type RTOIE_W<'a, REG> = crate::BitWriter<'a, REG, RTOIE_A>;
impl<'a, REG> RTOIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RTOIE_A::B_0X0)
    }
    #[doc = "USART interrupt generated when the RTOF bit is set in the USART_ISR register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RTOIE_A::B_0X1)
    }
}
#[doc = "Field `EOBIE` reader - End of Block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type EOBIE_R = crate::BitReader<EOBIE_A>;
#[doc = "End of Block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOBIE_A {
    #[doc = "0: Interrupt inhibited"]
    B_0X0 = 0,
    #[doc = "1: USART interrupt generated when the EOBF flag is set in the USART_ISR register"]
    B_0X1 = 1,
}
impl From<EOBIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOBIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOBIE_A {
        match self.bits {
            false => EOBIE_A::B_0X0,
            true => EOBIE_A::B_0X1,
        }
    }
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOBIE_A::B_0X0
    }
    #[doc = "USART interrupt generated when the EOBF flag is set in the USART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOBIE_A::B_0X1
    }
}
#[doc = "Field `EOBIE` writer - End of Block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type EOBIE_W<'a, REG> = crate::BitWriter<'a, REG, EOBIE_A>;
impl<'a, REG> EOBIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOBIE_A::B_0X0)
    }
    #[doc = "USART interrupt generated when the EOBF flag is set in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOBIE_A::B_0X1)
    }
}
#[doc = "Field `M1` reader - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\\[1:0\\]
= '00': 1 start bit, 8 Data bits, n Stop bit M\\[1:0\\]
= '01': 1 start bit, 9 Data bits, n Stop bit M\\[1:0\\]
= '10': 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE=0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and Auto baud rate (0x7F and 0x55 frames detection) are not supported."]
pub type M1_R = crate::BitReader;
#[doc = "Field `M1` writer - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\\[1:0\\]
= '00': 1 start bit, 8 Data bits, n Stop bit M\\[1:0\\]
= '01': 1 start bit, 9 Data bits, n Stop bit M\\[1:0\\]
= '10': 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE=0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and Auto baud rate (0x7F and 0x55 frames detection) are not supported."]
pub type M1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEN` reader - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes."]
pub type FIFOEN_R = crate::BitReader<FIFOEN_A>;
#[doc = "FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOEN_A {
    #[doc = "0: FIFO mode is disabled."]
    B_0X0 = 0,
    #[doc = "1: FIFO mode is enabled."]
    B_0X1 = 1,
}
impl From<FIFOEN_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIFOEN_A {
        match self.bits {
            false => FIFOEN_A::B_0X0,
            true => FIFOEN_A::B_0X1,
        }
    }
    #[doc = "FIFO mode is disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FIFOEN_A::B_0X0
    }
    #[doc = "FIFO mode is enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FIFOEN_A::B_0X1
    }
}
#[doc = "Field `FIFOEN` writer - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes."]
pub type FIFOEN_W<'a, REG> = crate::BitWriter<'a, REG, FIFOEN_A>;
impl<'a, REG> FIFOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIFO mode is disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FIFOEN_A::B_0X0)
    }
    #[doc = "FIFO mode is enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FIFOEN_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. In Smartcard mode, (SCEN = 1), the SCLK is always available when CLKEN = 1, regardless of the UE bit value."]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable This bit enables the receiver. It is set and cleared by software."]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ('0' followed by '1') sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to '1'. To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts."]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive data register not empty This bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data register empty This bit is set and cleared by software."]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PE interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiver wakeup method This bit determines the USART wakeup method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software."]
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Character match interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn cmie(&self) -> CMIE_R {
        CMIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Oversampling mode This bit can only be written when the USART is disabled (UE=0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared."]
    #[inline(always)]
    pub fn over8(&self) -> OVER8_R {
        OVER8_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn dedt(&self) -> DEDT_R {
        DEDT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn deat(&self) -> DEAT_R {
        DEAT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. ."]
    #[inline(always)]
    pub fn rtoie(&self) -> RTOIE_R {
        RTOIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - End of Block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn eobie(&self) -> EOBIE_R {
        EOBIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\\[1:0\\]
= '00': 1 start bit, 8 Data bits, n Stop bit M\\[1:0\\]
= '01': 1 start bit, 9 Data bits, n Stop bit M\\[1:0\\]
= '10': 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE=0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and Auto baud rate (0x7F and 0x55 frames detection) are not supported."]
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes."]
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. In Smartcard mode, (SCEN = 1), the SCLK is always available when CLKEN = 1, regardless of the UE bit value."]
    #[inline(always)]
    #[must_use]
    pub fn ue(&mut self) -> UE_W<CR1_FIFO_DISABLED_SPEC> {
        UE_W::new(self, 0)
    }
    #[doc = "Bit 1 - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn uesm(&mut self) -> UESM_W<CR1_FIFO_DISABLED_SPEC> {
        UESM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver enable This bit enables the receiver. It is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<CR1_FIFO_DISABLED_SPEC> {
        RE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ('0' followed by '1') sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to '1'. To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts."]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<CR1_FIFO_DISABLED_SPEC> {
        TE_W::new(self, 3)
    }
    #[doc = "Bit 4 - IDLE interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn idleie(&mut self) -> IDLEIE_W<CR1_FIFO_DISABLED_SPEC> {
        IDLEIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Receive data register not empty This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<CR1_FIFO_DISABLED_SPEC> {
        RXNEIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CR1_FIFO_DISABLED_SPEC> {
        TCIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit data register empty This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TXEIE_W<CR1_FIFO_DISABLED_SPEC> {
        TXEIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - PE interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PEIE_W<CR1_FIFO_DISABLED_SPEC> {
        PEIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<CR1_FIFO_DISABLED_SPEC> {
        PS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PCE_W<CR1_FIFO_DISABLED_SPEC> {
        PCE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Receiver wakeup method This bit determines the USART wakeup method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WAKE_W<CR1_FIFO_DISABLED_SPEC> {
        WAKE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    #[must_use]
    pub fn m0(&mut self) -> M0_W<CR1_FIFO_DISABLED_SPEC> {
        M0_W::new(self, 12)
    }
    #[doc = "Bit 13 - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn mme(&mut self) -> MME_W<CR1_FIFO_DISABLED_SPEC> {
        MME_W::new(self, 13)
    }
    #[doc = "Bit 14 - Character match interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn cmie(&mut self) -> CMIE_W<CR1_FIFO_DISABLED_SPEC> {
        CMIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Oversampling mode This bit can only be written when the USART is disabled (UE=0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared."]
    #[inline(always)]
    #[must_use]
    pub fn over8(&mut self) -> OVER8_W<CR1_FIFO_DISABLED_SPEC> {
        OVER8_W::new(self, 15)
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn dedt(&mut self) -> DEDT_W<CR1_FIFO_DISABLED_SPEC> {
        DEDT_W::new(self, 16)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn deat(&mut self) -> DEAT_W<CR1_FIFO_DISABLED_SPEC> {
        DEAT_W::new(self, 21)
    }
    #[doc = "Bit 26 - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. ."]
    #[inline(always)]
    #[must_use]
    pub fn rtoie(&mut self) -> RTOIE_W<CR1_FIFO_DISABLED_SPEC> {
        RTOIE_W::new(self, 26)
    }
    #[doc = "Bit 27 - End of Block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn eobie(&mut self) -> EOBIE_W<CR1_FIFO_DISABLED_SPEC> {
        EOBIE_W::new(self, 27)
    }
    #[doc = "Bit 28 - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\\[1:0\\]
= '00': 1 start bit, 8 Data bits, n Stop bit M\\[1:0\\]
= '01': 1 start bit, 9 Data bits, n Stop bit M\\[1:0\\]
= '10': 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE=0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and Auto baud rate (0x7F and 0x55 frames detection) are not supported."]
    #[inline(always)]
    #[must_use]
    pub fn m1(&mut self) -> M1_W<CR1_FIFO_DISABLED_SPEC> {
        M1_W::new(self, 28)
    }
    #[doc = "Bit 29 - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE=0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes."]
    #[inline(always)]
    #[must_use]
    pub fn fifoen(&mut self) -> FIFOEN_W<CR1_FIFO_DISABLED_SPEC> {
        FIFOEN_W::new(self, 29)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1_fifo_disabled::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1_fifo_disabled::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_FIFO_DISABLED_SPEC;
impl crate::RegisterSpec for CR1_FIFO_DISABLED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1_fifo_disabled::R`](R) reader structure"]
impl crate::Readable for CR1_FIFO_DISABLED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1_fifo_disabled::W`](W) writer structure"]
impl crate::Writable for CR1_FIFO_DISABLED_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1_FIFO_DISABLED to value 0"]
impl crate::Resettable for CR1_FIFO_DISABLED_SPEC {
    const RESET_VALUE: u32 = 0;
}
