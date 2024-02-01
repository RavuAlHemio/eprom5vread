#[doc = "Register `WRP2BR` reader"]
pub type R = crate::R<WRP2BR_SPEC>;
#[doc = "Register `WRP2BR` writer"]
pub type W = crate::W<WRP2BR_SPEC>;
#[doc = "Field `WRP2B_STRT` reader - WRP2B_STRT"]
pub type WRP2B_STRT_R = crate::FieldReader;
#[doc = "Field `WRP2B_STRT` writer - WRP2B_STRT"]
pub type WRP2B_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRP2B_END` reader - WRP2B_END"]
pub type WRP2B_END_R = crate::FieldReader;
#[doc = "Field `WRP2B_END` writer - WRP2B_END"]
pub type WRP2B_END_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - WRP2B_STRT"]
    #[inline(always)]
    pub fn wrp2b_strt(&self) -> WRP2B_STRT_R {
        WRP2B_STRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - WRP2B_END"]
    #[inline(always)]
    pub fn wrp2b_end(&self) -> WRP2B_END_R {
        WRP2B_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - WRP2B_STRT"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2b_strt(&mut self) -> WRP2B_STRT_W<WRP2BR_SPEC> {
        WRP2B_STRT_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - WRP2B_END"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2b_end(&mut self) -> WRP2B_END_W<WRP2BR_SPEC> {
        WRP2B_END_W::new(self, 16)
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
#[doc = "FLASH WRP2 area B address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrp2br::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrp2br::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRP2BR_SPEC;
impl crate::RegisterSpec for WRP2BR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp2br::R`](R) reader structure"]
impl crate::Readable for WRP2BR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wrp2br::W`](W) writer structure"]
impl crate::Writable for WRP2BR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRP2BR to value 0"]
impl crate::Resettable for WRP2BR_SPEC {
    const RESET_VALUE: u32 = 0;
}
