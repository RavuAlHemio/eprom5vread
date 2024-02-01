#[doc = "Register `WRP1AR` reader"]
pub type R = crate::R<WRP1AR_SPEC>;
#[doc = "Field `WRP1A_STRT` reader - WRP area A start offset"]
pub type WRP1A_STRT_R = crate::FieldReader;
#[doc = "Field `WRP1A_END` reader - WRP area A end offset"]
pub type WRP1A_END_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - WRP area A start offset"]
    #[inline(always)]
    pub fn wrp1a_strt(&self) -> WRP1A_STRT_R {
        WRP1A_STRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - WRP area A end offset"]
    #[inline(always)]
    pub fn wrp1a_end(&self) -> WRP1A_END_R {
        WRP1A_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "Flash WRP area A address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp1ar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRP1AR_SPEC;
impl crate::RegisterSpec for WRP1AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp1ar::R`](R) reader structure"]
impl crate::Readable for WRP1AR_SPEC {}
#[doc = "`reset()` method sets WRP1AR to value 0xf000_0000"]
impl crate::Resettable for WRP1AR_SPEC {
    const RESET_VALUE: u32 = 0xf000_0000;
}
