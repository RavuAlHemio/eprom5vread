#[doc = "Register `WWDG_SR` reader"]
pub type R = crate::R<WWDG_SR_SPEC>;
#[doc = "Register `WWDG_SR` writer"]
pub type W = crate::W<WWDG_SR_SPEC>;
#[doc = "Field `EWIF` reader - Early wakeup interrupt flag"]
pub type EWIF_R = crate::BitReader;
#[doc = "Field `EWIF` writer - Early wakeup interrupt flag"]
pub type EWIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Early wakeup interrupt flag"]
    #[inline(always)]
    pub fn ewif(&self) -> EWIF_R {
        EWIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Early wakeup interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ewif(&mut self) -> EWIF_W<WWDG_SR_SPEC> {
        EWIF_W::new(self, 0)
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
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wwdg_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WWDG_SR_SPEC;
impl crate::RegisterSpec for WWDG_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdg_sr::R`](R) reader structure"]
impl crate::Readable for WWDG_SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wwdg_sr::W`](W) writer structure"]
impl crate::Writable for WWDG_SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WWDG_SR to value 0"]
impl crate::Resettable for WWDG_SR_SPEC {
    const RESET_VALUE: u32 = 0;
}
