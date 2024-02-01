#[doc = "Register `SPI_CR1` reader"]
pub type R = crate::R<SPI_CR1_SPEC>;
#[doc = "Register `SPI_CR1` writer"]
pub type W = crate::W<SPI_CR1_SPEC>;
#[doc = "Field `CPHA` reader - Clock phase Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode."]
pub type CPHA_R = crate::BitReader<CPHA_A>;
#[doc = "Clock phase Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA_A {
    #[doc = "0: The first clock transition is the first data capture edge"]
    B_0X0 = 0,
    #[doc = "1: The second clock transition is the first data capture edge"]
    B_0X1 = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::B_0X0,
            true => CPHA_A::B_0X1,
        }
    }
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CPHA_A::B_0X0
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CPHA_A::B_0X1
    }
}
#[doc = "Field `CPHA` writer - Clock phase Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode."]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG, CPHA_A>;
impl<'a, REG> CPHA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA_A::B_0X0)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA_A::B_0X1)
    }
}
#[doc = "Field `CPOL` reader - Clock polarity Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode."]
pub type CPOL_R = crate::BitReader<CPOL_A>;
#[doc = "Clock polarity Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    #[doc = "0: CK to 0 when idle"]
    B_0X0 = 0,
    #[doc = "1: CK to 1 when idle"]
    B_0X1 = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::B_0X0,
            true => CPOL_A::B_0X1,
        }
    }
    #[doc = "CK to 0 when idle"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CPOL_A::B_0X0
    }
    #[doc = "CK to 1 when idle"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CPOL_A::B_0X1
    }
}
#[doc = "Field `CPOL` writer - Clock polarity Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode."]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG, CPOL_A>;
impl<'a, REG> CPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CK to 0 when idle"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL_A::B_0X0)
    }
    #[doc = "CK to 1 when idle"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL_A::B_0X1)
    }
}
#[doc = "Field `MSTR` reader - Master selection Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode."]
pub type MSTR_R = crate::BitReader<MSTR_A>;
#[doc = "Master selection Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTR_A {
    #[doc = "0: Slave configuration"]
    B_0X0 = 0,
    #[doc = "1: Master configuration"]
    B_0X1 = 1,
}
impl From<MSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTR_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTR_A {
        match self.bits {
            false => MSTR_A::B_0X0,
            true => MSTR_A::B_0X1,
        }
    }
    #[doc = "Slave configuration"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MSTR_A::B_0X0
    }
    #[doc = "Master configuration"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MSTR_A::B_0X1
    }
}
#[doc = "Field `MSTR` writer - Master selection Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode."]
pub type MSTR_W<'a, REG> = crate::BitWriter<'a, REG, MSTR_A>;
impl<'a, REG> MSTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave configuration"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTR_A::B_0X0)
    }
    #[doc = "Master configuration"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTR_A::B_0X1)
    }
}
#[doc = "Field `BR` reader - Baud rate control Note: These bits should not be changed when communication is ongoing. These bits are not used in I2S mode."]
pub type BR_R = crate::FieldReader<BR_A>;
#[doc = "Baud rate control Note: These bits should not be changed when communication is ongoing. These bits are not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BR_A {
    #[doc = "0: fPCLK/2"]
    B_0X0 = 0,
    #[doc = "1: fPCLK/4"]
    B_0X1 = 1,
    #[doc = "2: fPCLK/8"]
    B_0X2 = 2,
    #[doc = "3: fPCLK/16"]
    B_0X3 = 3,
    #[doc = "4: fPCLK/32"]
    B_0X4 = 4,
    #[doc = "5: fPCLK/64"]
    B_0X5 = 5,
    #[doc = "6: fPCLK/128"]
    B_0X6 = 6,
    #[doc = "7: fPCLK/256"]
    B_0X7 = 7,
}
impl From<BR_A> for u8 {
    #[inline(always)]
    fn from(variant: BR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BR_A {
    type Ux = u8;
}
impl BR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BR_A {
        match self.bits {
            0 => BR_A::B_0X0,
            1 => BR_A::B_0X1,
            2 => BR_A::B_0X2,
            3 => BR_A::B_0X3,
            4 => BR_A::B_0X4,
            5 => BR_A::B_0X5,
            6 => BR_A::B_0X6,
            7 => BR_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "fPCLK/2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BR_A::B_0X0
    }
    #[doc = "fPCLK/4"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BR_A::B_0X1
    }
    #[doc = "fPCLK/8"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == BR_A::B_0X2
    }
    #[doc = "fPCLK/16"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == BR_A::B_0X3
    }
    #[doc = "fPCLK/32"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == BR_A::B_0X4
    }
    #[doc = "fPCLK/64"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == BR_A::B_0X5
    }
    #[doc = "fPCLK/128"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == BR_A::B_0X6
    }
    #[doc = "fPCLK/256"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == BR_A::B_0X7
    }
}
#[doc = "Field `BR` writer - Baud rate control Note: These bits should not be changed when communication is ongoing. These bits are not used in I2S mode."]
pub type BR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, BR_A>;
impl<'a, REG> BR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fPCLK/2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR_A::B_0X0)
    }
    #[doc = "fPCLK/4"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR_A::B_0X1)
    }
    #[doc = "fPCLK/8"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(BR_A::B_0X2)
    }
    #[doc = "fPCLK/16"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(BR_A::B_0X3)
    }
    #[doc = "fPCLK/32"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(BR_A::B_0X4)
    }
    #[doc = "fPCLK/64"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(BR_A::B_0X5)
    }
    #[doc = "fPCLK/128"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(BR_A::B_0X6)
    }
    #[doc = "fPCLK/256"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(BR_A::B_0X7)
    }
}
#[doc = "Field `SPE` reader - SPI enable Note: When disabling the SPI, follow the procedure described in SPI on page1020. This bit is not used in I2S mode."]
pub type SPE_R = crate::BitReader<SPE_A>;
#[doc = "SPI enable Note: When disabling the SPI, follow the procedure described in SPI on page1020. This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPE_A {
    #[doc = "0: Peripheral disabled"]
    B_0X0 = 0,
    #[doc = "1: Peripheral enabled"]
    B_0X1 = 1,
}
impl From<SPE_A> for bool {
    #[inline(always)]
    fn from(variant: SPE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPE_A {
        match self.bits {
            false => SPE_A::B_0X0,
            true => SPE_A::B_0X1,
        }
    }
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPE_A::B_0X0
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPE_A::B_0X1
    }
}
#[doc = "Field `SPE` writer - SPI enable Note: When disabling the SPI, follow the procedure described in SPI on page1020. This bit is not used in I2S mode."]
pub type SPE_W<'a, REG> = crate::BitWriter<'a, REG, SPE_A>;
impl<'a, REG> SPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPE_A::B_0X0)
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPE_A::B_0X1)
    }
}
#[doc = "Field `LSBFIRST` reader - Frame format Note: 1. This bit should not be changed when communication is ongoing. 2. This bit is not used in I2S mode and SPI TI mode."]
pub type LSBFIRST_R = crate::BitReader<LSBFIRST_A>;
#[doc = "Frame format Note: 1. This bit should not be changed when communication is ongoing. 2. This bit is not used in I2S mode and SPI TI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSBFIRST_A {
    #[doc = "0: data is transmitted / received with the MSB first"]
    B_0X0 = 0,
    #[doc = "1: data is transmitted / received with the LSB first"]
    B_0X1 = 1,
}
impl From<LSBFIRST_A> for bool {
    #[inline(always)]
    fn from(variant: LSBFIRST_A) -> Self {
        variant as u8 != 0
    }
}
impl LSBFIRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSBFIRST_A {
        match self.bits {
            false => LSBFIRST_A::B_0X0,
            true => LSBFIRST_A::B_0X1,
        }
    }
    #[doc = "data is transmitted / received with the MSB first"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSBFIRST_A::B_0X0
    }
    #[doc = "data is transmitted / received with the LSB first"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSBFIRST_A::B_0X1
    }
}
#[doc = "Field `LSBFIRST` writer - Frame format Note: 1. This bit should not be changed when communication is ongoing. 2. This bit is not used in I2S mode and SPI TI mode."]
pub type LSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG, LSBFIRST_A>;
impl<'a, REG> LSBFIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "data is transmitted / received with the MSB first"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSBFIRST_A::B_0X0)
    }
    #[doc = "data is transmitted / received with the LSB first"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSBFIRST_A::B_0X1)
    }
}
#[doc = "Field `SSI` reader - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored. Note: This bit is not used in I2S mode and SPI TI mode."]
pub type SSI_R = crate::BitReader;
#[doc = "Field `SSI` writer - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored. Note: This bit is not used in I2S mode and SPI TI mode."]
pub type SSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSM` reader - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I2S mode and SPI TI mode."]
pub type SSM_R = crate::BitReader<SSM_A>;
#[doc = "Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I2S mode and SPI TI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSM_A {
    #[doc = "0: Software slave management disabled"]
    B_0X0 = 0,
    #[doc = "1: Software slave management enabled"]
    B_0X1 = 1,
}
impl From<SSM_A> for bool {
    #[inline(always)]
    fn from(variant: SSM_A) -> Self {
        variant as u8 != 0
    }
}
impl SSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSM_A {
        match self.bits {
            false => SSM_A::B_0X0,
            true => SSM_A::B_0X1,
        }
    }
    #[doc = "Software slave management disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SSM_A::B_0X0
    }
    #[doc = "Software slave management enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SSM_A::B_0X1
    }
}
#[doc = "Field `SSM` writer - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I2S mode and SPI TI mode."]
pub type SSM_W<'a, REG> = crate::BitWriter<'a, REG, SSM_A>;
impl<'a, REG> SSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software slave management disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SSM_A::B_0X0)
    }
    #[doc = "Software slave management enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SSM_A::B_0X1)
    }
}
#[doc = "Field `RXONLY` reader - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I2S mode."]
pub type RXONLY_R = crate::BitReader<RXONLY_A>;
#[doc = "Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXONLY_A {
    #[doc = "0: Full-duplex (Transmit and receive)"]
    B_0X0 = 0,
    #[doc = "1: Output disabled (Receive-only mode)"]
    B_0X1 = 1,
}
impl From<RXONLY_A> for bool {
    #[inline(always)]
    fn from(variant: RXONLY_A) -> Self {
        variant as u8 != 0
    }
}
impl RXONLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXONLY_A {
        match self.bits {
            false => RXONLY_A::B_0X0,
            true => RXONLY_A::B_0X1,
        }
    }
    #[doc = "Full-duplex (Transmit and receive)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXONLY_A::B_0X0
    }
    #[doc = "Output disabled (Receive-only mode)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXONLY_A::B_0X1
    }
}
#[doc = "Field `RXONLY` writer - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I2S mode."]
pub type RXONLY_W<'a, REG> = crate::BitWriter<'a, REG, RXONLY_A>;
impl<'a, REG> RXONLY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Full-duplex (Transmit and receive)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXONLY_A::B_0X0)
    }
    #[doc = "Output disabled (Receive-only mode)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXONLY_A::B_0X1)
    }
}
#[doc = "Field `CRCL` reader - CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode."]
pub type CRCL_R = crate::BitReader<CRCL_A>;
#[doc = "CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCL_A {
    #[doc = "0: 8-bit CRC length"]
    B_0X0 = 0,
    #[doc = "1: 16-bit CRC length"]
    B_0X1 = 1,
}
impl From<CRCL_A> for bool {
    #[inline(always)]
    fn from(variant: CRCL_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCL_A {
        match self.bits {
            false => CRCL_A::B_0X0,
            true => CRCL_A::B_0X1,
        }
    }
    #[doc = "8-bit CRC length"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRCL_A::B_0X0
    }
    #[doc = "16-bit CRC length"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRCL_A::B_0X1
    }
}
#[doc = "Field `CRCL` writer - CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode."]
pub type CRCL_W<'a, REG> = crate::BitWriter<'a, REG, CRCL_A>;
impl<'a, REG> CRCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8-bit CRC length"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCL_A::B_0X0)
    }
    #[doc = "16-bit CRC length"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCL_A::B_0X1)
    }
}
#[doc = "Field `CRCNEXT` reader - Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPIx_DR register. This bit is not used in I2S mode."]
pub type CRCNEXT_R = crate::BitReader<CRCNEXT_A>;
#[doc = "Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPIx_DR register. This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCNEXT_A {
    #[doc = "0: Next transmit value is from Tx buffer."]
    B_0X0 = 0,
    #[doc = "1: Next transmit value is from Tx CRC register."]
    B_0X1 = 1,
}
impl From<CRCNEXT_A> for bool {
    #[inline(always)]
    fn from(variant: CRCNEXT_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCNEXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCNEXT_A {
        match self.bits {
            false => CRCNEXT_A::B_0X0,
            true => CRCNEXT_A::B_0X1,
        }
    }
    #[doc = "Next transmit value is from Tx buffer."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRCNEXT_A::B_0X0
    }
    #[doc = "Next transmit value is from Tx CRC register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRCNEXT_A::B_0X1
    }
}
#[doc = "Field `CRCNEXT` writer - Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPIx_DR register. This bit is not used in I2S mode."]
pub type CRCNEXT_W<'a, REG> = crate::BitWriter<'a, REG, CRCNEXT_A>;
impl<'a, REG> CRCNEXT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Next transmit value is from Tx buffer."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCNEXT_A::B_0X0)
    }
    #[doc = "Next transmit value is from Tx CRC register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCNEXT_A::B_0X1)
    }
}
#[doc = "Field `CRCEN` reader - Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode."]
pub type CRCEN_R = crate::BitReader<CRCEN_A>;
#[doc = "Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEN_A {
    #[doc = "0: CRC calculation disabled"]
    B_0X0 = 0,
    #[doc = "1: CRC calculation enabled"]
    B_0X1 = 1,
}
impl From<CRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCEN_A {
        match self.bits {
            false => CRCEN_A::B_0X0,
            true => CRCEN_A::B_0X1,
        }
    }
    #[doc = "CRC calculation disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRCEN_A::B_0X0
    }
    #[doc = "CRC calculation enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRCEN_A::B_0X1
    }
}
#[doc = "Field `CRCEN` writer - Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode."]
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG, CRCEN_A>;
impl<'a, REG> CRCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC calculation disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN_A::B_0X0)
    }
    #[doc = "CRC calculation enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN_A::B_0X1)
    }
}
#[doc = "Field `BIDIOE` reader - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. This bit is not used in I2S mode."]
pub type BIDIOE_R = crate::BitReader<BIDIOE_A>;
#[doc = "Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIDIOE_A {
    #[doc = "0: Output disabled (receive-only mode)"]
    B_0X0 = 0,
    #[doc = "1: Output enabled (transmit-only mode)"]
    B_0X1 = 1,
}
impl From<BIDIOE_A> for bool {
    #[inline(always)]
    fn from(variant: BIDIOE_A) -> Self {
        variant as u8 != 0
    }
}
impl BIDIOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BIDIOE_A {
        match self.bits {
            false => BIDIOE_A::B_0X0,
            true => BIDIOE_A::B_0X1,
        }
    }
    #[doc = "Output disabled (receive-only mode)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BIDIOE_A::B_0X0
    }
    #[doc = "Output enabled (transmit-only mode)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BIDIOE_A::B_0X1
    }
}
#[doc = "Field `BIDIOE` writer - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. This bit is not used in I2S mode."]
pub type BIDIOE_W<'a, REG> = crate::BitWriter<'a, REG, BIDIOE_A>;
impl<'a, REG> BIDIOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output disabled (receive-only mode)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BIDIOE_A::B_0X0)
    }
    #[doc = "Output enabled (transmit-only mode)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BIDIOE_A::B_0X1)
    }
}
#[doc = "Field `BIDIMODE` reader - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I2S mode."]
pub type BIDIMODE_R = crate::BitReader<BIDIMODE_A>;
#[doc = "Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIDIMODE_A {
    #[doc = "0: 2-line unidirectional data mode selected"]
    B_0X0 = 0,
    #[doc = "1: 1-line bidirectional data mode selected"]
    B_0X1 = 1,
}
impl From<BIDIMODE_A> for bool {
    #[inline(always)]
    fn from(variant: BIDIMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl BIDIMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BIDIMODE_A {
        match self.bits {
            false => BIDIMODE_A::B_0X0,
            true => BIDIMODE_A::B_0X1,
        }
    }
    #[doc = "2-line unidirectional data mode selected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BIDIMODE_A::B_0X0
    }
    #[doc = "1-line bidirectional data mode selected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BIDIMODE_A::B_0X1
    }
}
#[doc = "Field `BIDIMODE` writer - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I2S mode."]
pub type BIDIMODE_W<'a, REG> = crate::BitWriter<'a, REG, BIDIMODE_A>;
impl<'a, REG> BIDIMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "2-line unidirectional data mode selected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BIDIMODE_A::B_0X0)
    }
    #[doc = "1-line bidirectional data mode selected"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BIDIMODE_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - Clock phase Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode."]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock polarity Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode."]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master selection Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Baud rate control Note: These bits should not be changed when communication is ongoing. These bits are not used in I2S mode."]
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - SPI enable Note: When disabling the SPI, follow the procedure described in SPI on page1020. This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frame format Note: 1. This bit should not be changed when communication is ongoing. 2. This bit is not used in I2S mode and SPI TI mode."]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored. Note: This bit is not used in I2S mode and SPI TI mode."]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I2S mode and SPI TI mode."]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn rxonly(&self) -> RXONLY_R {
        RXONLY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn crcl(&self) -> CRCL_R {
        CRCL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPIx_DR register. This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn crcnext(&self) -> CRCNEXT_R {
        CRCNEXT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn bidioe(&self) -> BIDIOE_R {
        BIDIOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn bidimode(&self) -> BIDIMODE_R {
        BIDIMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock phase Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode."]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<SPI_CR1_SPEC> {
        CPHA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clock polarity Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode and SPI TI mode except the case when CRC is applied at TI mode."]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<SPI_CR1_SPEC> {
        CPOL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Master selection Note: This bit should not be changed when communication is ongoing. This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn mstr(&mut self) -> MSTR_W<SPI_CR1_SPEC> {
        MSTR_W::new(self, 2)
    }
    #[doc = "Bits 3:5 - Baud rate control Note: These bits should not be changed when communication is ongoing. These bits are not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn br(&mut self) -> BR_W<SPI_CR1_SPEC> {
        BR_W::new(self, 3)
    }
    #[doc = "Bit 6 - SPI enable Note: When disabling the SPI, follow the procedure described in SPI on page1020. This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SPE_W<SPI_CR1_SPEC> {
        SPE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Frame format Note: 1. This bit should not be changed when communication is ongoing. 2. This bit is not used in I2S mode and SPI TI mode."]
    #[inline(always)]
    #[must_use]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W<SPI_CR1_SPEC> {
        LSBFIRST_W::new(self, 7)
    }
    #[doc = "Bit 8 - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored. Note: This bit is not used in I2S mode and SPI TI mode."]
    #[inline(always)]
    #[must_use]
    pub fn ssi(&mut self) -> SSI_W<SPI_CR1_SPEC> {
        SSI_W::new(self, 8)
    }
    #[doc = "Bit 9 - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I2S mode and SPI TI mode."]
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<SPI_CR1_SPEC> {
        SSM_W::new(self, 9)
    }
    #[doc = "Bit 10 - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn rxonly(&mut self) -> RXONLY_W<SPI_CR1_SPEC> {
        RXONLY_W::new(self, 10)
    }
    #[doc = "Bit 11 - CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn crcl(&mut self) -> CRCL_W<SPI_CR1_SPEC> {
        CRCL_W::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPIx_DR register. This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn crcnext(&mut self) -> CRCNEXT_W<SPI_CR1_SPEC> {
        CRCNEXT_W::new(self, 12)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = '0') for correct operation. This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<SPI_CR1_SPEC> {
        CRCEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn bidioe(&mut self) -> BIDIOE_W<SPI_CR1_SPEC> {
        BIDIOE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn bidimode(&mut self) -> BIDIMODE_W<SPI_CR1_SPEC> {
        BIDIMODE_W::new(self, 15)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_CR1_SPEC;
impl crate::RegisterSpec for SPI_CR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spi_cr1::R`](R) reader structure"]
impl crate::Readable for SPI_CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_cr1::W`](W) writer structure"]
impl crate::Writable for SPI_CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SPI_CR1 to value 0"]
impl crate::Resettable for SPI_CR1_SPEC {
    const RESET_VALUE: u16 = 0;
}
