#[doc = "Register `ADC_CHSELR_0` reader"]
pub type R = crate::R<ADC_CHSELR_0_SPEC>;
#[doc = "Register `ADC_CHSELR_0` writer"]
pub type W = crate::W<ADC_CHSELR_0_SPEC>;
#[doc = "Field `CHSEL0` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL0_R = crate::BitReader<CHSEL0_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL0_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL0_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL0_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL0_A {
        match self.bits {
            false => CHSEL0_A::B_0X0,
            true => CHSEL0_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL0_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL0_A::B_0X1
    }
}
#[doc = "Field `CHSEL0` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL0_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL0_A>;
impl<'a, REG> CHSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL0_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL0_A::B_0X1)
    }
}
#[doc = "Field `CHSEL1` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL1_R = crate::BitReader<CHSEL1_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL1_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL1_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL1_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL1_A {
        match self.bits {
            false => CHSEL1_A::B_0X0,
            true => CHSEL1_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL1_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL1_A::B_0X1
    }
}
#[doc = "Field `CHSEL1` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL1_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL1_A>;
impl<'a, REG> CHSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL1_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL1_A::B_0X1)
    }
}
#[doc = "Field `CHSEL2` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL2_R = crate::BitReader<CHSEL2_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL2_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL2_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL2_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL2_A {
        match self.bits {
            false => CHSEL2_A::B_0X0,
            true => CHSEL2_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL2_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL2_A::B_0X1
    }
}
#[doc = "Field `CHSEL2` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL2_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL2_A>;
impl<'a, REG> CHSEL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL2_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL2_A::B_0X1)
    }
}
#[doc = "Field `CHSEL3` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL3_R = crate::BitReader<CHSEL3_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL3_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL3_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL3_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL3_A {
        match self.bits {
            false => CHSEL3_A::B_0X0,
            true => CHSEL3_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL3_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL3_A::B_0X1
    }
}
#[doc = "Field `CHSEL3` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL3_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL3_A>;
impl<'a, REG> CHSEL3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL3_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL3_A::B_0X1)
    }
}
#[doc = "Field `CHSEL4` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL4_R = crate::BitReader<CHSEL4_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL4_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL4_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL4_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL4_A {
        match self.bits {
            false => CHSEL4_A::B_0X0,
            true => CHSEL4_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL4_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL4_A::B_0X1
    }
}
#[doc = "Field `CHSEL4` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL4_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL4_A>;
impl<'a, REG> CHSEL4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL4_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL4_A::B_0X1)
    }
}
#[doc = "Field `CHSEL5` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL5_R = crate::BitReader<CHSEL5_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL5_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL5_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL5_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL5_A {
        match self.bits {
            false => CHSEL5_A::B_0X0,
            true => CHSEL5_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL5_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL5_A::B_0X1
    }
}
#[doc = "Field `CHSEL5` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL5_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL5_A>;
impl<'a, REG> CHSEL5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL5_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL5_A::B_0X1)
    }
}
#[doc = "Field `CHSEL6` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL6_R = crate::BitReader<CHSEL6_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL6_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL6_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL6_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL6_A {
        match self.bits {
            false => CHSEL6_A::B_0X0,
            true => CHSEL6_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL6_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL6_A::B_0X1
    }
}
#[doc = "Field `CHSEL6` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL6_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL6_A>;
impl<'a, REG> CHSEL6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL6_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL6_A::B_0X1)
    }
}
#[doc = "Field `CHSEL7` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL7_R = crate::BitReader<CHSEL7_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL7_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL7_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL7_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL7_A {
        match self.bits {
            false => CHSEL7_A::B_0X0,
            true => CHSEL7_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL7_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL7_A::B_0X1
    }
}
#[doc = "Field `CHSEL7` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL7_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL7_A>;
impl<'a, REG> CHSEL7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL7_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL7_A::B_0X1)
    }
}
#[doc = "Field `CHSEL8` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL8_R = crate::BitReader<CHSEL8_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL8_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL8_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL8_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL8_A {
        match self.bits {
            false => CHSEL8_A::B_0X0,
            true => CHSEL8_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL8_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL8_A::B_0X1
    }
}
#[doc = "Field `CHSEL8` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL8_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL8_A>;
impl<'a, REG> CHSEL8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL8_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL8_A::B_0X1)
    }
}
#[doc = "Field `CHSEL9` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL9_R = crate::BitReader<CHSEL9_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL9_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL9_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL9_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL9_A {
        match self.bits {
            false => CHSEL9_A::B_0X0,
            true => CHSEL9_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL9_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL9_A::B_0X1
    }
}
#[doc = "Field `CHSEL9` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL9_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL9_A>;
impl<'a, REG> CHSEL9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL9_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL9_A::B_0X1)
    }
}
#[doc = "Field `CHSEL10` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL10_R = crate::BitReader<CHSEL10_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL10_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL10_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL10_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL10_A {
        match self.bits {
            false => CHSEL10_A::B_0X0,
            true => CHSEL10_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL10_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL10_A::B_0X1
    }
}
#[doc = "Field `CHSEL10` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL10_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL10_A>;
impl<'a, REG> CHSEL10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL10_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL10_A::B_0X1)
    }
}
#[doc = "Field `CHSEL11` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL11_R = crate::BitReader<CHSEL11_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL11_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL11_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL11_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL11_A {
        match self.bits {
            false => CHSEL11_A::B_0X0,
            true => CHSEL11_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL11_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL11_A::B_0X1
    }
}
#[doc = "Field `CHSEL11` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL11_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL11_A>;
impl<'a, REG> CHSEL11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL11_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL11_A::B_0X1)
    }
}
#[doc = "Field `CHSEL12` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL12_R = crate::BitReader<CHSEL12_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL12_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL12_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL12_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL12_A {
        match self.bits {
            false => CHSEL12_A::B_0X0,
            true => CHSEL12_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL12_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL12_A::B_0X1
    }
}
#[doc = "Field `CHSEL12` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL12_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL12_A>;
impl<'a, REG> CHSEL12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL12_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL12_A::B_0X1)
    }
}
#[doc = "Field `CHSEL13` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL13_R = crate::BitReader<CHSEL13_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL13_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL13_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL13_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL13_A {
        match self.bits {
            false => CHSEL13_A::B_0X0,
            true => CHSEL13_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL13_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL13_A::B_0X1
    }
}
#[doc = "Field `CHSEL13` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL13_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL13_A>;
impl<'a, REG> CHSEL13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL13_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL13_A::B_0X1)
    }
}
#[doc = "Field `CHSEL14` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL14_R = crate::BitReader<CHSEL14_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL14_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL14_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL14_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL14_A {
        match self.bits {
            false => CHSEL14_A::B_0X0,
            true => CHSEL14_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL14_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL14_A::B_0X1
    }
}
#[doc = "Field `CHSEL14` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL14_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL14_A>;
impl<'a, REG> CHSEL14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL14_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL14_A::B_0X1)
    }
}
#[doc = "Field `CHSEL15` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL15_R = crate::BitReader<CHSEL15_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL15_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL15_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL15_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL15_A {
        match self.bits {
            false => CHSEL15_A::B_0X0,
            true => CHSEL15_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL15_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL15_A::B_0X1
    }
}
#[doc = "Field `CHSEL15` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL15_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL15_A>;
impl<'a, REG> CHSEL15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL15_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL15_A::B_0X1)
    }
}
#[doc = "Field `CHSEL16` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL16_R = crate::BitReader<CHSEL16_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL16_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL16_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL16_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL16_A {
        match self.bits {
            false => CHSEL16_A::B_0X0,
            true => CHSEL16_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL16_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL16_A::B_0X1
    }
}
#[doc = "Field `CHSEL16` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL16_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL16_A>;
impl<'a, REG> CHSEL16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL16_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL16_A::B_0X1)
    }
}
#[doc = "Field `CHSEL17` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL17_R = crate::BitReader<CHSEL17_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL17_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL17_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL17_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL17_A {
        match self.bits {
            false => CHSEL17_A::B_0X0,
            true => CHSEL17_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL17_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL17_A::B_0X1
    }
}
#[doc = "Field `CHSEL17` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL17_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL17_A>;
impl<'a, REG> CHSEL17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL17_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL17_A::B_0X1)
    }
}
#[doc = "Field `CHSEL18` reader - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL18_R = crate::BitReader<CHSEL18_A>;
#[doc = "Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSEL18_A {
    #[doc = "0: Input Channel-x is not selected for conversion"]
    B_0X0 = 0,
    #[doc = "1: Input Channel-x is selected for conversion"]
    B_0X1 = 1,
}
impl From<CHSEL18_A> for bool {
    #[inline(always)]
    fn from(variant: CHSEL18_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSEL18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSEL18_A {
        match self.bits {
            false => CHSEL18_A::B_0X0,
            true => CHSEL18_A::B_0X1,
        }
    }
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSEL18_A::B_0X0
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSEL18_A::B_0X1
    }
}
#[doc = "Field `CHSEL18` writer - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSEL18_W<'a, REG> = crate::BitWriter<'a, REG, CHSEL18_A>;
impl<'a, REG> CHSEL18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input Channel-x is not selected for conversion"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL18_A::B_0X0)
    }
    #[doc = "Input Channel-x is selected for conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL18_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel0(&self) -> CHSEL0_R {
        CHSEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel1(&self) -> CHSEL1_R {
        CHSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel2(&self) -> CHSEL2_R {
        CHSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel3(&self) -> CHSEL3_R {
        CHSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel4(&self) -> CHSEL4_R {
        CHSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel5(&self) -> CHSEL5_R {
        CHSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel6(&self) -> CHSEL6_R {
        CHSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel7(&self) -> CHSEL7_R {
        CHSEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel8(&self) -> CHSEL8_R {
        CHSEL8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel9(&self) -> CHSEL9_R {
        CHSEL9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel10(&self) -> CHSEL10_R {
        CHSEL10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel11(&self) -> CHSEL11_R {
        CHSEL11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel12(&self) -> CHSEL12_R {
        CHSEL12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel13(&self) -> CHSEL13_R {
        CHSEL13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel14(&self) -> CHSEL14_R {
        CHSEL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel15(&self) -> CHSEL15_R {
        CHSEL15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel16(&self) -> CHSEL16_R {
        CHSEL16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel17(&self) -> CHSEL17_R {
        CHSEL17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chsel18(&self) -> CHSEL18_R {
        CHSEL18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel0(&mut self) -> CHSEL0_W<ADC_CHSELR_0_SPEC> {
        CHSEL0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel1(&mut self) -> CHSEL1_W<ADC_CHSELR_0_SPEC> {
        CHSEL1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel2(&mut self) -> CHSEL2_W<ADC_CHSELR_0_SPEC> {
        CHSEL2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel3(&mut self) -> CHSEL3_W<ADC_CHSELR_0_SPEC> {
        CHSEL3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel4(&mut self) -> CHSEL4_W<ADC_CHSELR_0_SPEC> {
        CHSEL4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel5(&mut self) -> CHSEL5_W<ADC_CHSELR_0_SPEC> {
        CHSEL5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel6(&mut self) -> CHSEL6_W<ADC_CHSELR_0_SPEC> {
        CHSEL6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel7(&mut self) -> CHSEL7_W<ADC_CHSELR_0_SPEC> {
        CHSEL7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel8(&mut self) -> CHSEL8_W<ADC_CHSELR_0_SPEC> {
        CHSEL8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel9(&mut self) -> CHSEL9_W<ADC_CHSELR_0_SPEC> {
        CHSEL9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel10(&mut self) -> CHSEL10_W<ADC_CHSELR_0_SPEC> {
        CHSEL10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel11(&mut self) -> CHSEL11_W<ADC_CHSELR_0_SPEC> {
        CHSEL11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel12(&mut self) -> CHSEL12_W<ADC_CHSELR_0_SPEC> {
        CHSEL12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel13(&mut self) -> CHSEL13_W<ADC_CHSELR_0_SPEC> {
        CHSEL13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel14(&mut self) -> CHSEL14_W<ADC_CHSELR_0_SPEC> {
        CHSEL14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel15(&mut self) -> CHSEL15_W<ADC_CHSELR_0_SPEC> {
        CHSEL15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel16(&mut self) -> CHSEL16_W<ADC_CHSELR_0_SPEC> {
        CHSEL16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel17(&mut self) -> CHSEL17_W<ADC_CHSELR_0_SPEC> {
        CHSEL17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Channel-x selection These bits are written by software and define which channels are part of the sequence of channels to be converted. Refer to for ADC inputs connected to external channels and internal sources. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chsel18(&mut self) -> CHSEL18_W<ADC_CHSELR_0_SPEC> {
        CHSEL18_W::new(self, 18)
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
#[doc = "ADC channel selection register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_chselr_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_chselr_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_CHSELR_0_SPEC;
impl crate::RegisterSpec for ADC_CHSELR_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_chselr_0::R`](R) reader structure"]
impl crate::Readable for ADC_CHSELR_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc_chselr_0::W`](W) writer structure"]
impl crate::Writable for ADC_CHSELR_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CHSELR_0 to value 0"]
impl crate::Resettable for ADC_CHSELR_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
