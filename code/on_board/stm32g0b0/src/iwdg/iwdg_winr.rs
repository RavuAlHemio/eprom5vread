#[doc = "Register `IWDG_WINR` reader"]
pub type R = crate::R<IWDG_WINR_SPEC>;
#[doc = "Register `IWDG_WINR` writer"]
pub type W = crate::W<IWDG_WINR_SPEC>;
#[doc = "Field `WIN` reader - Watchdog counter window value"]
pub type WIN_R = crate::FieldReader<u16>;
#[doc = "Field `WIN` writer - Watchdog counter window value"]
pub type WIN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WIN_W<IWDG_WINR_SPEC> {
        WIN_W::new(self, 0)
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
#[doc = "Window register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_winr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_winr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IWDG_WINR_SPEC;
impl crate::RegisterSpec for IWDG_WINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_winr::R`](R) reader structure"]
impl crate::Readable for IWDG_WINR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iwdg_winr::W`](W) writer structure"]
impl crate::Writable for IWDG_WINR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IWDG_WINR to value 0x0fff"]
impl crate::Resettable for IWDG_WINR_SPEC {
    const RESET_VALUE: u32 = 0x0fff;
}
