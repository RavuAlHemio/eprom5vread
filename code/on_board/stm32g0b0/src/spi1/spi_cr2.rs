#[doc = "Register `SPI_CR2` reader"]
pub type R = crate::R<SPI_CR2_SPEC>;
#[doc = "Register `SPI_CR2` writer"]
pub type W = crate::W<SPI_CR2_SPEC>;
#[doc = "Field `RXDMAEN` reader - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set."]
pub type RXDMAEN_R = crate::BitReader<RXDMAEN_A>;
#[doc = "Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN_A {
    #[doc = "0: Rx buffer DMA disabled"]
    B_0X0 = 0,
    #[doc = "1: Rx buffer DMA enabled"]
    B_0X1 = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::B_0X0,
            true => RXDMAEN_A::B_0X1,
        }
    }
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXDMAEN_A::B_0X0
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXDMAEN_A::B_0X1
    }
}
#[doc = "Field `RXDMAEN` writer - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set."]
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, RXDMAEN_A>;
impl<'a, REG> RXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN_A::B_0X0)
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN_A::B_0X1)
    }
}
#[doc = "Field `TXDMAEN` reader - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set."]
pub type TXDMAEN_R = crate::BitReader<TXDMAEN_A>;
#[doc = "Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN_A {
    #[doc = "0: Tx buffer DMA disabled"]
    B_0X0 = 0,
    #[doc = "1: Tx buffer DMA enabled"]
    B_0X1 = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::B_0X0,
            true => TXDMAEN_A::B_0X1,
        }
    }
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXDMAEN_A::B_0X0
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXDMAEN_A::B_0X1
    }
}
#[doc = "Field `TXDMAEN` writer - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set."]
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, TXDMAEN_A>;
impl<'a, REG> TXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN_A::B_0X0)
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN_A::B_0X1)
    }
}
#[doc = "Field `SSOE` reader - SS output enable Note: This bit is not used in I2S mode and SPI TI mode."]
pub type SSOE_R = crate::BitReader<SSOE_A>;
#[doc = "SS output enable Note: This bit is not used in I2S mode and SPI TI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSOE_A {
    #[doc = "0: SS output is disabled in master mode and the SPI interface can work in multimaster configuration"]
    B_0X0 = 0,
    #[doc = "1: SS output is enabled in master mode and when the SPI interface is enabled. The SPI interface cannot work in a multimaster environment."]
    B_0X1 = 1,
}
impl From<SSOE_A> for bool {
    #[inline(always)]
    fn from(variant: SSOE_A) -> Self {
        variant as u8 != 0
    }
}
impl SSOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSOE_A {
        match self.bits {
            false => SSOE_A::B_0X0,
            true => SSOE_A::B_0X1,
        }
    }
    #[doc = "SS output is disabled in master mode and the SPI interface can work in multimaster configuration"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SSOE_A::B_0X0
    }
    #[doc = "SS output is enabled in master mode and when the SPI interface is enabled. The SPI interface cannot work in a multimaster environment."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SSOE_A::B_0X1
    }
}
#[doc = "Field `SSOE` writer - SS output enable Note: This bit is not used in I2S mode and SPI TI mode."]
pub type SSOE_W<'a, REG> = crate::BitWriter<'a, REG, SSOE_A>;
impl<'a, REG> SSOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SS output is disabled in master mode and the SPI interface can work in multimaster configuration"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SSOE_A::B_0X0)
    }
    #[doc = "SS output is enabled in master mode and when the SPI interface is enabled. The SPI interface cannot work in a multimaster environment."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SSOE_A::B_0X1)
    }
}
#[doc = "Field `NSSP` reader - NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = '1', or FRF = '1'. Note: 1. This bit must be written only when the SPI is disabled (SPE=0). 2. This bit is not used in I2S mode and SPI TI mode."]
pub type NSSP_R = crate::BitReader<NSSP_A>;
#[doc = "NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = '1', or FRF = '1'. Note: 1. This bit must be written only when the SPI is disabled (SPE=0). 2. This bit is not used in I2S mode and SPI TI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSSP_A {
    #[doc = "0: No NSS pulse"]
    B_0X0 = 0,
    #[doc = "1: NSS pulse generated"]
    B_0X1 = 1,
}
impl From<NSSP_A> for bool {
    #[inline(always)]
    fn from(variant: NSSP_A) -> Self {
        variant as u8 != 0
    }
}
impl NSSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NSSP_A {
        match self.bits {
            false => NSSP_A::B_0X0,
            true => NSSP_A::B_0X1,
        }
    }
    #[doc = "No NSS pulse"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == NSSP_A::B_0X0
    }
    #[doc = "NSS pulse generated"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NSSP_A::B_0X1
    }
}
#[doc = "Field `NSSP` writer - NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = '1', or FRF = '1'. Note: 1. This bit must be written only when the SPI is disabled (SPE=0). 2. This bit is not used in I2S mode and SPI TI mode."]
pub type NSSP_W<'a, REG> = crate::BitWriter<'a, REG, NSSP_A>;
impl<'a, REG> NSSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No NSS pulse"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NSSP_A::B_0X0)
    }
    #[doc = "NSS pulse generated"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NSSP_A::B_0X1)
    }
}
#[doc = "Field `FRF` reader - Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0). This bit is not used in I2S mode."]
pub type FRF_R = crate::BitReader<FRF_A>;
#[doc = "Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0). This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRF_A {
    #[doc = "0: SPI Motorola mode"]
    B_0X0 = 0,
}
impl From<FRF_A> for bool {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        variant as u8 != 0
    }
}
impl FRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FRF_A> {
        match self.bits {
            false => Some(FRF_A::B_0X0),
            _ => None,
        }
    }
    #[doc = "SPI Motorola mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FRF_A::B_0X0
    }
}
#[doc = "Field `FRF` writer - Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0). This bit is not used in I2S mode."]
pub type FRF_W<'a, REG> = crate::BitWriter<'a, REG, FRF_A>;
impl<'a, REG> FRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI Motorola mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FRF_A::B_0X0)
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I2S mode)."]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
#[doc = "Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I2S mode).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE_A {
    #[doc = "0: Error interrupt is masked"]
    B_0X0 = 0,
    #[doc = "1: Error interrupt is enabled"]
    B_0X1 = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::B_0X0,
            true => ERRIE_A::B_0X1,
        }
    }
    #[doc = "Error interrupt is masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ERRIE_A::B_0X0
    }
    #[doc = "Error interrupt is enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ERRIE_A::B_0X1
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I2S mode)."]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE_A>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt is masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE_A::B_0X0)
    }
    #[doc = "Error interrupt is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE_A::B_0X1)
    }
}
#[doc = "Field `RXNEIE` reader - RX buffer not empty interrupt enable"]
pub type RXNEIE_R = crate::BitReader<RXNEIE_A>;
#[doc = "RX buffer not empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNEIE_A {
    #[doc = "0: RXNE interrupt masked"]
    B_0X0 = 0,
    #[doc = "1: RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
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
    #[doc = "RXNE interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXNEIE_A::B_0X0
    }
    #[doc = "RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXNEIE_A::B_0X1
    }
}
#[doc = "Field `RXNEIE` writer - RX buffer not empty interrupt enable"]
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG, RXNEIE_A>;
impl<'a, REG> RXNEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RXNE interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE_A::B_0X0)
    }
    #[doc = "RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE_A::B_0X1)
    }
}
#[doc = "Field `TXEIE` reader - Tx buffer empty interrupt enable"]
pub type TXEIE_R = crate::BitReader<TXEIE_A>;
#[doc = "Tx buffer empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEIE_A {
    #[doc = "0: TXE interrupt masked"]
    B_0X0 = 0,
    #[doc = "1: TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
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
    #[doc = "TXE interrupt masked"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXEIE_A::B_0X0
    }
    #[doc = "TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXEIE_A::B_0X1
    }
}
#[doc = "Field `TXEIE` writer - Tx buffer empty interrupt enable"]
pub type TXEIE_W<'a, REG> = crate::BitWriter<'a, REG, TXEIE_A>;
impl<'a, REG> TXEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TXE interrupt masked"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE_A::B_0X0)
    }
    #[doc = "TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE_A::B_0X1)
    }
}
#[doc = "Field `DS` reader - Data size These bits configure the data length for SPI transfers. If software attempts to write one of the 'Not used' values, they are forced to the value '0111' (8-bit) Note: These bits are not used in I2S mode."]
pub type DS_R = crate::FieldReader<DS_A>;
#[doc = "Data size These bits configure the data length for SPI transfers. If software attempts to write one of the 'Not used' values, they are forced to the value '0111' (8-bit) Note: These bits are not used in I2S mode.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DS_A {
    #[doc = "0: Not used"]
    B_0X0 = 0,
    #[doc = "1: Not used"]
    B_0X1 = 1,
    #[doc = "2: Not used"]
    B_0X2 = 2,
    #[doc = "3: 4-bit"]
    B_0X3 = 3,
    #[doc = "4: 5-bit"]
    B_0X4 = 4,
    #[doc = "5: 6-bit"]
    B_0X5 = 5,
    #[doc = "6: 7-bit"]
    B_0X6 = 6,
    #[doc = "7: 8-bit"]
    B_0X7 = 7,
    #[doc = "8: 9-bit"]
    B_0X8 = 8,
    #[doc = "9: 10-bit"]
    B_0X9 = 9,
    #[doc = "10: 11-bit"]
    B_0X_A = 10,
    #[doc = "11: 12-bit"]
    B_0X_B = 11,
    #[doc = "12: 13-bit"]
    B_0X_C = 12,
    #[doc = "13: 14-bit"]
    B_0X_D = 13,
    #[doc = "14: 15-bit"]
    B_0X_E = 14,
    #[doc = "15: 16-bit"]
    B_0X_F = 15,
}
impl From<DS_A> for u8 {
    #[inline(always)]
    fn from(variant: DS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DS_A {
    type Ux = u8;
}
impl DS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DS_A {
        match self.bits {
            0 => DS_A::B_0X0,
            1 => DS_A::B_0X1,
            2 => DS_A::B_0X2,
            3 => DS_A::B_0X3,
            4 => DS_A::B_0X4,
            5 => DS_A::B_0X5,
            6 => DS_A::B_0X6,
            7 => DS_A::B_0X7,
            8 => DS_A::B_0X8,
            9 => DS_A::B_0X9,
            10 => DS_A::B_0X_A,
            11 => DS_A::B_0X_B,
            12 => DS_A::B_0X_C,
            13 => DS_A::B_0X_D,
            14 => DS_A::B_0X_E,
            15 => DS_A::B_0X_F,
            _ => unreachable!(),
        }
    }
    #[doc = "Not used"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DS_A::B_0X0
    }
    #[doc = "Not used"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DS_A::B_0X1
    }
    #[doc = "Not used"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DS_A::B_0X2
    }
    #[doc = "4-bit"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == DS_A::B_0X3
    }
    #[doc = "5-bit"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == DS_A::B_0X4
    }
    #[doc = "6-bit"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == DS_A::B_0X5
    }
    #[doc = "7-bit"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == DS_A::B_0X6
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == DS_A::B_0X7
    }
    #[doc = "9-bit"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == DS_A::B_0X8
    }
    #[doc = "10-bit"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == DS_A::B_0X9
    }
    #[doc = "11-bit"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == DS_A::B_0X_A
    }
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == DS_A::B_0X_B
    }
    #[doc = "13-bit"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == DS_A::B_0X_C
    }
    #[doc = "14-bit"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == DS_A::B_0X_D
    }
    #[doc = "15-bit"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == DS_A::B_0X_E
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == DS_A::B_0X_F
    }
}
#[doc = "Field `DS` writer - Data size These bits configure the data length for SPI transfers. If software attempts to write one of the 'Not used' values, they are forced to the value '0111' (8-bit) Note: These bits are not used in I2S mode."]
pub type DS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, DS_A>;
impl<'a, REG> DS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not used"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DS_A::B_0X0)
    }
    #[doc = "Not used"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DS_A::B_0X1)
    }
    #[doc = "Not used"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(DS_A::B_0X2)
    }
    #[doc = "4-bit"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(DS_A::B_0X3)
    }
    #[doc = "5-bit"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(DS_A::B_0X4)
    }
    #[doc = "6-bit"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(DS_A::B_0X5)
    }
    #[doc = "7-bit"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(DS_A::B_0X6)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(DS_A::B_0X7)
    }
    #[doc = "9-bit"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(DS_A::B_0X8)
    }
    #[doc = "10-bit"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(DS_A::B_0X9)
    }
    #[doc = "11-bit"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(DS_A::B_0X_A)
    }
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(DS_A::B_0X_B)
    }
    #[doc = "13-bit"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(DS_A::B_0X_C)
    }
    #[doc = "14-bit"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(DS_A::B_0X_D)
    }
    #[doc = "15-bit"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(DS_A::B_0X_E)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(DS_A::B_0X_F)
    }
}
#[doc = "Field `FRXTH` reader - FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event Note: This bit is not used in I2S mode."]
pub type FRXTH_R = crate::BitReader<FRXTH_A>;
#[doc = "FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event Note: This bit is not used in I2S mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRXTH_A {
    #[doc = "0: RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    B_0X0 = 0,
    #[doc = "1: RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    B_0X1 = 1,
}
impl From<FRXTH_A> for bool {
    #[inline(always)]
    fn from(variant: FRXTH_A) -> Self {
        variant as u8 != 0
    }
}
impl FRXTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FRXTH_A {
        match self.bits {
            false => FRXTH_A::B_0X0,
            true => FRXTH_A::B_0X1,
        }
    }
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FRXTH_A::B_0X0
    }
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FRXTH_A::B_0X1
    }
}
#[doc = "Field `FRXTH` writer - FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event Note: This bit is not used in I2S mode."]
pub type FRXTH_W<'a, REG> = crate::BitWriter<'a, REG, FRXTH_A>;
impl<'a, REG> FRXTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FRXTH_A::B_0X0)
    }
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FRXTH_A::B_0X1)
    }
}
#[doc = "Field `LDMA_RX` reader - Last DMA transfer for reception"]
pub type LDMA_RX_R = crate::BitReader<LDMA_RX_A>;
#[doc = "Last DMA transfer for reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDMA_RX_A {
    #[doc = "0: Number of data to transfer is even"]
    B_0X0 = 0,
    #[doc = "1: Number of data to transfer is odd"]
    B_0X1 = 1,
}
impl From<LDMA_RX_A> for bool {
    #[inline(always)]
    fn from(variant: LDMA_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl LDMA_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LDMA_RX_A {
        match self.bits {
            false => LDMA_RX_A::B_0X0,
            true => LDMA_RX_A::B_0X1,
        }
    }
    #[doc = "Number of data to transfer is even"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LDMA_RX_A::B_0X0
    }
    #[doc = "Number of data to transfer is odd"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LDMA_RX_A::B_0X1
    }
}
#[doc = "Field `LDMA_RX` writer - Last DMA transfer for reception"]
pub type LDMA_RX_W<'a, REG> = crate::BitWriter<'a, REG, LDMA_RX_A>;
impl<'a, REG> LDMA_RX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Number of data to transfer is even"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LDMA_RX_A::B_0X0)
    }
    #[doc = "Number of data to transfer is odd"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LDMA_RX_A::B_0X1)
    }
}
#[doc = "Field `LDMA_TX` reader - Last DMA transfer for transmission"]
pub type LDMA_TX_R = crate::BitReader<LDMA_TX_A>;
#[doc = "Last DMA transfer for transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDMA_TX_A {
    #[doc = "0: Number of data to transfer is even"]
    B_0X0 = 0,
    #[doc = "1: Number of data to transfer is odd"]
    B_0X1 = 1,
}
impl From<LDMA_TX_A> for bool {
    #[inline(always)]
    fn from(variant: LDMA_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl LDMA_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LDMA_TX_A {
        match self.bits {
            false => LDMA_TX_A::B_0X0,
            true => LDMA_TX_A::B_0X1,
        }
    }
    #[doc = "Number of data to transfer is even"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LDMA_TX_A::B_0X0
    }
    #[doc = "Number of data to transfer is odd"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LDMA_TX_A::B_0X1
    }
}
#[doc = "Field `LDMA_TX` writer - Last DMA transfer for transmission"]
pub type LDMA_TX_W<'a, REG> = crate::BitWriter<'a, REG, LDMA_TX_A>;
impl<'a, REG> LDMA_TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Number of data to transfer is even"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LDMA_TX_A::B_0X0)
    }
    #[doc = "Number of data to transfer is odd"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LDMA_TX_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set."]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set."]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS output enable Note: This bit is not used in I2S mode and SPI TI mode."]
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = '1', or FRF = '1'. Note: 1. This bit must be written only when the SPI is disabled (SPE=0). 2. This bit is not used in I2S mode and SPI TI mode."]
    #[inline(always)]
    pub fn nssp(&self) -> NSSP_R {
        NSSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0). This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I2S mode)."]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Data size These bits configure the data length for SPI transfers. If software attempts to write one of the 'Not used' values, they are forced to the value '0111' (8-bit) Note: These bits are not used in I2S mode."]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event Note: This bit is not used in I2S mode."]
    #[inline(always)]
    pub fn frxth(&self) -> FRXTH_R {
        FRXTH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Last DMA transfer for reception"]
    #[inline(always)]
    pub fn ldma_rx(&self) -> LDMA_RX_R {
        LDMA_RX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission"]
    #[inline(always)]
    pub fn ldma_tx(&self) -> LDMA_TX_R {
        LDMA_TX_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<SPI_CR2_SPEC> {
        RXDMAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set."]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<SPI_CR2_SPEC> {
        TXDMAEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - SS output enable Note: This bit is not used in I2S mode and SPI TI mode."]
    #[inline(always)]
    #[must_use]
    pub fn ssoe(&mut self) -> SSOE_W<SPI_CR2_SPEC> {
        SSOE_W::new(self, 2)
    }
    #[doc = "Bit 3 - NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = '1', or FRF = '1'. Note: 1. This bit must be written only when the SPI is disabled (SPE=0). 2. This bit is not used in I2S mode and SPI TI mode."]
    #[inline(always)]
    #[must_use]
    pub fn nssp(&mut self) -> NSSP_W<SPI_CR2_SPEC> {
        NSSP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0). This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FRF_W<SPI_CR2_SPEC> {
        FRF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I2S mode)."]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<SPI_CR2_SPEC> {
        ERRIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<SPI_CR2_SPEC> {
        RXNEIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TXEIE_W<SPI_CR2_SPEC> {
        TXEIE_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Data size These bits configure the data length for SPI transfers. If software attempts to write one of the 'Not used' values, they are forced to the value '0111' (8-bit) Note: These bits are not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<SPI_CR2_SPEC> {
        DS_W::new(self, 8)
    }
    #[doc = "Bit 12 - FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event Note: This bit is not used in I2S mode."]
    #[inline(always)]
    #[must_use]
    pub fn frxth(&mut self) -> FRXTH_W<SPI_CR2_SPEC> {
        FRXTH_W::new(self, 12)
    }
    #[doc = "Bit 13 - Last DMA transfer for reception"]
    #[inline(always)]
    #[must_use]
    pub fn ldma_rx(&mut self) -> LDMA_RX_W<SPI_CR2_SPEC> {
        LDMA_RX_W::new(self, 13)
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ldma_tx(&mut self) -> LDMA_TX_W<SPI_CR2_SPEC> {
        LDMA_TX_W::new(self, 14)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_CR2_SPEC;
impl crate::RegisterSpec for SPI_CR2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spi_cr2::R`](R) reader structure"]
impl crate::Readable for SPI_CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_cr2::W`](W) writer structure"]
impl crate::Writable for SPI_CR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SPI_CR2 to value 0x0700"]
impl crate::Resettable for SPI_CR2_SPEC {
    const RESET_VALUE: u16 = 0x0700;
}
