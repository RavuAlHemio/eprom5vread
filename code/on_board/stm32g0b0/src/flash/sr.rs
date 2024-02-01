#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Field `EOP` reader - End of operation"]
pub type EOP_R = crate::BitReader;
#[doc = "Field `EOP` writer - End of operation"]
pub type EOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPERR` reader - Operation error"]
pub type OPERR_R = crate::BitReader;
#[doc = "Field `OPERR` writer - Operation error"]
pub type OPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROGERR` reader - Programming error"]
pub type PROGERR_R = crate::BitReader;
#[doc = "Field `PROGERR` writer - Programming error"]
pub type PROGERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPERR` reader - Write protected error"]
pub type WRPERR_R = crate::BitReader;
#[doc = "Field `WRPERR` writer - Write protected error"]
pub type WRPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGAERR` reader - Programming alignment error"]
pub type PGAERR_R = crate::BitReader;
#[doc = "Field `PGAERR` writer - Programming alignment error"]
pub type PGAERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIZERR` reader - Size error"]
pub type SIZERR_R = crate::BitReader;
#[doc = "Field `SIZERR` writer - Size error"]
pub type SIZERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGSERR` reader - Programming sequence error"]
pub type PGSERR_R = crate::BitReader;
#[doc = "Field `PGSERR` writer - Programming sequence error"]
pub type PGSERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISERR` reader - Fast programming data miss error"]
pub type MISERR_R = crate::BitReader;
#[doc = "Field `MISERR` writer - Fast programming data miss error"]
pub type MISERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FASTERR` reader - Fast programming error"]
pub type FASTERR_R = crate::BitReader;
#[doc = "Field `FASTERR` writer - Fast programming error"]
pub type FASTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTVERR` reader - Option and Engineering bits loading validity error"]
pub type OPTVERR_R = crate::BitReader;
#[doc = "Field `OPTVERR` writer - Option and Engineering bits loading validity error"]
pub type OPTVERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSY1` reader - BSY1"]
pub type BSY1_R = crate::BitReader;
#[doc = "Field `BSY1` writer - BSY1"]
pub type BSY1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSY2` reader - BSY2"]
pub type BSY2_R = crate::BitReader;
#[doc = "Field `BSY2` writer - BSY2"]
pub type BSY2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGBSY` reader - Programming or erase configuration busy."]
pub type CFGBSY_R = crate::BitReader;
#[doc = "Field `CFGBSY` writer - Programming or erase configuration busy."]
pub type CFGBSY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operation error"]
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Programming error"]
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protected error"]
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Programming alignment error"]
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Size error"]
    #[inline(always)]
    pub fn sizerr(&self) -> SIZERR_R {
        SIZERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Programming sequence error"]
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fast programming data miss error"]
    #[inline(always)]
    pub fn miserr(&self) -> MISERR_R {
        MISERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast programming error"]
    #[inline(always)]
    pub fn fasterr(&self) -> FASTERR_R {
        FASTERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - Option and Engineering bits loading validity error"]
    #[inline(always)]
    pub fn optverr(&self) -> OPTVERR_R {
        OPTVERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - BSY1"]
    #[inline(always)]
    pub fn bsy1(&self) -> BSY1_R {
        BSY1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BSY2"]
    #[inline(always)]
    pub fn bsy2(&self) -> BSY2_R {
        BSY2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Programming or erase configuration busy."]
    #[inline(always)]
    pub fn cfgbsy(&self) -> CFGBSY_R {
        CFGBSY_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    #[must_use]
    pub fn eop(&mut self) -> EOP_W<SR_SPEC> {
        EOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Operation error"]
    #[inline(always)]
    #[must_use]
    pub fn operr(&mut self) -> OPERR_W<SR_SPEC> {
        OPERR_W::new(self, 1)
    }
    #[doc = "Bit 3 - Programming error"]
    #[inline(always)]
    #[must_use]
    pub fn progerr(&mut self) -> PROGERR_W<SR_SPEC> {
        PROGERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write protected error"]
    #[inline(always)]
    #[must_use]
    pub fn wrperr(&mut self) -> WRPERR_W<SR_SPEC> {
        WRPERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Programming alignment error"]
    #[inline(always)]
    #[must_use]
    pub fn pgaerr(&mut self) -> PGAERR_W<SR_SPEC> {
        PGAERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Size error"]
    #[inline(always)]
    #[must_use]
    pub fn sizerr(&mut self) -> SIZERR_W<SR_SPEC> {
        SIZERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Programming sequence error"]
    #[inline(always)]
    #[must_use]
    pub fn pgserr(&mut self) -> PGSERR_W<SR_SPEC> {
        PGSERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Fast programming data miss error"]
    #[inline(always)]
    #[must_use]
    pub fn miserr(&mut self) -> MISERR_W<SR_SPEC> {
        MISERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Fast programming error"]
    #[inline(always)]
    #[must_use]
    pub fn fasterr(&mut self) -> FASTERR_W<SR_SPEC> {
        FASTERR_W::new(self, 9)
    }
    #[doc = "Bit 15 - Option and Engineering bits loading validity error"]
    #[inline(always)]
    #[must_use]
    pub fn optverr(&mut self) -> OPTVERR_W<SR_SPEC> {
        OPTVERR_W::new(self, 15)
    }
    #[doc = "Bit 16 - BSY1"]
    #[inline(always)]
    #[must_use]
    pub fn bsy1(&mut self) -> BSY1_W<SR_SPEC> {
        BSY1_W::new(self, 16)
    }
    #[doc = "Bit 17 - BSY2"]
    #[inline(always)]
    #[must_use]
    pub fn bsy2(&mut self) -> BSY2_W<SR_SPEC> {
        BSY2_W::new(self, 17)
    }
    #[doc = "Bit 18 - Programming or erase configuration busy."]
    #[inline(always)]
    #[must_use]
    pub fn cfgbsy(&mut self) -> CFGBSY_W<SR_SPEC> {
        CFGBSY_W::new(self, 18)
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
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0;
}
