#[doc = "Register `TIM1_RCR` reader"]
pub type R = crate::R<TIM1_RCR_SPEC>;
#[doc = "Register `TIM1_RCR` writer"]
pub type W = crate::W<TIM1_RCR_SPEC>;
#[doc = "Field `REP` reader - Repetition counter value"]
pub type REP_R = crate::FieldReader<u16>;
#[doc = "Field `REP` writer - Repetition counter value"]
pub type REP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Repetition counter value"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Repetition counter value"]
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> REP_W<TIM1_RCR_SPEC> {
        REP_W::new(self, 0)
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
#[doc = "repetition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_rcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_rcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_RCR_SPEC;
impl crate::RegisterSpec for TIM1_RCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_rcr::R`](R) reader structure"]
impl crate::Readable for TIM1_RCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tim1_rcr::W`](W) writer structure"]
impl crate::Writable for TIM1_RCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_RCR to value 0"]
impl crate::Resettable for TIM1_RCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
