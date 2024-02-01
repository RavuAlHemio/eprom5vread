#[doc = "Register `TIM1_CCR3` reader"]
pub type R = crate::R<TIM1_CCR3_SPEC>;
#[doc = "Register `TIM1_CCR3` writer"]
pub type W = crate::W<TIM1_CCR3_SPEC>;
#[doc = "Field `CCR3` reader - Capture/Compare value"]
pub type CCR3_R = crate::FieldReader<u16>;
#[doc = "Field `CCR3` writer - Capture/Compare value"]
pub type CCR3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    pub fn ccr3(&self) -> CCR3_R {
        CCR3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr3(&mut self) -> CCR3_W<TIM1_CCR3_SPEC> {
        CCR3_W::new(self, 0)
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
#[doc = "capture/compare register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_CCR3_SPEC;
impl crate::RegisterSpec for TIM1_CCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_ccr3::R`](R) reader structure"]
impl crate::Readable for TIM1_CCR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tim1_ccr3::W`](W) writer structure"]
impl crate::Writable for TIM1_CCR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_CCR3 to value 0"]
impl crate::Resettable for TIM1_CCR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
