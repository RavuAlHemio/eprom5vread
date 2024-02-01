#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Field `LPMS` reader - Low-power mode selection"]
pub type LPMS_R = crate::FieldReader;
#[doc = "Field `LPMS` writer - Low-power mode selection"]
pub type LPMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FPD_STOP` reader - Flash memory powered down during Stop mode"]
pub type FPD_STOP_R = crate::BitReader;
#[doc = "Field `FPD_STOP` writer - Flash memory powered down during Stop mode"]
pub type FPD_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPD_LPRUN` reader - Flash memory powered down during Low-power run mode"]
pub type FPD_LPRUN_R = crate::BitReader;
#[doc = "Field `FPD_LPRUN` writer - Flash memory powered down during Low-power run mode"]
pub type FPD_LPRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPD_LPSLP` reader - Flash memory powered down during Low-power sleep mode"]
pub type FPD_LPSLP_R = crate::BitReader;
#[doc = "Field `FPD_LPSLP` writer - Flash memory powered down during Low-power sleep mode"]
pub type FPD_LPSLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBP` reader - Disable backup domain write protection"]
pub type DBP_R = crate::BitReader;
#[doc = "Field `DBP` writer - Disable backup domain write protection"]
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VOS` reader - Voltage scaling range selection"]
pub type VOS_R = crate::FieldReader;
#[doc = "Field `VOS` writer - Voltage scaling range selection"]
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPR` reader - Low-power run"]
pub type LPR_R = crate::BitReader;
#[doc = "Field `LPR` writer - Low-power run"]
pub type LPR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Low-power mode selection"]
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Flash memory powered down during Stop mode"]
    #[inline(always)]
    pub fn fpd_stop(&self) -> FPD_STOP_R {
        FPD_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash memory powered down during Low-power run mode"]
    #[inline(always)]
    pub fn fpd_lprun(&self) -> FPD_LPRUN_R {
        FPD_LPRUN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash memory powered down during Low-power sleep mode"]
    #[inline(always)]
    pub fn fpd_lpslp(&self) -> FPD_LPSLP_R {
        FPD_LPSLP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 14 - Low-power run"]
    #[inline(always)]
    pub fn lpr(&self) -> LPR_R {
        LPR_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Low-power mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpms(&mut self) -> LPMS_W<CR1_SPEC> {
        LPMS_W::new(self, 0)
    }
    #[doc = "Bit 3 - Flash memory powered down during Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn fpd_stop(&mut self) -> FPD_STOP_W<CR1_SPEC> {
        FPD_STOP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Flash memory powered down during Low-power run mode"]
    #[inline(always)]
    #[must_use]
    pub fn fpd_lprun(&mut self) -> FPD_LPRUN_W<CR1_SPEC> {
        FPD_LPRUN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Flash memory powered down during Low-power sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn fpd_lpslp(&mut self) -> FPD_LPSLP_W<CR1_SPEC> {
        FPD_LPSLP_W::new(self, 5)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<CR1_SPEC> {
        DBP_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - Voltage scaling range selection"]
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<CR1_SPEC> {
        VOS_W::new(self, 9)
    }
    #[doc = "Bit 14 - Low-power run"]
    #[inline(always)]
    #[must_use]
    pub fn lpr(&mut self) -> LPR_W<CR1_SPEC> {
        LPR_W::new(self, 14)
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
#[doc = "Power control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0x0208"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: u32 = 0x0208;
}
