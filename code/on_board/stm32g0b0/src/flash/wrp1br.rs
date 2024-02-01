#[doc = "Register `WRP1BR` reader"]
pub type R = crate::R<WRP1BR_SPEC>;
#[doc = "Field `WRP1B_STRT` reader - WRP area B start offset"]
pub type WRP1B_STRT_R = crate::FieldReader;
#[doc = "Field `WRP1B_END` reader - WRP area B end offset"]
pub type WRP1B_END_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - WRP area B start offset"]
    #[inline(always)]
    pub fn wrp1b_strt(&self) -> WRP1B_STRT_R {
        WRP1B_STRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - WRP area B end offset"]
    #[inline(always)]
    pub fn wrp1b_end(&self) -> WRP1B_END_R {
        WRP1B_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "Flash WRP area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp1br::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRP1BR_SPEC;
impl crate::RegisterSpec for WRP1BR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp1br::R`](R) reader structure"]
impl crate::Readable for WRP1BR_SPEC {}
#[doc = "`reset()` method sets WRP1BR to value 0xf000_0000"]
impl crate::Resettable for WRP1BR_SPEC {
    const RESET_VALUE: u32 = 0xf000_0000;
}
