#[doc = "Register `ITLINE18` reader"]
pub type R = crate::R<ITLINE18_SPEC>;
#[doc = "Field `TIM7` reader - TIM7"]
pub type TIM7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM7"]
    #[inline(always)]
    pub fn tim7(&self) -> TIM7_R {
        TIM7_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 18 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline18::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE18_SPEC;
impl crate::RegisterSpec for ITLINE18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline18::R`](R) reader structure"]
impl crate::Readable for ITLINE18_SPEC {}
#[doc = "`reset()` method sets ITLINE18 to value 0"]
impl crate::Resettable for ITLINE18_SPEC {
    const RESET_VALUE: u32 = 0;
}
