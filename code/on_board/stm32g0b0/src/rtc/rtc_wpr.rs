#[doc = "Register `RTC_WPR` writer"]
pub type W = crate::W<RTC_WPR_SPEC>;
#[doc = "Field `KEY` writer - Write protection key This byte is written by software. Reading this byte always returns 0x00. Refer to for a description of how to unlock RTC register write protection."]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Write protection key This byte is written by software. Reading this byte always returns 0x00. Refer to for a description of how to unlock RTC register write protection."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<RTC_WPR_SPEC> {
        KEY_W::new(self, 0)
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
#[doc = "write protection register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_wpr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_WPR_SPEC;
impl crate::RegisterSpec for RTC_WPR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rtc_wpr::W`](W) writer structure"]
impl crate::Writable for RTC_WPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_WPR to value 0"]
impl crate::Resettable for RTC_WPR_SPEC {
    const RESET_VALUE: u32 = 0;
}
