#[doc = "Register `ITLINE26` reader"]
pub type R = crate::R<ITLINE26_SPEC>;
#[doc = "Field `SPI2` reader - SPI2"]
pub type SPI2_R = crate::BitReader;
#[doc = "Field `SPI3` reader - SPI3"]
pub type SPI3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SPI2"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 14 - SPI3"]
    #[inline(always)]
    pub fn spi3(&self) -> SPI3_R {
        SPI3_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "interrupt line 26 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline26::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE26_SPEC;
impl crate::RegisterSpec for ITLINE26_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline26::R`](R) reader structure"]
impl crate::Readable for ITLINE26_SPEC {}
#[doc = "`reset()` method sets ITLINE26 to value 0"]
impl crate::Resettable for ITLINE26_SPEC {
    const RESET_VALUE: u32 = 0;
}
