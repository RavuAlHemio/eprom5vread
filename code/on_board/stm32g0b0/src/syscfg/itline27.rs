#[doc = "Register `ITLINE27` reader"]
pub type R = crate::R<ITLINE27_SPEC>;
#[doc = "Field `USART1` reader - USART1"]
pub type USART1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USART1"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 27 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline27::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE27_SPEC;
impl crate::RegisterSpec for ITLINE27_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline27::R`](R) reader structure"]
impl crate::Readable for ITLINE27_SPEC {}
#[doc = "`reset()` method sets ITLINE27 to value 0"]
impl crate::Resettable for ITLINE27_SPEC {
    const RESET_VALUE: u32 = 0;
}
