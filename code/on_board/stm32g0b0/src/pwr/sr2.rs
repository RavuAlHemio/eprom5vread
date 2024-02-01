#[doc = "Register `SR2` reader"]
pub type R = crate::R<SR2_SPEC>;
#[doc = "Field `FLASH_RDY` reader - Flash ready flag"]
pub type FLASH_RDY_R = crate::BitReader;
#[doc = "Field `REGLPS` reader - Low-power regulator started"]
pub type REGLPS_R = crate::BitReader;
#[doc = "Field `REGLPF` reader - Low-power regulator flag"]
pub type REGLPF_R = crate::BitReader;
#[doc = "Field `VOSF` reader - Voltage scaling flag"]
pub type VOSF_R = crate::BitReader;
impl R {
    #[doc = "Bit 7 - Flash ready flag"]
    #[inline(always)]
    pub fn flash_rdy(&self) -> FLASH_RDY_R {
        FLASH_RDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Low-power regulator started"]
    #[inline(always)]
    pub fn reglps(&self) -> REGLPS_R {
        REGLPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low-power regulator flag"]
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Voltage scaling flag"]
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Power status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR2_SPEC;
impl crate::RegisterSpec for SR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr2::R`](R) reader structure"]
impl crate::Readable for SR2_SPEC {}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for SR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
