#[doc = "Register `PLLSYSCFGR` reader"]
pub type R = crate::R<PLLSYSCFGR_SPEC>;
#[doc = "Register `PLLSYSCFGR` writer"]
pub type W = crate::W<PLLSYSCFGR_SPEC>;
#[doc = "Field `PLLSRC` reader - PLL input clock source"]
pub type PLLSRC_R = crate::FieldReader;
#[doc = "Field `PLLSRC` writer - PLL input clock source"]
pub type PLLSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLLM` reader - Division factor M of the PLL input clock divider"]
pub type PLLM_R = crate::FieldReader;
#[doc = "Field `PLLM` writer - Division factor M of the PLL input clock divider"]
pub type PLLM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLN` reader - PLL frequency multiplication factor N"]
pub type PLLN_R = crate::FieldReader;
#[doc = "Field `PLLN` writer - PLL frequency multiplication factor N"]
pub type PLLN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PLLPEN` reader - PLLPCLK clock output enable"]
pub type PLLPEN_R = crate::BitReader;
#[doc = "Field `PLLPEN` writer - PLLPCLK clock output enable"]
pub type PLLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLP` reader - PLL VCO division factor P for PLLPCLK clock output"]
pub type PLLP_R = crate::FieldReader;
#[doc = "Field `PLLP` writer - PLL VCO division factor P for PLLPCLK clock output"]
pub type PLLP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PLLQEN` reader - PLLQCLK clock output enable"]
pub type PLLQEN_R = crate::BitReader;
#[doc = "Field `PLLQEN` writer - PLLQCLK clock output enable"]
pub type PLLQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLQ` reader - PLL VCO division factor Q for PLLQCLK clock output"]
pub type PLLQ_R = crate::FieldReader;
#[doc = "Field `PLLQ` writer - PLL VCO division factor Q for PLLQCLK clock output"]
pub type PLLQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLREN` reader - PLLRCLK clock output enable"]
pub type PLLREN_R = crate::BitReader;
#[doc = "Field `PLLREN` writer - PLLRCLK clock output enable"]
pub type PLLREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLR` reader - PLL VCO division factor R for PLLRCLK clock output"]
pub type PLLR_R = crate::FieldReader;
#[doc = "Field `PLLR` writer - PLL VCO division factor R for PLLRCLK clock output"]
pub type PLLR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - PLL input clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Division factor M of the PLL input clock divider"]
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:15 - PLL frequency multiplication factor N"]
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - PLLPCLK clock output enable"]
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - PLL VCO division factor P for PLLPCLK clock output"]
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - PLLQCLK clock output enable"]
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - PLL VCO division factor Q for PLLQCLK clock output"]
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - PLLRCLK clock output enable"]
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - PLL VCO division factor R for PLLRCLK clock output"]
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL input clock source"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<PLLSYSCFGR_SPEC> {
        PLLSRC_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Division factor M of the PLL input clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn pllm(&mut self) -> PLLM_W<PLLSYSCFGR_SPEC> {
        PLLM_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - PLL frequency multiplication factor N"]
    #[inline(always)]
    #[must_use]
    pub fn plln(&mut self) -> PLLN_W<PLLSYSCFGR_SPEC> {
        PLLN_W::new(self, 8)
    }
    #[doc = "Bit 16 - PLLPCLK clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllpen(&mut self) -> PLLPEN_W<PLLSYSCFGR_SPEC> {
        PLLPEN_W::new(self, 16)
    }
    #[doc = "Bits 17:21 - PLL VCO division factor P for PLLPCLK clock output"]
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PLLP_W<PLLSYSCFGR_SPEC> {
        PLLP_W::new(self, 17)
    }
    #[doc = "Bit 24 - PLLQCLK clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllqen(&mut self) -> PLLQEN_W<PLLSYSCFGR_SPEC> {
        PLLQEN_W::new(self, 24)
    }
    #[doc = "Bits 25:27 - PLL VCO division factor Q for PLLQCLK clock output"]
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PLLQ_W<PLLSYSCFGR_SPEC> {
        PLLQ_W::new(self, 25)
    }
    #[doc = "Bit 28 - PLLRCLK clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllren(&mut self) -> PLLREN_W<PLLSYSCFGR_SPEC> {
        PLLREN_W::new(self, 28)
    }
    #[doc = "Bits 29:31 - PLL VCO division factor R for PLLRCLK clock output"]
    #[inline(always)]
    #[must_use]
    pub fn pllr(&mut self) -> PLLR_W<PLLSYSCFGR_SPEC> {
        PLLR_W::new(self, 29)
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
#[doc = "PLL configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllsyscfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllsyscfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLSYSCFGR_SPEC;
impl crate::RegisterSpec for PLLSYSCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllsyscfgr::R`](R) reader structure"]
impl crate::Readable for PLLSYSCFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllsyscfgr::W`](W) writer structure"]
impl crate::Writable for PLLSYSCFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLSYSCFGR to value 0x1000"]
impl crate::Resettable for PLLSYSCFGR_SPEC {
    const RESET_VALUE: u32 = 0x1000;
}
