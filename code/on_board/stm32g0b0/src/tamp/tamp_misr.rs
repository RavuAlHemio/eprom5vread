#[doc = "Register `TAMP_MISR` reader"]
pub type R = crate::R<TAMP_MISR_SPEC>;
#[doc = "Field `TAMP1MF` reader - TAMP1 interrupt masked flag This flag is set by hardware when the tamper 1 interrupt is raised."]
pub type TAMP1MF_R = crate::BitReader;
#[doc = "Field `TAMP2MF` reader - TAMP2 interrupt masked flag This flag is set by hardware when the tamper 2 interrupt is raised."]
pub type TAMP2MF_R = crate::BitReader;
#[doc = "Field `TAMP3MF` reader - TAMP3 interrupt masked flag This flag is set by hardware when the tamper 3 interrupt is raised."]
pub type TAMP3MF_R = crate::BitReader;
#[doc = "Field `ITAMP3MF` reader - LSE monitoring tamper interrupt masked flag This flag is set by hardware when the internal tamper 3 interrupt is raised."]
pub type ITAMP3MF_R = crate::BitReader;
#[doc = "Field `ITAMP4MF` reader - HSE monitoring tamper interrupt masked flag This flag is set by hardware when the internal tamper 4 interrupt is raised."]
pub type ITAMP4MF_R = crate::BitReader;
#[doc = "Field `ITAMP5MF` reader - RTC calendar overflow tamper interrupt masked flag This flag is set by hardware when the internal tamper 5 interrupt is raised."]
pub type ITAMP5MF_R = crate::BitReader;
#[doc = "Field `ITAMP6MF` reader - ST manufacturer readout tamper interrupt masked flag This flag is set by hardware when the internal tamper 6 interrupt is raised."]
pub type ITAMP6MF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TAMP1 interrupt masked flag This flag is set by hardware when the tamper 1 interrupt is raised."]
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2 interrupt masked flag This flag is set by hardware when the tamper 2 interrupt is raised."]
    #[inline(always)]
    pub fn tamp2mf(&self) -> TAMP2MF_R {
        TAMP2MF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP3 interrupt masked flag This flag is set by hardware when the tamper 3 interrupt is raised."]
    #[inline(always)]
    pub fn tamp3mf(&self) -> TAMP3MF_R {
        TAMP3MF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - LSE monitoring tamper interrupt masked flag This flag is set by hardware when the internal tamper 3 interrupt is raised."]
    #[inline(always)]
    pub fn itamp3mf(&self) -> ITAMP3MF_R {
        ITAMP3MF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSE monitoring tamper interrupt masked flag This flag is set by hardware when the internal tamper 4 interrupt is raised."]
    #[inline(always)]
    pub fn itamp4mf(&self) -> ITAMP4MF_R {
        ITAMP4MF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RTC calendar overflow tamper interrupt masked flag This flag is set by hardware when the internal tamper 5 interrupt is raised."]
    #[inline(always)]
    pub fn itamp5mf(&self) -> ITAMP5MF_R {
        ITAMP5MF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ST manufacturer readout tamper interrupt masked flag This flag is set by hardware when the internal tamper 6 interrupt is raised."]
    #[inline(always)]
    pub fn itamp6mf(&self) -> ITAMP6MF_R {
        ITAMP6MF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "TAMP masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_misr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAMP_MISR_SPEC;
impl crate::RegisterSpec for TAMP_MISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp_misr::R`](R) reader structure"]
impl crate::Readable for TAMP_MISR_SPEC {}
#[doc = "`reset()` method sets TAMP_MISR to value 0"]
impl crate::Resettable for TAMP_MISR_SPEC {
    const RESET_VALUE: u32 = 0;
}
