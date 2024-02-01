#[doc = "Register `TIM1_ARR` reader"]
pub type R = crate::R<TIM1_ARR_SPEC>;
#[doc = "Register `TIM1_ARR` writer"]
pub type W = crate::W<TIM1_ARR_SPEC>;
#[doc = "Field `ARR` reader - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null."]
pub type ARR_R = crate::FieldReader<u16>;
#[doc = "Field `ARR` writer - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null."]
pub type ARR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null."]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Auto-reload value ARR is the value to be loaded in the actual auto-reload register. Refer to the for more details about ARR update and behavior. The counter is blocked while the auto-reload value is null."]
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ARR_W<TIM1_ARR_SPEC> {
        ARR_W::new(self, 0)
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
#[doc = "auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_arr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_arr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_ARR_SPEC;
impl crate::RegisterSpec for TIM1_ARR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_arr::R`](R) reader structure"]
impl crate::Readable for TIM1_ARR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tim1_arr::W`](W) writer structure"]
impl crate::Writable for TIM1_ARR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_ARR to value 0xffff"]
impl crate::Resettable for TIM1_ARR_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
