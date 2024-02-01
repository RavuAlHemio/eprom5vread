#[doc = "Register `EXTICR2` reader"]
pub type R = crate::R<EXTICR2_SPEC>;
#[doc = "Register `EXTICR2` writer"]
pub type W = crate::W<EXTICR2_SPEC>;
#[doc = "Field `EXTI0_7` reader - GPIO port selection"]
pub type EXTI0_7_R = crate::FieldReader;
#[doc = "Field `EXTI0_7` writer - GPIO port selection"]
pub type EXTI0_7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI8_15` reader - GPIO port selection"]
pub type EXTI8_15_R = crate::FieldReader;
#[doc = "Field `EXTI8_15` writer - GPIO port selection"]
pub type EXTI8_15_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI16_23` reader - GPIO port selection"]
pub type EXTI16_23_R = crate::FieldReader;
#[doc = "Field `EXTI16_23` writer - GPIO port selection"]
pub type EXTI16_23_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI24_31` reader - GPIO port selection"]
pub type EXTI24_31_R = crate::FieldReader;
#[doc = "Field `EXTI24_31` writer - GPIO port selection"]
pub type EXTI24_31_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO port selection"]
    #[inline(always)]
    pub fn exti0_7(&self) -> EXTI0_7_R {
        EXTI0_7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO port selection"]
    #[inline(always)]
    pub fn exti8_15(&self) -> EXTI8_15_R {
        EXTI8_15_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO port selection"]
    #[inline(always)]
    pub fn exti16_23(&self) -> EXTI16_23_R {
        EXTI16_23_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO port selection"]
    #[inline(always)]
    pub fn exti24_31(&self) -> EXTI24_31_R {
        EXTI24_31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti0_7(&mut self) -> EXTI0_7_W<EXTICR2_SPEC> {
        EXTI0_7_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti8_15(&mut self) -> EXTI8_15_W<EXTICR2_SPEC> {
        EXTI8_15_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti16_23(&mut self) -> EXTI16_23_W<EXTICR2_SPEC> {
        EXTI16_23_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti24_31(&mut self) -> EXTI24_31_W<EXTICR2_SPEC> {
        EXTI24_31_W::new(self, 24)
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
#[doc = "EXTI external interrupt selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTICR2_SPEC;
impl crate::RegisterSpec for EXTICR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr2::R`](R) reader structure"]
impl crate::Readable for EXTICR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exticr2::W`](W) writer structure"]
impl crate::Writable for EXTICR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTICR2 to value 0"]
impl crate::Resettable for EXTICR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
