#[doc = "Register `FTSR1` reader"]
pub type R = crate::R<FTSR1_SPEC>;
#[doc = "Register `FTSR1` writer"]
pub type W = crate::W<FTSR1_SPEC>;
#[doc = "Field `FT0` reader - Falling trigger event configuration bit of configurable line"]
pub type FT0_R = crate::BitReader;
#[doc = "Field `FT0` writer - Falling trigger event configuration bit of configurable line"]
pub type FT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT1` reader - Falling trigger event configuration bit of configurable line"]
pub type FT1_R = crate::BitReader;
#[doc = "Field `FT1` writer - Falling trigger event configuration bit of configurable line"]
pub type FT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT2` reader - Falling trigger event configuration bit of configurable line"]
pub type FT2_R = crate::BitReader;
#[doc = "Field `FT2` writer - Falling trigger event configuration bit of configurable line"]
pub type FT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT3` reader - Falling trigger event configuration bit of configurable line"]
pub type FT3_R = crate::BitReader;
#[doc = "Field `FT3` writer - Falling trigger event configuration bit of configurable line"]
pub type FT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT4` reader - Falling trigger event configuration bit of configurable line"]
pub type FT4_R = crate::BitReader;
#[doc = "Field `FT4` writer - Falling trigger event configuration bit of configurable line"]
pub type FT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT5` reader - Falling trigger event configuration bit of configurable line"]
pub type FT5_R = crate::BitReader;
#[doc = "Field `FT5` writer - Falling trigger event configuration bit of configurable line"]
pub type FT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT6` reader - Falling trigger event configuration bit of configurable line"]
pub type FT6_R = crate::BitReader;
#[doc = "Field `FT6` writer - Falling trigger event configuration bit of configurable line"]
pub type FT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT7` reader - Falling trigger event configuration bit of configurable line"]
pub type FT7_R = crate::BitReader;
#[doc = "Field `FT7` writer - Falling trigger event configuration bit of configurable line"]
pub type FT7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT8` reader - Falling trigger event configuration bit of configurable line"]
pub type FT8_R = crate::BitReader;
#[doc = "Field `FT8` writer - Falling trigger event configuration bit of configurable line"]
pub type FT8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT9` reader - Falling trigger event configuration bit of configurable line"]
pub type FT9_R = crate::BitReader;
#[doc = "Field `FT9` writer - Falling trigger event configuration bit of configurable line"]
pub type FT9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT10` reader - Falling trigger event configuration bit of configurable line"]
pub type FT10_R = crate::BitReader;
#[doc = "Field `FT10` writer - Falling trigger event configuration bit of configurable line"]
pub type FT10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT11` reader - Falling trigger event configuration bit of configurable line"]
pub type FT11_R = crate::BitReader;
#[doc = "Field `FT11` writer - Falling trigger event configuration bit of configurable line"]
pub type FT11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT12` reader - Falling trigger event configuration bit of configurable line"]
pub type FT12_R = crate::BitReader;
#[doc = "Field `FT12` writer - Falling trigger event configuration bit of configurable line"]
pub type FT12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT13` reader - Falling trigger event configuration bit of configurable line"]
pub type FT13_R = crate::BitReader;
#[doc = "Field `FT13` writer - Falling trigger event configuration bit of configurable line"]
pub type FT13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT14` reader - Falling trigger event configuration bit of configurable line"]
pub type FT14_R = crate::BitReader;
#[doc = "Field `FT14` writer - Falling trigger event configuration bit of configurable line"]
pub type FT14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT15` reader - Falling trigger event configuration bit of configurable line"]
pub type FT15_R = crate::BitReader;
#[doc = "Field `FT15` writer - Falling trigger event configuration bit of configurable line"]
pub type FT15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft0(&self) -> FT0_R {
        FT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft1(&self) -> FT1_R {
        FT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft2(&self) -> FT2_R {
        FT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft3(&self) -> FT3_R {
        FT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft4(&self) -> FT4_R {
        FT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft5(&self) -> FT5_R {
        FT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft6(&self) -> FT6_R {
        FT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft7(&self) -> FT7_R {
        FT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft8(&self) -> FT8_R {
        FT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft9(&self) -> FT9_R {
        FT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft10(&self) -> FT10_R {
        FT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft11(&self) -> FT11_R {
        FT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft12(&self) -> FT12_R {
        FT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft13(&self) -> FT13_R {
        FT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft14(&self) -> FT14_R {
        FT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    pub fn ft15(&self) -> FT15_R {
        FT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft0(&mut self) -> FT0_W<FTSR1_SPEC> {
        FT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft1(&mut self) -> FT1_W<FTSR1_SPEC> {
        FT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft2(&mut self) -> FT2_W<FTSR1_SPEC> {
        FT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft3(&mut self) -> FT3_W<FTSR1_SPEC> {
        FT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft4(&mut self) -> FT4_W<FTSR1_SPEC> {
        FT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft5(&mut self) -> FT5_W<FTSR1_SPEC> {
        FT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft6(&mut self) -> FT6_W<FTSR1_SPEC> {
        FT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft7(&mut self) -> FT7_W<FTSR1_SPEC> {
        FT7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft8(&mut self) -> FT8_W<FTSR1_SPEC> {
        FT8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft9(&mut self) -> FT9_W<FTSR1_SPEC> {
        FT9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft10(&mut self) -> FT10_W<FTSR1_SPEC> {
        FT10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft11(&mut self) -> FT11_W<FTSR1_SPEC> {
        FT11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft12(&mut self) -> FT12_W<FTSR1_SPEC> {
        FT12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft13(&mut self) -> FT13_W<FTSR1_SPEC> {
        FT13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft14(&mut self) -> FT14_W<FTSR1_SPEC> {
        FT14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Falling trigger event configuration bit of configurable line"]
    #[inline(always)]
    #[must_use]
    pub fn ft15(&mut self) -> FT15_W<FTSR1_SPEC> {
        FT15_W::new(self, 15)
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
#[doc = "EXTI falling trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FTSR1_SPEC;
impl crate::RegisterSpec for FTSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftsr1::R`](R) reader structure"]
impl crate::Readable for FTSR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ftsr1::W`](W) writer structure"]
impl crate::Writable for FTSR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FTSR1 to value 0"]
impl crate::Resettable for FTSR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
