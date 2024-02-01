#[doc = "Register `DMAMUX_RGCFR` writer"]
pub type W = crate::W<DMAMUX_RGCFR_SPEC>;
#[doc = "Field `COF0` writer - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register."]
pub type COF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COF1` writer - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register."]
pub type COF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COF2` writer - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register."]
pub type COF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COF3` writer - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register."]
pub type COF3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof0(&mut self) -> COF0_W<DMAMUX_RGCFR_SPEC> {
        COF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof1(&mut self) -> COF1_W<DMAMUX_RGCFR_SPEC> {
        COF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof2(&mut self) -> COF2_W<DMAMUX_RGCFR_SPEC> {
        COF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear trigger overrun event flag Writing 1 in each bit clears the corresponding overrun flag OFx in the DMAMUX_RGSR register."]
    #[inline(always)]
    #[must_use]
    pub fn cof3(&mut self) -> COF3_W<DMAMUX_RGCFR_SPEC> {
        COF3_W::new(self, 3)
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
#[doc = "DMAMUX request generator interrupt clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rgcfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMUX_RGCFR_SPEC;
impl crate::RegisterSpec for DMAMUX_RGCFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmamux_rgcfr::W`](W) writer structure"]
impl crate::Writable for DMAMUX_RGCFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAMUX_RGCFR to value 0"]
impl crate::Resettable for DMAMUX_RGCFR_SPEC {
    const RESET_VALUE: u32 = 0;
}
