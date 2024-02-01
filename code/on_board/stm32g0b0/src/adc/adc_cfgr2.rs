#[doc = "Register `ADC_CFGR2` reader"]
pub type R = crate::R<ADC_CFGR2_SPEC>;
#[doc = "Register `ADC_CFGR2` writer"]
pub type W = crate::W<ADC_CFGR2_SPEC>;
#[doc = "Field `OVSE` reader - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type OVSE_R = crate::BitReader<OVSE_A>;
#[doc = "Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVSE_A {
    #[doc = "0: Oversampler disabled"]
    B_0X0 = 0,
    #[doc = "1: Oversampler enabled"]
    B_0X1 = 1,
}
impl From<OVSE_A> for bool {
    #[inline(always)]
    fn from(variant: OVSE_A) -> Self {
        variant as u8 != 0
    }
}
impl OVSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVSE_A {
        match self.bits {
            false => OVSE_A::B_0X0,
            true => OVSE_A::B_0X1,
        }
    }
    #[doc = "Oversampler disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OVSE_A::B_0X0
    }
    #[doc = "Oversampler enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OVSE_A::B_0X1
    }
}
#[doc = "Field `OVSE` writer - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type OVSE_W<'a, REG> = crate::BitWriter<'a, REG, OVSE_A>;
impl<'a, REG> OVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oversampler disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVSE_A::B_0X0)
    }
    #[doc = "Oversampler enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVSE_A::B_0X1)
    }
}
#[doc = "Field `OVSR` reader - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type OVSR_R = crate::FieldReader<OVSR_A>;
#[doc = "Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSR_A {
    #[doc = "0: 2x"]
    B_0X0 = 0,
    #[doc = "1: 4x"]
    B_0X1 = 1,
    #[doc = "2: 8x"]
    B_0X2 = 2,
    #[doc = "3: 16x"]
    B_0X3 = 3,
    #[doc = "4: 32x"]
    B_0X4 = 4,
    #[doc = "5: 64x"]
    B_0X5 = 5,
    #[doc = "6: 128x"]
    B_0X6 = 6,
    #[doc = "7: 256x"]
    B_0X7 = 7,
}
impl From<OVSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OVSR_A {
    type Ux = u8;
}
impl OVSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVSR_A {
        match self.bits {
            0 => OVSR_A::B_0X0,
            1 => OVSR_A::B_0X1,
            2 => OVSR_A::B_0X2,
            3 => OVSR_A::B_0X3,
            4 => OVSR_A::B_0X4,
            5 => OVSR_A::B_0X5,
            6 => OVSR_A::B_0X6,
            7 => OVSR_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "2x"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OVSR_A::B_0X0
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OVSR_A::B_0X1
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OVSR_A::B_0X2
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OVSR_A::B_0X3
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == OVSR_A::B_0X4
    }
    #[doc = "64x"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == OVSR_A::B_0X5
    }
    #[doc = "128x"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == OVSR_A::B_0X6
    }
    #[doc = "256x"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == OVSR_A::B_0X7
    }
}
#[doc = "Field `OVSR` writer - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type OVSR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, OVSR_A>;
impl<'a, REG> OVSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2x"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::B_0X0)
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::B_0X1)
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::B_0X2)
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::B_0X3)
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::B_0X4)
    }
    #[doc = "64x"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::B_0X5)
    }
    #[doc = "128x"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::B_0X6)
    }
    #[doc = "256x"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(OVSR_A::B_0X7)
    }
}
#[doc = "Field `OVSS` reader - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type OVSS_R = crate::FieldReader<OVSS_A>;
#[doc = "Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSS_A {
    #[doc = "0: No shift"]
    B_0X0 = 0,
    #[doc = "1: Shift 1-bit"]
    B_0X1 = 1,
    #[doc = "2: Shift 2-bits"]
    B_0X2 = 2,
    #[doc = "3: Shift 3-bits"]
    B_0X3 = 3,
    #[doc = "4: Shift 4-bits"]
    B_0X4 = 4,
    #[doc = "5: Shift 5-bits"]
    B_0X5 = 5,
    #[doc = "6: Shift 6-bits"]
    B_0X6 = 6,
    #[doc = "7: Shift 7-bits"]
    B_0X7 = 7,
    #[doc = "8: Shift 8-bits"]
    B_0X8 = 8,
}
impl From<OVSS_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OVSS_A {
    type Ux = u8;
}
impl OVSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OVSS_A> {
        match self.bits {
            0 => Some(OVSS_A::B_0X0),
            1 => Some(OVSS_A::B_0X1),
            2 => Some(OVSS_A::B_0X2),
            3 => Some(OVSS_A::B_0X3),
            4 => Some(OVSS_A::B_0X4),
            5 => Some(OVSS_A::B_0X5),
            6 => Some(OVSS_A::B_0X6),
            7 => Some(OVSS_A::B_0X7),
            8 => Some(OVSS_A::B_0X8),
            _ => None,
        }
    }
    #[doc = "No shift"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OVSS_A::B_0X0
    }
    #[doc = "Shift 1-bit"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OVSS_A::B_0X1
    }
    #[doc = "Shift 2-bits"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OVSS_A::B_0X2
    }
    #[doc = "Shift 3-bits"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OVSS_A::B_0X3
    }
    #[doc = "Shift 4-bits"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == OVSS_A::B_0X4
    }
    #[doc = "Shift 5-bits"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == OVSS_A::B_0X5
    }
    #[doc = "Shift 6-bits"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == OVSS_A::B_0X6
    }
    #[doc = "Shift 7-bits"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == OVSS_A::B_0X7
    }
    #[doc = "Shift 8-bits"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == OVSS_A::B_0X8
    }
}
#[doc = "Field `OVSS` writer - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, OVSS_A>;
impl<'a, REG> OVSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No shift"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS_A::B_0X0)
    }
    #[doc = "Shift 1-bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS_A::B_0X1)
    }
    #[doc = "Shift 2-bits"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS_A::B_0X2)
    }
    #[doc = "Shift 3-bits"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS_A::B_0X3)
    }
    #[doc = "Shift 4-bits"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS_A::B_0X4)
    }
    #[doc = "Shift 5-bits"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS_A::B_0X5)
    }
    #[doc = "Shift 6-bits"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS_A::B_0X6)
    }
    #[doc = "Shift 7-bits"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS_A::B_0X7)
    }
    #[doc = "Shift 8-bits"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(OVSS_A::B_0X8)
    }
}
#[doc = "Field `TOVS` reader - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type TOVS_R = crate::BitReader<TOVS_A>;
#[doc = "Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOVS_A {
    #[doc = "0: All oversampled conversions for a channel are done consecutively after a trigger"]
    B_0X0 = 0,
    #[doc = "1: Each oversampled conversion for a channel needs a trigger"]
    B_0X1 = 1,
}
impl From<TOVS_A> for bool {
    #[inline(always)]
    fn from(variant: TOVS_A) -> Self {
        variant as u8 != 0
    }
}
impl TOVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TOVS_A {
        match self.bits {
            false => TOVS_A::B_0X0,
            true => TOVS_A::B_0X1,
        }
    }
    #[doc = "All oversampled conversions for a channel are done consecutively after a trigger"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TOVS_A::B_0X0
    }
    #[doc = "Each oversampled conversion for a channel needs a trigger"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TOVS_A::B_0X1
    }
}
#[doc = "Field `TOVS` writer - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type TOVS_W<'a, REG> = crate::BitWriter<'a, REG, TOVS_A>;
impl<'a, REG> TOVS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All oversampled conversions for a channel are done consecutively after a trigger"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TOVS_A::B_0X0)
    }
    #[doc = "Each oversampled conversion for a channel needs a trigger"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TOVS_A::B_0X1)
    }
}
#[doc = "Field `LFTRIG` reader - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type LFTRIG_R = crate::BitReader<LFTRIG_A>;
#[doc = "Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFTRIG_A {
    #[doc = "0: Low Frequency Trigger Mode disabled"]
    B_0X0 = 0,
    #[doc = "1: Low Frequency Trigger Mode enabled"]
    B_0X1 = 1,
}
impl From<LFTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: LFTRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl LFTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LFTRIG_A {
        match self.bits {
            false => LFTRIG_A::B_0X0,
            true => LFTRIG_A::B_0X1,
        }
    }
    #[doc = "Low Frequency Trigger Mode disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LFTRIG_A::B_0X0
    }
    #[doc = "Low Frequency Trigger Mode enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LFTRIG_A::B_0X1
    }
}
#[doc = "Field `LFTRIG` writer - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type LFTRIG_W<'a, REG> = crate::BitWriter<'a, REG, LFTRIG_A>;
impl<'a, REG> LFTRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low Frequency Trigger Mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LFTRIG_A::B_0X0)
    }
    #[doc = "Low Frequency Trigger Mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LFTRIG_A::B_0X1)
    }
}
#[doc = "Field `CKMODE` reader - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0)."]
pub type CKMODE_R = crate::FieldReader<CKMODE_A>;
#[doc = "ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKMODE_A {
    #[doc = "0: ADCCLK (Asynchronous clock mode), generated at product level (refer to RCC section)"]
    B_0X0 = 0,
    #[doc = "1: PCLK/2 (Synchronous clock mode)"]
    B_0X1 = 1,
    #[doc = "2: PCLK/4 (Synchronous clock mode)"]
    B_0X2 = 2,
    #[doc = "3: PCLK (Synchronous clock mode). This configuration must be enabled only if PCLK has a 50% duty clock cycle (APB prescaler configured inside the RCC must be bypassed and the system clock must by 50% duty cycle)"]
    B_0X3 = 3,
}
impl From<CKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKMODE_A {
    type Ux = u8;
}
impl CKMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKMODE_A {
        match self.bits {
            0 => CKMODE_A::B_0X0,
            1 => CKMODE_A::B_0X1,
            2 => CKMODE_A::B_0X2,
            3 => CKMODE_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "ADCCLK (Asynchronous clock mode), generated at product level (refer to RCC section)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CKMODE_A::B_0X0
    }
    #[doc = "PCLK/2 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CKMODE_A::B_0X1
    }
    #[doc = "PCLK/4 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == CKMODE_A::B_0X2
    }
    #[doc = "PCLK (Synchronous clock mode). This configuration must be enabled only if PCLK has a 50% duty clock cycle (APB prescaler configured inside the RCC must be bypassed and the system clock must by 50% duty cycle)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == CKMODE_A::B_0X3
    }
}
#[doc = "Field `CKMODE` writer - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0)."]
pub type CKMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CKMODE_A>;
impl<'a, REG> CKMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADCCLK (Asynchronous clock mode), generated at product level (refer to RCC section)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE_A::B_0X0)
    }
    #[doc = "PCLK/2 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE_A::B_0X1)
    }
    #[doc = "PCLK/4 (Synchronous clock mode)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE_A::B_0X2)
    }
    #[doc = "PCLK (Synchronous clock mode). This configuration must be enabled only if PCLK has a 50% duty clock cycle (APB prescaler configured inside the RCC must be bypassed and the system clock must by 50% duty cycle)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CKMODE_A::B_0X3)
    }
}
impl R {
    #[doc = "Bit 0 - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovse(&self) -> OVSE_R {
        OVSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 29 - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn lftrig(&self) -> LFTRIG_R {
        LFTRIG_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0)."]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Oversampler Enable This bit is set and cleared by software. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ovse(&mut self) -> OVSE_W<ADC_CFGR2_SPEC> {
        OVSE_W::new(self, 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio This bit filed defines the number of oversampling ratio. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ovsr(&mut self) -> OVSR_W<ADC_CFGR2_SPEC> {
        OVSR_W::new(self, 2)
    }
    #[doc = "Bits 5:8 - Oversampling shift This bit is set and cleared by software. Others: Reserved Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<ADC_CFGR2_SPEC> {
        OVSS_W::new(self, 5)
    }
    #[doc = "Bit 9 - Triggered Oversampling This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn tovs(&mut self) -> TOVS_W<ADC_CFGR2_SPEC> {
        TOVS_W::new(self, 9)
    }
    #[doc = "Bit 29 - Low frequency trigger mode enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn lftrig(&mut self) -> LFTRIG_W<ADC_CFGR2_SPEC> {
        LFTRIG_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - ADC clock mode These bits are set and cleared by software to define how the analog ADC is clocked: In all synchronous clock modes, there is no jitter in the delay from a timer trigger to the start of a conversion. Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0)."]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<ADC_CFGR2_SPEC> {
        CKMODE_W::new(self, 30)
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
#[doc = "ADC configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_CFGR2_SPEC;
impl crate::RegisterSpec for ADC_CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_cfgr2::R`](R) reader structure"]
impl crate::Readable for ADC_CFGR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc_cfgr2::W`](W) writer structure"]
impl crate::Writable for ADC_CFGR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CFGR2 to value 0"]
impl crate::Resettable for ADC_CFGR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
