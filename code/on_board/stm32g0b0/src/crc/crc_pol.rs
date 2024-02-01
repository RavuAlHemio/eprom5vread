#[doc = "Register `CRC_POL` reader"]
pub type R = crate::R<CRC_POL_SPEC>;
#[doc = "Register `CRC_POL` writer"]
pub type W = crate::W<CRC_POL_SPEC>;
#[doc = "Field `POL` reader - Programmable polynomial"]
pub type POL_R = crate::FieldReader<u32>;
#[doc = "Field `POL` writer - Programmable polynomial"]
pub type POL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Programmable polynomial"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Programmable polynomial"]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<CRC_POL_SPEC> {
        POL_W::new(self, 0)
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
#[doc = "polynomial\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_pol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_pol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRC_POL_SPEC;
impl crate::RegisterSpec for CRC_POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_pol::R`](R) reader structure"]
impl crate::Readable for CRC_POL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crc_pol::W`](W) writer structure"]
impl crate::Writable for CRC_POL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_POL to value 0x04c1_1db7"]
impl crate::Resettable for CRC_POL_SPEC {
    const RESET_VALUE: u32 = 0x04c1_1db7;
}
