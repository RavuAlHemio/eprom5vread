#[doc = "Register `TISEL` reader"]
pub type R = crate::R<TISEL_SPEC>;
#[doc = "Register `TISEL` writer"]
pub type W = crate::W<TISEL_SPEC>;
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
    #[doc = "0: TIM15_CH1 input"]
    B_0X0 = 0,
    #[doc = "1: TIM2_IC1"]
    B_0X1 = 1,
    #[doc = "2: TIM3_IC1"]
    B_0X2 = 2,
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
            2 => Some(TI1SEL_A::B_0X2),
            _ => None,
        }
    }
    #[doc = "TIM15_CH1 input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TI1SEL_A::B_0X0
    }
    #[doc = "TIM2_IC1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TI1SEL_A::B_0X1
    }
    #[doc = "TIM3_IC1"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TI1SEL_A::B_0X2
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
    #[doc = "TIM15_CH1 input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TI1SEL_A::B_0X0)
    }
    #[doc = "TIM2_IC1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TI1SEL_A::B_0X1)
    }
    #[doc = "TIM3_IC1"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TI1SEL_A::B_0X2)
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
    #[doc = "0: TIM15_CH2 input"]
    B_0X0 = 0,
    #[doc = "1: TIM2_IC2"]
    B_0X1 = 1,
    #[doc = "2: TIM3_IC2"]
    B_0X2 = 2,
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
            2 => Some(TI2SEL_A::B_0X2),
            _ => None,
        }
    }
    #[doc = "TIM15_CH2 input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TI2SEL_A::B_0X0
    }
    #[doc = "TIM2_IC2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TI2SEL_A::B_0X1
    }
    #[doc = "TIM3_IC2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TI2SEL_A::B_0X2
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
    #[doc = "TIM15_CH2 input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TI2SEL_A::B_0X0)
    }
    #[doc = "TIM2_IC2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TI2SEL_A::B_0X1)
    }
    #[doc = "TIM3_IC2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TI2SEL_A::B_0X2)
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
}
impl W {
    #[doc = "Bits 0:3 - selects TI1\\[0\\]
to TI1\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ti1sel(&mut self) -> TI1SEL_W<TISEL_SPEC> {
        TI1SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - selects TI2\\[0\\]
to TI2\\[15\\]
input Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ti2sel(&mut self) -> TI2SEL_W<TISEL_SPEC> {
        TI2SEL_W::new(self, 8)
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
#[doc = "input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tisel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tisel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TISEL_SPEC;
impl crate::RegisterSpec for TISEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tisel::R`](R) reader structure"]
impl crate::Readable for TISEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tisel::W`](W) writer structure"]
impl crate::Writable for TISEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TISEL to value 0"]
impl crate::Resettable for TISEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
