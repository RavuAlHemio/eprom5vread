#[doc = "Register `SPI_I2SCFGR` reader"]
pub type R = crate::R<SPI_I2SCFGR_SPEC>;
#[doc = "Register `SPI_I2SCFGR` writer"]
pub type W = crate::W<SPI_I2SCFGR_SPEC>;
#[doc = "Field `CHLEN` reader - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode."]
pub type CHLEN_R = crate::BitReader<CHLEN_A>;
#[doc = "Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHLEN_A {
    #[doc = "0: 16-bit wide"]
    B_0X0 = 0,
    #[doc = "1: 32-bit wide"]
    B_0X1 = 1,
}
impl From<CHLEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CHLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHLEN_A {
        match self.bits {
            false => CHLEN_A::B_0X0,
            true => CHLEN_A::B_0X1,
        }
    }
    #[doc = "16-bit wide"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHLEN_A::B_0X0
    }
    #[doc = "32-bit wide"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHLEN_A::B_0X1
    }
}
#[doc = "Field `CHLEN` writer - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode."]
pub type CHLEN_W<'a, REG> = crate::BitWriter<'a, REG, CHLEN_A>;
impl<'a, REG> CHLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "16-bit wide"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHLEN_A::B_0X0)
    }
    #[doc = "32-bit wide"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHLEN_A::B_0X1)
    }
}
#[doc = "Field `DATLEN` reader - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type DATLEN_R = crate::FieldReader<DATLEN_A>;
#[doc = "Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATLEN_A {
    #[doc = "0: 16-bit data length"]
    B_0X0 = 0,
    #[doc = "1: 24-bit data length"]
    B_0X1 = 1,
    #[doc = "2: 32-bit data length"]
    B_0X2 = 2,
    #[doc = "3: Not allowed"]
    B_0X3 = 3,
}
impl From<DATLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DATLEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATLEN_A {
    type Ux = u8;
}
impl DATLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATLEN_A {
        match self.bits {
            0 => DATLEN_A::B_0X0,
            1 => DATLEN_A::B_0X1,
            2 => DATLEN_A::B_0X2,
            3 => DATLEN_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "16-bit data length"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DATLEN_A::B_0X0
    }
    #[doc = "24-bit data length"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DATLEN_A::B_0X1
    }
    #[doc = "32-bit data length"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DATLEN_A::B_0X2
    }
    #[doc = "Not allowed"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == DATLEN_A::B_0X3
    }
}
#[doc = "Field `DATLEN` writer - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type DATLEN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DATLEN_A>;
impl<'a, REG> DATLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-bit data length"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN_A::B_0X0)
    }
    #[doc = "24-bit data length"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN_A::B_0X1)
    }
    #[doc = "32-bit data length"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN_A::B_0X2)
    }
    #[doc = "Not allowed"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN_A::B_0X3)
    }
}
#[doc = "Field `CKPOL` reader - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals."]
pub type CKPOL_R = crate::BitReader<CKPOL_A>;
#[doc = "Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPOL_A {
    #[doc = "0: I2S clock inactive state is low level"]
    B_0X0 = 0,
    #[doc = "1: I2S clock inactive state is high level"]
    B_0X1 = 1,
}
impl From<CKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CKPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CKPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKPOL_A {
        match self.bits {
            false => CKPOL_A::B_0X0,
            true => CKPOL_A::B_0X1,
        }
    }
    #[doc = "I2S clock inactive state is low level"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CKPOL_A::B_0X0
    }
    #[doc = "I2S clock inactive state is high level"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CKPOL_A::B_0X1
    }
}
#[doc = "Field `CKPOL` writer - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals."]
pub type CKPOL_W<'a, REG> = crate::BitWriter<'a, REG, CKPOL_A>;
impl<'a, REG> CKPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2S clock inactive state is low level"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL_A::B_0X0)
    }
    #[doc = "I2S clock inactive state is high level"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL_A::B_0X1)
    }
}
#[doc = "Field `I2SSTD` reader - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type I2SSTD_R = crate::FieldReader<I2SSTD_A>;
#[doc = "I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SSTD_A {
    #[doc = "0: I2S Philips standard"]
    B_0X0 = 0,
    #[doc = "1: MSB justified standard (left justified)"]
    B_0X1 = 1,
    #[doc = "2: LSB justified standard (right justified)"]
    B_0X2 = 2,
    #[doc = "3: PCM standard"]
    B_0X3 = 3,
}
impl From<I2SSTD_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SSTD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2SSTD_A {
    type Ux = u8;
}
impl I2SSTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2SSTD_A {
        match self.bits {
            0 => I2SSTD_A::B_0X0,
            1 => I2SSTD_A::B_0X1,
            2 => I2SSTD_A::B_0X2,
            3 => I2SSTD_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "I2S Philips standard"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2SSTD_A::B_0X0
    }
    #[doc = "MSB justified standard (left justified)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2SSTD_A::B_0X1
    }
    #[doc = "LSB justified standard (right justified)"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2SSTD_A::B_0X2
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == I2SSTD_A::B_0X3
    }
}
#[doc = "Field `I2SSTD` writer - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type I2SSTD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, I2SSTD_A>;
impl<'a, REG> I2SSTD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I2S Philips standard"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD_A::B_0X0)
    }
    #[doc = "MSB justified standard (left justified)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD_A::B_0X1)
    }
    #[doc = "LSB justified standard (right justified)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD_A::B_0X2)
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD_A::B_0X3)
    }
}
#[doc = "Field `PCMSYNC` reader - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
pub type PCMSYNC_R = crate::BitReader<PCMSYNC_A>;
#[doc = "PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCMSYNC_A {
    #[doc = "0: Short frame synchronization"]
    B_0X0 = 0,
    #[doc = "1: Long frame synchronization"]
    B_0X1 = 1,
}
impl From<PCMSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: PCMSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl PCMSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCMSYNC_A {
        match self.bits {
            false => PCMSYNC_A::B_0X0,
            true => PCMSYNC_A::B_0X1,
        }
    }
    #[doc = "Short frame synchronization"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PCMSYNC_A::B_0X0
    }
    #[doc = "Long frame synchronization"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PCMSYNC_A::B_0X1
    }
}
#[doc = "Field `PCMSYNC` writer - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
pub type PCMSYNC_W<'a, REG> = crate::BitWriter<'a, REG, PCMSYNC_A>;
impl<'a, REG> PCMSYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Short frame synchronization"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PCMSYNC_A::B_0X0)
    }
    #[doc = "Long frame synchronization"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PCMSYNC_A::B_0X1)
    }
}
#[doc = "Field `I2SCFG` reader - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type I2SCFG_R = crate::FieldReader<I2SCFG_A>;
#[doc = "I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SCFG_A {
    #[doc = "0: Slave - transmit"]
    B_0X0 = 0,
    #[doc = "1: Slave - receive"]
    B_0X1 = 1,
    #[doc = "2: Master - transmit"]
    B_0X2 = 2,
    #[doc = "3: Master - receive"]
    B_0X3 = 3,
}
impl From<I2SCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SCFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2SCFG_A {
    type Ux = u8;
}
impl I2SCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2SCFG_A {
        match self.bits {
            0 => I2SCFG_A::B_0X0,
            1 => I2SCFG_A::B_0X1,
            2 => I2SCFG_A::B_0X2,
            3 => I2SCFG_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Slave - transmit"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2SCFG_A::B_0X0
    }
    #[doc = "Slave - receive"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2SCFG_A::B_0X1
    }
    #[doc = "Master - transmit"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2SCFG_A::B_0X2
    }
    #[doc = "Master - receive"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == I2SCFG_A::B_0X3
    }
}
#[doc = "Field `I2SCFG` writer - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode."]
pub type I2SCFG_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, I2SCFG_A>;
impl<'a, REG> I2SCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slave - transmit"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG_A::B_0X0)
    }
    #[doc = "Slave - receive"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG_A::B_0X1)
    }
    #[doc = "Master - transmit"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG_A::B_0X2)
    }
    #[doc = "Master - receive"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG_A::B_0X3)
    }
}
#[doc = "Field `I2SE` reader - I2S enable Note: This bit is not used in SPI mode."]
pub type I2SE_R = crate::BitReader<I2SE_A>;
#[doc = "I2S enable Note: This bit is not used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SE_A {
    #[doc = "0: I2S peripheral is disabled"]
    B_0X0 = 0,
    #[doc = "1: I2S peripheral is enabled"]
    B_0X1 = 1,
}
impl From<I2SE_A> for bool {
    #[inline(always)]
    fn from(variant: I2SE_A) -> Self {
        variant as u8 != 0
    }
}
impl I2SE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2SE_A {
        match self.bits {
            false => I2SE_A::B_0X0,
            true => I2SE_A::B_0X1,
        }
    }
    #[doc = "I2S peripheral is disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2SE_A::B_0X0
    }
    #[doc = "I2S peripheral is enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2SE_A::B_0X1
    }
}
#[doc = "Field `I2SE` writer - I2S enable Note: This bit is not used in SPI mode."]
pub type I2SE_W<'a, REG> = crate::BitWriter<'a, REG, I2SE_A>;
impl<'a, REG> I2SE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2S peripheral is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2SE_A::B_0X0)
    }
    #[doc = "I2S peripheral is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2SE_A::B_0X1)
    }
}
#[doc = "Field `I2SMOD` reader - I2S mode selection Note: This bit should be configured when the SPI is disabled."]
pub type I2SMOD_R = crate::BitReader<I2SMOD_A>;
#[doc = "I2S mode selection Note: This bit should be configured when the SPI is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SMOD_A {
    #[doc = "0: SPI mode is selected"]
    B_0X0 = 0,
    #[doc = "1: I2S mode is selected"]
    B_0X1 = 1,
}
impl From<I2SMOD_A> for bool {
    #[inline(always)]
    fn from(variant: I2SMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl I2SMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2SMOD_A {
        match self.bits {
            false => I2SMOD_A::B_0X0,
            true => I2SMOD_A::B_0X1,
        }
    }
    #[doc = "SPI mode is selected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2SMOD_A::B_0X0
    }
    #[doc = "I2S mode is selected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2SMOD_A::B_0X1
    }
}
#[doc = "Field `I2SMOD` writer - I2S mode selection Note: This bit should be configured when the SPI is disabled."]
pub type I2SMOD_W<'a, REG> = crate::BitWriter<'a, REG, I2SMOD_A>;
impl<'a, REG> I2SMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI mode is selected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2SMOD_A::B_0X0)
    }
    #[doc = "I2S mode is selected"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2SMOD_A::B_0X1)
    }
}
#[doc = "Field `ASTRTEN` reader - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information."]
pub type ASTRTEN_R = crate::BitReader<ASTRTEN_A>;
#[doc = "Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASTRTEN_A {
    #[doc = "0: The Asynchronous start is disabled."]
    B_0X0 = 0,
    #[doc = "1: The Asynchronous start is enabled."]
    B_0X1 = 1,
}
impl From<ASTRTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ASTRTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ASTRTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASTRTEN_A {
        match self.bits {
            false => ASTRTEN_A::B_0X0,
            true => ASTRTEN_A::B_0X1,
        }
    }
    #[doc = "The Asynchronous start is disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ASTRTEN_A::B_0X0
    }
    #[doc = "The Asynchronous start is enabled."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ASTRTEN_A::B_0X1
    }
}
#[doc = "Field `ASTRTEN` writer - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information."]
pub type ASTRTEN_W<'a, REG> = crate::BitWriter<'a, REG, ASTRTEN_A>;
impl<'a, REG> ASTRTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Asynchronous start is disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ASTRTEN_A::B_0X0)
    }
    #[doc = "The Asynchronous start is enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ASTRTEN_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode."]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals."]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - I2S enable Note: This bit is not used in SPI mode."]
    #[inline(always)]
    pub fn i2se(&self) -> I2SE_R {
        I2SE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2S mode selection Note: This bit should be configured when the SPI is disabled."]
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information."]
    #[inline(always)]
    pub fn astrten(&self) -> ASTRTEN_R {
        ASTRTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> CHLEN_W<SPI_I2SCFGR_SPEC> {
        CHLEN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DATLEN_W<SPI_I2SCFGR_SPEC> {
        DATLEN_W::new(self, 1)
    }
    #[doc = "Bit 3 - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals."]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<SPI_I2SCFGR_SPEC> {
        CKPOL_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2SSTD_W<SPI_I2SCFGR_SPEC> {
        I2SSTD_W::new(self, 4)
    }
    #[doc = "Bit 7 - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<SPI_I2SCFGR_SPEC> {
        PCMSYNC_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2scfg(&mut self) -> I2SCFG_W<SPI_I2SCFGR_SPEC> {
        I2SCFG_W::new(self, 8)
    }
    #[doc = "Bit 10 - I2S enable Note: This bit is not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2se(&mut self) -> I2SE_W<SPI_I2SCFGR_SPEC> {
        I2SE_W::new(self, 10)
    }
    #[doc = "Bit 11 - I2S mode selection Note: This bit should be configured when the SPI is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn i2smod(&mut self) -> I2SMOD_W<SPI_I2SCFGR_SPEC> {
        I2SMOD_W::new(self, 11)
    }
    #[doc = "Bit 12 - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn astrten(&mut self) -> ASTRTEN_W<SPI_I2SCFGR_SPEC> {
        ASTRTEN_W::new(self, 12)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_i2scfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_i2scfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_I2SCFGR_SPEC;
impl crate::RegisterSpec for SPI_I2SCFGR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spi_i2scfgr::R`](R) reader structure"]
impl crate::Readable for SPI_I2SCFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_i2scfgr::W`](W) writer structure"]
impl crate::Writable for SPI_I2SCFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SPI_I2SCFGR to value 0"]
impl crate::Resettable for SPI_I2SCFGR_SPEC {
    const RESET_VALUE: u16 = 0;
}
