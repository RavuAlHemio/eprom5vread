#[doc = "Register `TIM1_OR1` reader"]
pub type R = crate::R<TIM1_OR1_SPEC>;
#[doc = "Register `TIM1_OR1` writer"]
pub type W = crate::W<TIM1_OR1_SPEC>;
#[doc = "Field `OCREF_CLR` reader - Ocref_clr source selection This bit selects the ocref_clr input source."]
pub type OCREF_CLR_R = crate::BitReader<OCREF_CLR_A>;
#[doc = "Ocref_clr source selection This bit selects the ocref_clr input source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCREF_CLR_A {
    #[doc = "0: COMP1 output is connected to the OCREF_CLR input"]
    B_0X0 = 0,
    #[doc = "1: COMP2 output is connected to the OCREF_CLR input"]
    B_0X1 = 1,
}
impl From<OCREF_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: OCREF_CLR_A) -> Self {
        variant as u8 != 0
    }
}
impl OCREF_CLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OCREF_CLR_A {
        match self.bits {
            false => OCREF_CLR_A::B_0X0,
            true => OCREF_CLR_A::B_0X1,
        }
    }
    #[doc = "COMP1 output is connected to the OCREF_CLR input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OCREF_CLR_A::B_0X0
    }
    #[doc = "COMP2 output is connected to the OCREF_CLR input"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OCREF_CLR_A::B_0X1
    }
}
#[doc = "Field `OCREF_CLR` writer - Ocref_clr source selection This bit selects the ocref_clr input source."]
pub type OCREF_CLR_W<'a, REG> = crate::BitWriter<'a, REG, OCREF_CLR_A>;
impl<'a, REG> OCREF_CLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP1 output is connected to the OCREF_CLR input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OCREF_CLR_A::B_0X0)
    }
    #[doc = "COMP2 output is connected to the OCREF_CLR input"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OCREF_CLR_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - Ocref_clr source selection This bit selects the ocref_clr input source."]
    #[inline(always)]
    pub fn ocref_clr(&self) -> OCREF_CLR_R {
        OCREF_CLR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ocref_clr source selection This bit selects the ocref_clr input source."]
    #[inline(always)]
    #[must_use]
    pub fn ocref_clr(&mut self) -> OCREF_CLR_W<TIM1_OR1_SPEC> {
        OCREF_CLR_W::new(self, 0)
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
#[doc = "option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_or1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_or1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_OR1_SPEC;
impl crate::RegisterSpec for TIM1_OR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_or1::R`](R) reader structure"]
impl crate::Readable for TIM1_OR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tim1_or1::W`](W) writer structure"]
impl crate::Writable for TIM1_OR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_OR1 to value 0"]
impl crate::Resettable for TIM1_OR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
