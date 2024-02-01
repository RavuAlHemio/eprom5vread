#[doc = "Register `ADC_SMPR` reader"]
pub type R = crate::R<ADC_SMPR_SPEC>;
#[doc = "Register `ADC_SMPR` writer"]
pub type W = crate::W<ADC_SMPR_SPEC>;
#[doc = "Field `SMP1` reader - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMP1_R = crate::FieldReader<SMP1_A>;
#[doc = "Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP1_A {
    #[doc = "0: 1.5 ADC clock cycles"]
    B_0X0 = 0,
    #[doc = "1: 3.5 ADC clock cycles"]
    B_0X1 = 1,
    #[doc = "2: 7.5 ADC clock cycles"]
    B_0X2 = 2,
    #[doc = "3: 12.5 ADC clock cycles"]
    B_0X3 = 3,
    #[doc = "4: 19.5 ADC clock cycles"]
    B_0X4 = 4,
    #[doc = "5: 39.5 ADC clock cycles"]
    B_0X5 = 5,
    #[doc = "6: 79.5 ADC clock cycles"]
    B_0X6 = 6,
    #[doc = "7: 160.5 ADC clock cycles"]
    B_0X7 = 7,
}
impl From<SMP1_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP1_A {
    type Ux = u8;
}
impl SMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP1_A {
        match self.bits {
            0 => SMP1_A::B_0X0,
            1 => SMP1_A::B_0X1,
            2 => SMP1_A::B_0X2,
            3 => SMP1_A::B_0X3,
            4 => SMP1_A::B_0X4,
            5 => SMP1_A::B_0X5,
            6 => SMP1_A::B_0X6,
            7 => SMP1_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMP1_A::B_0X0
    }
    #[doc = "3.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMP1_A::B_0X1
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SMP1_A::B_0X2
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SMP1_A::B_0X3
    }
    #[doc = "19.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == SMP1_A::B_0X4
    }
    #[doc = "39.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == SMP1_A::B_0X5
    }
    #[doc = "79.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == SMP1_A::B_0X6
    }
    #[doc = "160.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == SMP1_A::B_0X7
    }
}
#[doc = "Field `SMP1` writer - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMP1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SMP1_A>;
impl<'a, REG> SMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1_A::B_0X0)
    }
    #[doc = "3.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1_A::B_0X1)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1_A::B_0X2)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1_A::B_0X3)
    }
    #[doc = "19.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1_A::B_0X4)
    }
    #[doc = "39.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1_A::B_0X5)
    }
    #[doc = "79.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1_A::B_0X6)
    }
    #[doc = "160.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1_A::B_0X7)
    }
}
#[doc = "Field `SMP2` reader - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMP2_R = crate::FieldReader<SMP2_A>;
#[doc = "Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP2_A {
    #[doc = "0: 1.5 ADC clock cycles"]
    B_0X0 = 0,
    #[doc = "1: 3.5 ADC clock cycles"]
    B_0X1 = 1,
    #[doc = "2: 7.5 ADC clock cycles"]
    B_0X2 = 2,
    #[doc = "3: 12.5 ADC clock cycles"]
    B_0X3 = 3,
    #[doc = "4: 19.5 ADC clock cycles"]
    B_0X4 = 4,
    #[doc = "5: 39.5 ADC clock cycles"]
    B_0X5 = 5,
    #[doc = "6: 79.5 ADC clock cycles"]
    B_0X6 = 6,
    #[doc = "7: 160.5 ADC clock cycles"]
    B_0X7 = 7,
}
impl From<SMP2_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP2_A {
    type Ux = u8;
}
impl SMP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP2_A {
        match self.bits {
            0 => SMP2_A::B_0X0,
            1 => SMP2_A::B_0X1,
            2 => SMP2_A::B_0X2,
            3 => SMP2_A::B_0X3,
            4 => SMP2_A::B_0X4,
            5 => SMP2_A::B_0X5,
            6 => SMP2_A::B_0X6,
            7 => SMP2_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMP2_A::B_0X0
    }
    #[doc = "3.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMP2_A::B_0X1
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SMP2_A::B_0X2
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SMP2_A::B_0X3
    }
    #[doc = "19.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == SMP2_A::B_0X4
    }
    #[doc = "39.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == SMP2_A::B_0X5
    }
    #[doc = "79.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == SMP2_A::B_0X6
    }
    #[doc = "160.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == SMP2_A::B_0X7
    }
}
#[doc = "Field `SMP2` writer - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMP2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SMP2_A>;
impl<'a, REG> SMP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2_A::B_0X0)
    }
    #[doc = "3.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2_A::B_0X1)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2_A::B_0X2)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2_A::B_0X3)
    }
    #[doc = "19.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2_A::B_0X4)
    }
    #[doc = "39.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2_A::B_0X5)
    }
    #[doc = "79.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2_A::B_0X6)
    }
    #[doc = "160.5 ADC clock cycles"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2_A::B_0X7)
    }
}
#[doc = "Field `SMPSEL0` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL0_R = crate::BitReader<SMPSEL0_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL0_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL0_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL0_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL0_A {
        match self.bits {
            false => SMPSEL0_A::B_0X0,
            true => SMPSEL0_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL0_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL0_A::B_0X1
    }
}
#[doc = "Field `SMPSEL0` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL0_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL0_A>;
impl<'a, REG> SMPSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL0_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL0_A::B_0X1)
    }
}
#[doc = "Field `SMPSEL1` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL1_R = crate::BitReader<SMPSEL1_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL1_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL1_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL1_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL1_A {
        match self.bits {
            false => SMPSEL1_A::B_0X0,
            true => SMPSEL1_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL1_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL1_A::B_0X1
    }
}
#[doc = "Field `SMPSEL1` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL1_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL1_A>;
impl<'a, REG> SMPSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL1_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL1_A::B_0X1)
    }
}
#[doc = "Field `SMPSEL2` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL2_R = crate::BitReader<SMPSEL2_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL2_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL2_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL2_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL2_A {
        match self.bits {
            false => SMPSEL2_A::B_0X0,
            true => SMPSEL2_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL2_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL2_A::B_0X1
    }
}
#[doc = "Field `SMPSEL2` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL2_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL2_A>;
impl<'a, REG> SMPSEL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL2_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL2_A::B_0X1)
    }
}
#[doc = "Field `SMPSEL3` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL3_R = crate::BitReader<SMPSEL3_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL3_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL3_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL3_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL3_A {
        match self.bits {
            false => SMPSEL3_A::B_0X0,
            true => SMPSEL3_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL3_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL3_A::B_0X1
    }
}
#[doc = "Field `SMPSEL3` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL3_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL3_A>;
impl<'a, REG> SMPSEL3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL3_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL3_A::B_0X1)
    }
}
#[doc = "Field `SMPSEL4` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL4_R = crate::BitReader<SMPSEL4_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL4_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL4_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL4_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL4_A {
        match self.bits {
            false => SMPSEL4_A::B_0X0,
            true => SMPSEL4_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL4_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL4_A::B_0X1
    }
}
#[doc = "Field `SMPSEL4` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL4_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL4_A>;
impl<'a, REG> SMPSEL4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL4_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL4_A::B_0X1)
    }
}
#[doc = "Field `SMPSEL5` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL5_R = crate::BitReader<SMPSEL5_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL5_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL5_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL5_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL5_A {
        match self.bits {
            false => SMPSEL5_A::B_0X0,
            true => SMPSEL5_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL5_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL5_A::B_0X1
    }
}
#[doc = "Field `SMPSEL5` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL5_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL5_A>;
impl<'a, REG> SMPSEL5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL5_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL5_A::B_0X1)
    }
}
#[doc = "Field `SMPSEL6` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL6_R = crate::BitReader<SMPSEL6_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL6_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL6_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL6_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL6_A {
        match self.bits {
            false => SMPSEL6_A::B_0X0,
            true => SMPSEL6_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL6_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL6_A::B_0X1
    }
}
#[doc = "Field `SMPSEL6` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL6_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL6_A>;
impl<'a, REG> SMPSEL6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL6_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL6_A::B_0X1)
    }
}
#[doc = "Field `SMPSEL7` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL7_R = crate::BitReader<SMPSEL7_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL7_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL7_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL7_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL7_A {
        match self.bits {
            false => SMPSEL7_A::B_0X0,
            true => SMPSEL7_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL7_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL7_A::B_0X1
    }
}
#[doc = "Field `SMPSEL7` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL7_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL7_A>;
impl<'a, REG> SMPSEL7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL7_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL7_A::B_0X1)
    }
}
#[doc = "Field `SMPSEL8` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL8_R = crate::BitReader<SMPSEL8_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL8_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL8_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL8_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL8_A {
        match self.bits {
            false => SMPSEL8_A::B_0X0,
            true => SMPSEL8_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL8_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL8_A::B_0X1
    }
}
#[doc = "Field `SMPSEL8` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL8_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL8_A>;
impl<'a, REG> SMPSEL8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL8_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL8_A::B_0X1)
    }
}
#[doc = "Field `SMPSEL9` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL9_R = crate::BitReader<SMPSEL9_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL9_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL9_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL9_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL9_A {
        match self.bits {
            false => SMPSEL9_A::B_0X0,
            true => SMPSEL9_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL9_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL9_A::B_0X1
    }
}
#[doc = "Field `SMPSEL9` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL9_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL9_A>;
impl<'a, REG> SMPSEL9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL9_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL9_A::B_0X1)
    }
}
#[doc = "Field `SMPSEL10` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL10_R = crate::BitReader<SMPSEL10_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL10_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL10_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL10_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL10_A {
        match self.bits {
            false => SMPSEL10_A::B_0X0,
            true => SMPSEL10_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL10_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL10_A::B_0X1
    }
}
#[doc = "Field `SMPSEL10` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL10_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL10_A>;
impl<'a, REG> SMPSEL10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL10_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL10_A::B_0X1)
    }
}
#[doc = "Field `SMPSEL11` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL11_R = crate::BitReader<SMPSEL11_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL11_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL11_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL11_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL11_A {
        match self.bits {
            false => SMPSEL11_A::B_0X0,
            true => SMPSEL11_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL11_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL11_A::B_0X1
    }
}
#[doc = "Field `SMPSEL11` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL11_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL11_A>;
impl<'a, REG> SMPSEL11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL11_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL11_A::B_0X1)
    }
}
#[doc = "Field `SMPSEL12` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL12_R = crate::BitReader<SMPSEL12_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL12_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL12_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL12_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL12_A {
        match self.bits {
            false => SMPSEL12_A::B_0X0,
            true => SMPSEL12_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL12_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL12_A::B_0X1
    }
}
#[doc = "Field `SMPSEL12` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL12_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL12_A>;
impl<'a, REG> SMPSEL12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL12_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL12_A::B_0X1)
    }
}
#[doc = "Field `SMPSEL13` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL13_R = crate::BitReader<SMPSEL13_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL13_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL13_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL13_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL13_A {
        match self.bits {
            false => SMPSEL13_A::B_0X0,
            true => SMPSEL13_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL13_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL13_A::B_0X1
    }
}
#[doc = "Field `SMPSEL13` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL13_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL13_A>;
impl<'a, REG> SMPSEL13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL13_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL13_A::B_0X1)
    }
}
#[doc = "Field `SMPSEL14` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL14_R = crate::BitReader<SMPSEL14_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL14_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL14_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL14_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL14_A {
        match self.bits {
            false => SMPSEL14_A::B_0X0,
            true => SMPSEL14_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL14_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL14_A::B_0X1
    }
}
#[doc = "Field `SMPSEL14` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL14_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL14_A>;
impl<'a, REG> SMPSEL14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL14_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL14_A::B_0X1)
    }
}
#[doc = "Field `SMPSEL15` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL15_R = crate::BitReader<SMPSEL15_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL15_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL15_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL15_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL15_A {
        match self.bits {
            false => SMPSEL15_A::B_0X0,
            true => SMPSEL15_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL15_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL15_A::B_0X1
    }
}
#[doc = "Field `SMPSEL15` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL15_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL15_A>;
impl<'a, REG> SMPSEL15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL15_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL15_A::B_0X1)
    }
}
#[doc = "Field `SMPSEL16` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL16_R = crate::BitReader<SMPSEL16_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL16_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL16_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL16_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL16_A {
        match self.bits {
            false => SMPSEL16_A::B_0X0,
            true => SMPSEL16_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL16_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL16_A::B_0X1
    }
}
#[doc = "Field `SMPSEL16` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL16_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL16_A>;
impl<'a, REG> SMPSEL16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL16_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL16_A::B_0X1)
    }
}
#[doc = "Field `SMPSEL17` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL17_R = crate::BitReader<SMPSEL17_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL17_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL17_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL17_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL17_A {
        match self.bits {
            false => SMPSEL17_A::B_0X0,
            true => SMPSEL17_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL17_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL17_A::B_0X1
    }
}
#[doc = "Field `SMPSEL17` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL17_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL17_A>;
impl<'a, REG> SMPSEL17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL17_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL17_A::B_0X1)
    }
}
#[doc = "Field `SMPSEL18` reader - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL18_R = crate::BitReader<SMPSEL18_A>;
#[doc = "Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSEL18_A {
    #[doc = "0: Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    B_0X0 = 0,
    #[doc = "1: Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    B_0X1 = 1,
}
impl From<SMPSEL18_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSEL18_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSEL18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSEL18_A {
        match self.bits {
            false => SMPSEL18_A::B_0X0,
            true => SMPSEL18_A::B_0X1,
        }
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMPSEL18_A::B_0X0
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMPSEL18_A::B_0X1
    }
}
#[doc = "Field `SMPSEL18` writer - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type SMPSEL18_W<'a, REG> = crate::BitWriter<'a, REG, SMPSEL18_A>;
impl<'a, REG> SMPSEL18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling time of CHANNELx use the setting of SMP1\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL18_A::B_0X0)
    }
    #[doc = "Sampling time of CHANNELx use the setting of SMP2\\[2:0\\]
register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPSEL18_A::B_0X1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel0(&self) -> SMPSEL0_R {
        SMPSEL0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel1(&self) -> SMPSEL1_R {
        SMPSEL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel2(&self) -> SMPSEL2_R {
        SMPSEL2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel3(&self) -> SMPSEL3_R {
        SMPSEL3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel4(&self) -> SMPSEL4_R {
        SMPSEL4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel5(&self) -> SMPSEL5_R {
        SMPSEL5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel6(&self) -> SMPSEL6_R {
        SMPSEL6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel7(&self) -> SMPSEL7_R {
        SMPSEL7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel8(&self) -> SMPSEL8_R {
        SMPSEL8_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel9(&self) -> SMPSEL9_R {
        SMPSEL9_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel10(&self) -> SMPSEL10_R {
        SMPSEL10_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel11(&self) -> SMPSEL11_R {
        SMPSEL11_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel12(&self) -> SMPSEL12_R {
        SMPSEL12_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel13(&self) -> SMPSEL13_R {
        SMPSEL13_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel14(&self) -> SMPSEL14_R {
        SMPSEL14_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel15(&self) -> SMPSEL15_R {
        SMPSEL15_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel16(&self) -> SMPSEL16_R {
        SMPSEL16_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel17(&self) -> SMPSEL17_R {
        SMPSEL17_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn smpsel18(&self) -> SMPSEL18_R {
        SMPSEL18_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sampling time selection 1 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smp1(&mut self) -> SMP1_W<ADC_SMPR_SPEC> {
        SMP1_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Sampling time selection 2 These bits are written by software to select the sampling time that applies to all channels. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smp2(&mut self) -> SMP2_W<ADC_SMPR_SPEC> {
        SMP2_W::new(self, 4)
    }
    #[doc = "Bit 8 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel0(&mut self) -> SMPSEL0_W<ADC_SMPR_SPEC> {
        SMPSEL0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel1(&mut self) -> SMPSEL1_W<ADC_SMPR_SPEC> {
        SMPSEL1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel2(&mut self) -> SMPSEL2_W<ADC_SMPR_SPEC> {
        SMPSEL2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel3(&mut self) -> SMPSEL3_W<ADC_SMPR_SPEC> {
        SMPSEL3_W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel4(&mut self) -> SMPSEL4_W<ADC_SMPR_SPEC> {
        SMPSEL4_W::new(self, 12)
    }
    #[doc = "Bit 13 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel5(&mut self) -> SMPSEL5_W<ADC_SMPR_SPEC> {
        SMPSEL5_W::new(self, 13)
    }
    #[doc = "Bit 14 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel6(&mut self) -> SMPSEL6_W<ADC_SMPR_SPEC> {
        SMPSEL6_W::new(self, 14)
    }
    #[doc = "Bit 15 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel7(&mut self) -> SMPSEL7_W<ADC_SMPR_SPEC> {
        SMPSEL7_W::new(self, 15)
    }
    #[doc = "Bit 16 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel8(&mut self) -> SMPSEL8_W<ADC_SMPR_SPEC> {
        SMPSEL8_W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel9(&mut self) -> SMPSEL9_W<ADC_SMPR_SPEC> {
        SMPSEL9_W::new(self, 17)
    }
    #[doc = "Bit 18 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel10(&mut self) -> SMPSEL10_W<ADC_SMPR_SPEC> {
        SMPSEL10_W::new(self, 18)
    }
    #[doc = "Bit 19 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel11(&mut self) -> SMPSEL11_W<ADC_SMPR_SPEC> {
        SMPSEL11_W::new(self, 19)
    }
    #[doc = "Bit 20 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel12(&mut self) -> SMPSEL12_W<ADC_SMPR_SPEC> {
        SMPSEL12_W::new(self, 20)
    }
    #[doc = "Bit 21 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel13(&mut self) -> SMPSEL13_W<ADC_SMPR_SPEC> {
        SMPSEL13_W::new(self, 21)
    }
    #[doc = "Bit 22 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel14(&mut self) -> SMPSEL14_W<ADC_SMPR_SPEC> {
        SMPSEL14_W::new(self, 22)
    }
    #[doc = "Bit 23 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel15(&mut self) -> SMPSEL15_W<ADC_SMPR_SPEC> {
        SMPSEL15_W::new(self, 23)
    }
    #[doc = "Bit 24 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel16(&mut self) -> SMPSEL16_W<ADC_SMPR_SPEC> {
        SMPSEL16_W::new(self, 24)
    }
    #[doc = "Bit 25 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel17(&mut self) -> SMPSEL17_W<ADC_SMPR_SPEC> {
        SMPSEL17_W::new(self, 25)
    }
    #[doc = "Bit 26 - Channel-x sampling time selection These bits are written by software to define which sampling time is used. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn smpsel18(&mut self) -> SMPSEL18_W<ADC_SMPR_SPEC> {
        SMPSEL18_W::new(self, 26)
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
#[doc = "ADC sampling time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_smpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_smpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_SMPR_SPEC;
impl crate::RegisterSpec for ADC_SMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_smpr::R`](R) reader structure"]
impl crate::Readable for ADC_SMPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc_smpr::W`](W) writer structure"]
impl crate::Writable for ADC_SMPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_SMPR to value 0"]
impl crate::Resettable for ADC_SMPR_SPEC {
    const RESET_VALUE: u32 = 0;
}
