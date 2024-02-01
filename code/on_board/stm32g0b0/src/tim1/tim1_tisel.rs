#[doc = "Register `TIM1_TISEL` reader"]
pub type R = crate::R<TIM1_TISEL_SPEC>;
#[doc = "Register `TIM1_TISEL` writer"]
pub type W = crate::W<TIM1_TISEL_SPEC>;
#[doc = "Field `TI1SEL` reader - selects TI1\\[0\\]
to TI1\\[15\\]
input Others: Reserved"]
pub type TI1SEL_R = crate::FieldReader<TI1SEL_A>;
#[doc = "selects TI1\\[0\\]
to TI1\\[15\\]
input Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI1SEL_A {
    #[doc = "0: TIM1_CH1 input"]
    B_0X0 = 0,
    #[doc = "1: COMP1 output"]
    B_0X1 = 1,
}
impl From<TI1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TI1SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TI1SEL_A {
    type Ux = u8;
}
impl TI1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TI1SEL_A> {
        match self.bits {
            0 => Some(TI1SEL_A::B_0X0),
            1 => Some(TI1SEL_A::B_0X1),
            _ => None,
        }
    }
    #[doc = "TIM1_CH1 input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TI1SEL_A::B_0X0
    }
    #[doc = "COMP1 output"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TI1SEL_A::B_0X1
    }
}
#[doc = "Field `TI1SEL` writer - selects TI1\\[0\\]
to TI1\\[15\\]
input Others: Reserved"]
pub type TI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TI1SEL_A>;
impl<'a, REG> TI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM1_CH1 input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TI1SEL_A::B_0X0)
    }
    #[doc = "COMP1 output"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TI1SEL_A::B_0X1)
    }
}
#[doc = "Field `TI2SEL` reader - selects TI2\\[0\\]
to TI2\\[15\\]
input Others: Reserved"]
pub type TI2SEL_R = crate::FieldReader<TI2SEL_A>;
#[doc = "selects TI2\\[0\\]
to TI2\\[15\\]
input Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI2SEL_A {
    #[doc = "0: TIM1_CH2 input"]
    B_0X0 = 0,
    #[doc = "1: COMP2 output"]
    B_0X1 = 1,
}
impl From<TI2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TI2SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TI2SEL_A {
    type Ux = u8;
}
impl TI2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TI2SEL_A> {
        match self.bits {
            0 => Some(TI2SEL_A::B_0X0),
            1 => Some(TI2SEL_A::B_0X1),
            _ => None,
        }
    }
    #[doc = "TIM1_CH2 input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TI2SEL_A::B_0X0
    }
    #[doc = "COMP2 output"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TI2SEL_A::B_0X1
    }
}
#[doc = "Field `TI2SEL` writer - selects TI2\\[0\\]
to TI2\\[15\\]
input Others: Reserved"]
pub type TI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TI2SEL_A>;
impl<'a, REG> TI2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM1_CH2 input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TI2SEL_A::B_0X0)
    }
    #[doc = "COMP2 output"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TI2SEL_A::B_0X1)
    }
}
#[doc = "Field `TI3SEL` reader - selects TI3\\[0\\]
to TI3\\[15\\]
input Others: Reserved"]
pub type TI3SEL_R = crate::FieldReader<TI3SEL_A>;
#[doc = "selects TI3\\[0\\]
to TI3\\[15\\]
input Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI3SEL_A {
    #[doc = "0: TIM1_CH3 input"]
    B_0X0 = 0,
}
impl From<TI3SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TI3SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TI3SEL_A {
    type Ux = u8;
}
impl TI3SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TI3SEL_A> {
        match self.bits {
            0 => Some(TI3SEL_A::B_0X0),
            _ => None,
        }
    }
    #[doc = "TIM1_CH3 input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TI3SEL_A::B_0X0
    }
}
#[doc = "Field `TI3SEL` writer - selects TI3\\[0\\]
to TI3\\[15\\]
input Others: Reserved"]
pub type TI3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TI3SEL_A>;
impl<'a, REG> TI3SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM1_CH3 input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TI3SEL_A::B_0X0)
    }
}
#[doc = "Field `TI4SEL` reader - selects TI4\\[0\\]
to TI4\\[15\\]
input Others: Reserved"]
pub type TI4SEL_R = crate::FieldReader<TI4SEL_A>;
#[doc = "selects TI4\\[0\\]
to TI4\\[15\\]
input Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI4SEL_A {
    #[doc = "0: TIM1_CH4 input"]
    B_0X0 = 0,
}
impl From<TI4SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TI4SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TI4SEL_A {
    type Ux = u8;
}
impl TI4SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TI4SEL_A> {
        match self.bits {
            0 => Some(TI4SEL_A::B_0X0),
            _ => None,
        }
    }
    #[doc = "TIM1_CH4 input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TI4SEL_A::B_0X0
    }
}
#[doc = "Field `TI4SEL` writer - selects TI4\\[0\\]
to TI4\\[15\\]
input Others: Reserved"]
pub type TI4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TI4SEL_A>;
impl<'a, REG> TI4SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM1_CH4 input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TI4SEL_A::B_0X0)
    }
}
impl R {
    #[doc = "Bits 0:3 - selects TI1\\[0\\]
to TI1\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - selects TI2\\[0\\]
to TI2\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    pub fn ti2sel(&self) -> TI2SEL_R {
        TI2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - selects TI3\\[0\\]
to TI3\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    pub fn ti3sel(&self) -> TI3SEL_R {
        TI3SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - selects TI4\\[0\\]
to TI4\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    pub fn ti4sel(&self) -> TI4SEL_R {
        TI4SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - selects TI1\\[0\\]
to TI1\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ti1sel(&mut self) -> TI1SEL_W<TIM1_TISEL_SPEC> {
        TI1SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - selects TI2\\[0\\]
to TI2\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ti2sel(&mut self) -> TI2SEL_W<TIM1_TISEL_SPEC> {
        TI2SEL_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - selects TI3\\[0\\]
to TI3\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ti3sel(&mut self) -> TI3SEL_W<TIM1_TISEL_SPEC> {
        TI3SEL_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - selects TI4\\[0\\]
to TI4\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ti4sel(&mut self) -> TI4SEL_W<TIM1_TISEL_SPEC> {
        TI4SEL_W::new(self, 24)
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
#[doc = "TIM1 timer input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_tisel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_tisel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_TISEL_SPEC;
impl crate::RegisterSpec for TIM1_TISEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_tisel::R`](R) reader structure"]
impl crate::Readable for TIM1_TISEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tim1_tisel::W`](W) writer structure"]
impl crate::Writable for TIM1_TISEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_TISEL to value 0"]
impl crate::Resettable for TIM1_TISEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
