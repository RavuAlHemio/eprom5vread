#[doc = "Register `DMAMUX_CFR` writer"]
pub type W = crate::W<DMAMUX_CFR_SPEC>;
#[doc = "Field `CSOF0` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type CSOF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF1` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type CSOF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF2` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type CSOF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF3` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type CSOF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF4` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type CSOF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF5` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type CSOF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSOF6` writer - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
pub type CSOF6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof0(&mut self) -> CSOF0_W<DMAMUX_CFR_SPEC> {
        CSOF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof1(&mut self) -> CSOF1_W<DMAMUX_CFR_SPEC> {
        CSOF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof2(&mut self) -> CSOF2_W<DMAMUX_CFR_SPEC> {
        CSOF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof3(&mut self) -> CSOF3_W<DMAMUX_CFR_SPEC> {
        CSOF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof4(&mut self) -> CSOF4_W<DMAMUX_CFR_SPEC> {
        CSOF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof5(&mut self) -> CSOF5_W<DMAMUX_CFR_SPEC> {
        CSOF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear synchronization overrun event flag Writing 1 in each bit clears the corresponding overrun flag SOFx in the DMAMUX_CSR register."]
    #[inline(always)]
    #[must_use]
    pub fn csof6(&mut self) -> CSOF6_W<DMAMUX_CFR_SPEC> {
        CSOF6_W::new(self, 6)
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
#[doc = "DMAMUX request line multiplexer interrupt clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_cfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMUX_CFR_SPEC;
impl crate::RegisterSpec for DMAMUX_CFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmamux_cfr::W`](W) writer structure"]
impl crate::Writable for DMAMUX_CFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAMUX_CFR to value 0"]
impl crate::Resettable for DMAMUX_CFR_SPEC {
    const RESET_VALUE: u32 = 0;
}
