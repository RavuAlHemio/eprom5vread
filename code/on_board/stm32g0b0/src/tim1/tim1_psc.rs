#[doc = "Register `TIM1_PSC` reader"]
pub type R = crate::R<TIM1_PSC_SPEC>;
#[doc = "Register `TIM1_PSC` writer"]
pub type W = crate::W<TIM1_PSC_SPEC>;
#[doc = "Field `PSC` reader - Prescaler value The counter clock frequency (CK_CNT) is equal to fCK_PSC / (PSC\\[15:0\\]
+ 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in 'reset mode')."]
pub type PSC_R = crate::FieldReader<u16>;
#[doc = "Field `PSC` writer - Prescaler value The counter clock frequency (CK_CNT) is equal to fCK_PSC / (PSC\\[15:0\\]
+ 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in 'reset mode')."]
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Prescaler value The counter clock frequency (CK_CNT) is equal to fCK_PSC / (PSC\\[15:0\\]
+ 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in 'reset mode')."]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Prescaler value The counter clock frequency (CK_CNT) is equal to fCK_PSC / (PSC\\[15:0\\]
+ 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in 'reset mode')."]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<TIM1_PSC_SPEC> {
        PSC_W::new(self, 0)
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
#[doc = "prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_psc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_psc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_PSC_SPEC;
impl crate::RegisterSpec for TIM1_PSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_psc::R`](R) reader structure"]
impl crate::Readable for TIM1_PSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tim1_psc::W`](W) writer structure"]
impl crate::Writable for TIM1_PSC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_PSC to value 0"]
impl crate::Resettable for TIM1_PSC_SPEC {
    const RESET_VALUE: u32 = 0;
}
