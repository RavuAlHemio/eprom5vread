#[doc = "Register `CRC_DR` reader"]
pub type R = crate::R<CRC_DR_SPEC>;
#[doc = "Register `CRC_DR` writer"]
pub type W = crate::W<CRC_DR_SPEC>;
#[doc = "Field `DR` reader - Data register bits"]
pub type DR_R = crate::FieldReader<u32>;
#[doc = "Field `DR` writer - Data register bits"]
pub type DR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data register bits"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data register bits"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DR_W<CRC_DR_SPEC> {
        DR_W::new(self, 0)
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
#[doc = "Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRC_DR_SPEC;
impl crate::RegisterSpec for CRC_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_dr::R`](R) reader structure"]
impl crate::Readable for CRC_DR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crc_dr::W`](W) writer structure"]
impl crate::Writable for CRC_DR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_DR to value 0xffff_ffff"]
impl crate::Resettable for CRC_DR_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
