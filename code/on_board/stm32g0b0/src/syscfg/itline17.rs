#[doc = "Register `ITLINE17` reader"]
pub type R = crate::R<ITLINE17_SPEC>;
#[doc = "Field `TIM6` reader - TIM6"]
pub type TIM6_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM6"]
    #[inline(always)]
    pub fn tim6(&self) -> TIM6_R {
        TIM6_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 17 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline17::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE17_SPEC;
impl crate::RegisterSpec for ITLINE17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline17::R`](R) reader structure"]
impl crate::Readable for ITLINE17_SPEC {}
#[doc = "`reset()` method sets ITLINE17 to value 0"]
impl crate::Resettable for ITLINE17_SPEC {
    const RESET_VALUE: u32 = 0;
}
