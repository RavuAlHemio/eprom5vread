#[doc = "Register `ITLINE8` reader"]
pub type R = crate::R<ITLINE8_SPEC>;
#[doc = "Field `USB` reader - USB"]
pub type USB_R = crate::BitReader;
impl R {
    #[doc = "Bit 2 - USB"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "interrupt line 8 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE8_SPEC;
impl crate::RegisterSpec for ITLINE8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline8::R`](R) reader structure"]
impl crate::Readable for ITLINE8_SPEC {}
#[doc = "`reset()` method sets ITLINE8 to value 0"]
impl crate::Resettable for ITLINE8_SPEC {
    const RESET_VALUE: u32 = 0;
}
