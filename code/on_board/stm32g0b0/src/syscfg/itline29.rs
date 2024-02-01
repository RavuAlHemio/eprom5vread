#[doc = "Register `ITLINE29` reader"]
pub type R = crate::R<ITLINE29_SPEC>;
#[doc = "Field `USART3` reader - USART3"]
pub type USART3_R = crate::BitReader;
#[doc = "Field `USART4` reader - USART4"]
pub type USART4_R = crate::BitReader;
#[doc = "Field `USART5` reader - USART5"]
pub type USART5_R = crate::BitReader;
#[doc = "Field `USART6` reader - USART6"]
pub type USART6_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USART3"]
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USART4"]
    #[inline(always)]
    pub fn usart4(&self) -> USART4_R {
        USART4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - USART5"]
    #[inline(always)]
    pub fn usart5(&self) -> USART5_R {
        USART5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USART6"]
    #[inline(always)]
    pub fn usart6(&self) -> USART6_R {
        USART6_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "interrupt line 29 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline29::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE29_SPEC;
impl crate::RegisterSpec for ITLINE29_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline29::R`](R) reader structure"]
impl crate::Readable for ITLINE29_SPEC {}
#[doc = "`reset()` method sets ITLINE29 to value 0"]
impl crate::Resettable for ITLINE29_SPEC {
    const RESET_VALUE: u32 = 0;
}
