#[doc = "Register `SPI_SR` reader"]
pub type R = crate::R<SPI_SR_SPEC>;
#[doc = "Register `SPI_SR` writer"]
pub type W = crate::W<SPI_SR_SPEC>;
#[doc = "Field `RXNE` reader - Receive buffer not empty"]
pub type RXNE_R = crate::BitReader<RXNE_A>;
#[doc = "Receive buffer not empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNE_A {
    #[doc = "0: Rx buffer empty"]
    B_0X0 = 0,
    #[doc = "1: Rx buffer not empty"]
    B_0X1 = 1,
}
impl From<RXNE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXNE_A {
        match self.bits {
            false => RXNE_A::B_0X0,
            true => RXNE_A::B_0X1,
        }
    }
    #[doc = "Rx buffer empty"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXNE_A::B_0X0
    }
    #[doc = "Rx buffer not empty"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXNE_A::B_0X1
    }
}
#[doc = "Field `TXE` reader - Transmit buffer empty"]
pub type TXE_R = crate::BitReader<TXE_A>;
#[doc = "Transmit buffer empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXE_A {
    #[doc = "0: Tx buffer not empty"]
    B_0X0 = 0,
    #[doc = "1: Tx buffer empty"]
    B_0X1 = 1,
}
impl From<TXE_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXE_A {
        match self.bits {
            false => TXE_A::B_0X0,
            true => TXE_A::B_0X1,
        }
    }
    #[doc = "Tx buffer not empty"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXE_A::B_0X0
    }
    #[doc = "Tx buffer empty"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXE_A::B_0X1
    }
}
#[doc = "Field `CHSIDE` reader - Channel side Note: This bit is not used in SPI mode. It has no significance in PCM mode."]
pub type CHSIDE_R = crate::BitReader<CHSIDE_A>;
#[doc = "Channel side Note: This bit is not used in SPI mode. It has no significance in PCM mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSIDE_A {
    #[doc = "0: Channel Left has to be transmitted or has been received"]
    B_0X0 = 0,
    #[doc = "1: Channel Right has to be transmitted or has been received"]
    B_0X1 = 1,
}
impl From<CHSIDE_A> for bool {
    #[inline(always)]
    fn from(variant: CHSIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSIDE_A {
        match self.bits {
            false => CHSIDE_A::B_0X0,
            true => CHSIDE_A::B_0X1,
        }
    }
    #[doc = "Channel Left has to be transmitted or has been received"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSIDE_A::B_0X0
    }
    #[doc = "Channel Right has to be transmitted or has been received"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSIDE_A::B_0X1
    }
}
#[doc = "Field `UDR` reader - Underrun flag This flag is set by hardware and reset by a software sequence. Refer to page1056 for the software sequence. Note: This bit is not used in SPI mode."]
pub type UDR_R = crate::BitReader<UDR_A>;
#[doc = "Underrun flag This flag is set by hardware and reset by a software sequence. Refer to page1056 for the software sequence. Note: This bit is not used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDR_A {
    #[doc = "0: No underrun occurred"]
    B_0X0 = 0,
    #[doc = "1: Underrun occurred"]
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
    #[doc = "No underrun occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UDR_A::B_0X0
    }
    #[doc = "Underrun occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UDR_A::B_0X1
    }
}
#[doc = "Field `CRCERR` reader - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode."]
pub type CRCERR_R = crate::BitReader<CRCERR_A>;
#[doc = "CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCERR_A {
    #[doc = "0: CRC value received matches the SPIx_RXCRCR value"]
    B_0X0 = 0,
    #[doc = "1: CRC value received does not match the SPIx_RXCRCR value"]
    B_0X1 = 1,
}
impl From<CRCERR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERR_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCERR_A {
        match self.bits {
            false => CRCERR_A::B_0X0,
            true => CRCERR_A::B_0X1,
        }
    }
    #[doc = "CRC value received matches the SPIx_RXCRCR value"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRCERR_A::B_0X0
    }
    #[doc = "CRC value received does not match the SPIx_RXCRCR value"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRCERR_A::B_0X1
    }
}
#[doc = "Field `CRCERR` writer - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode."]
pub type CRCERR_W<'a, REG> = crate::BitWriter<'a, REG, CRCERR_A>;
impl<'a, REG> CRCERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC value received matches the SPIx_RXCRCR value"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCERR_A::B_0X0)
    }
    #[doc = "CRC value received does not match the SPIx_RXCRCR value"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCERR_A::B_0X1)
    }
}
#[doc = "Field `MODF` reader - Mode fault This flag is set by hardware and reset by a software sequence. Refer to (MODF) on page1030 for the software sequence. Note: This bit is not used in I2S mode."]
pub type MODF_R = crate::BitReader<MODF_A>;
#[doc = "Mode fault This flag is set by hardware and reset by a software sequence. Refer to (MODF) on page1030 for the software sequence. Note: This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODF_A {
    #[doc = "0: No mode fault occurred"]
    B_0X0 = 0,
    #[doc = "1: Mode fault occurred"]
    B_0X1 = 1,
}
impl From<MODF_A> for bool {
    #[inline(always)]
    fn from(variant: MODF_A) -> Self {
        variant as u8 != 0
    }
}
impl MODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODF_A {
        match self.bits {
            false => MODF_A::B_0X0,
            true => MODF_A::B_0X1,
        }
    }
    #[doc = "No mode fault occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MODF_A::B_0X0
    }
    #[doc = "Mode fault occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MODF_A::B_0X1
    }
}
#[doc = "Field `OVR` reader - Overrun flag This flag is set by hardware and reset by a software sequence. Refer to page1056 for the software sequence."]
pub type OVR_R = crate::BitReader<OVR_A>;
#[doc = "Overrun flag This flag is set by hardware and reset by a software sequence. Refer to page1056 for the software sequence.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_A {
    #[doc = "0: No overrun occurred"]
    B_0X0 = 0,
    #[doc = "1: Overrun occurred"]
    B_0X1 = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::B_0X0,
            true => OVR_A::B_0X1,
        }
    }
    #[doc = "No overrun occurred"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OVR_A::B_0X0
    }
    #[doc = "Overrun occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OVR_A::B_0X1
    }
}
#[doc = "Field `BSY` reader - Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to and ."]
pub type BSY_R = crate::BitReader<BSY_A>;
#[doc = "Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to and .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSY_A {
    #[doc = "0: SPI (or I2S) not busy"]
    B_0X0 = 0,
    #[doc = "1: SPI (or I2S) is busy in communication or Tx buffer is not empty"]
    B_0X1 = 1,
}
impl From<BSY_A> for bool {
    #[inline(always)]
    fn from(variant: BSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BSY_A {
        match self.bits {
            false => BSY_A::B_0X0,
            true => BSY_A::B_0X1,
        }
    }
    #[doc = "SPI (or I2S) not busy"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BSY_A::B_0X0
    }
    #[doc = "SPI (or I2S) is busy in communication or Tx buffer is not empty"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BSY_A::B_0X1
    }
}
#[doc = "Field `FRE` reader - Frame format error This flag is used for SPI in TI slave mode and I2S slave mode. Refer to error flags and . This flag is set by hardware and reset when SPIx_SR is read by software."]
pub type FRE_R = crate::BitReader<FRE_A>;
#[doc = "Frame format error This flag is used for SPI in TI slave mode and I2S slave mode. Refer to error flags and . This flag is set by hardware and reset when SPIx_SR is read by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRE_A {
    #[doc = "0: No frame format error"]
    B_0X0 = 0,
    #[doc = "1: A frame format error occurred"]
    B_0X1 = 1,
}
impl From<FRE_A> for bool {
    #[inline(always)]
    fn from(variant: FRE_A) -> Self {
        variant as u8 != 0
    }
}
impl FRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FRE_A {
        match self.bits {
            false => FRE_A::B_0X0,
            true => FRE_A::B_0X1,
        }
    }
    #[doc = "No frame format error"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FRE_A::B_0X0
    }
    #[doc = "A frame format error occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FRE_A::B_0X1
    }
}
#[doc = "Field `FRLVL` reader - FIFO reception level"]
pub type FRLVL_R = crate::FieldReader<FRLVL_A>;
#[doc = "FIFO reception level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRLVL_A {
    #[doc = "0: FIFO empty"]
    B_0X0 = 0,
    #[doc = "1: 1/4 FIFO"]
    B_0X1 = 1,
    #[doc = "2: 1/2 FIFO"]
    B_0X2 = 2,
    #[doc = "3: FIFO full"]
    B_0X3 = 3,
}
impl From<FRLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: FRLVL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FRLVL_A {
    type Ux = u8;
}
impl FRLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FRLVL_A {
        match self.bits {
            0 => FRLVL_A::B_0X0,
            1 => FRLVL_A::B_0X1,
            2 => FRLVL_A::B_0X2,
            3 => FRLVL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "FIFO empty"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FRLVL_A::B_0X0
    }
    #[doc = "1/4 FIFO"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FRLVL_A::B_0X1
    }
    #[doc = "1/2 FIFO"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == FRLVL_A::B_0X2
    }
    #[doc = "FIFO full"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == FRLVL_A::B_0X3
    }
}
#[doc = "Field `FTLVL` reader - FIFO transmission level These bits are set and cleared by hardware. Note: This bit is not used in I2S mode."]
pub type FTLVL_R = crate::FieldReader<FTLVL_A>;
#[doc = "FIFO transmission level These bits are set and cleared by hardware. Note: This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FTLVL_A {
    #[doc = "0: FIFO empty"]
    B_0X0 = 0,
    #[doc = "1: 1/4 FIFO"]
    B_0X1 = 1,
    #[doc = "2: 1/2 FIFO"]
    B_0X2 = 2,
    #[doc = "3: FIFO full (considered as FULL when the FIFO threshold is greater than 1/2)"]
    B_0X3 = 3,
}
impl From<FTLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTLVL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FTLVL_A {
    type Ux = u8;
}
impl FTLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FTLVL_A {
        match self.bits {
            0 => FTLVL_A::B_0X0,
            1 => FTLVL_A::B_0X1,
            2 => FTLVL_A::B_0X2,
            3 => FTLVL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "FIFO empty"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FTLVL_A::B_0X0
    }
    #[doc = "1/4 FIFO"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FTLVL_A::B_0X1
    }
    #[doc = "1/2 FIFO"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == FTLVL_A::B_0X2
    }
    #[doc = "FIFO full (considered as FULL when the FIFO threshold is greater than 1/2)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == FTLVL_A::B_0X3
    }
}
impl R {
    #[doc = "Bit 0 - Receive buffer not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel side Note: This bit is not used in SPI mode. It has no significance in PCM mode."]
    #[inline(always)]
    pub fn chside(&self) -> CHSIDE_R {
        CHSIDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Underrun flag This flag is set by hardware and reset by a software sequence. Refer to page1056 for the software sequence. Note: This bit is not used in SPI mode."]
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode fault This flag is set by hardware and reset by a software sequence. Refer to (MODF) on page1030 for the software sequence. Note: This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun flag This flag is set by hardware and reset by a software sequence. Refer to page1056 for the software sequence."]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to and ."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Frame format error This flag is used for SPI in TI slave mode and I2S slave mode. Refer to error flags and . This flag is set by hardware and reset when SPIx_SR is read by software."]
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - FIFO reception level"]
    #[inline(always)]
    pub fn frlvl(&self) -> FRLVL_R {
        FRLVL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - FIFO transmission level These bits are set and cleared by hardware. Note: This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn ftlvl(&self) -> FTLVL_R {
        FTLVL_R::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn crcerr(&mut self) -> CRCERR_W<SPI_SR_SPEC> {
        CRCERR_W::new(self, 4)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SR_SPEC;
impl crate::RegisterSpec for SPI_SR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spi_sr::R`](R) reader structure"]
impl crate::Readable for SPI_SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_sr::W`](W) writer structure"]
impl crate::Writable for SPI_SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SPI_SR to value 0x02"]
impl crate::Resettable for SPI_SR_SPEC {
    const RESET_VALUE: u16 = 0x02;
}
