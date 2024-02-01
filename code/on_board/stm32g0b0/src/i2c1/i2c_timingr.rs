#[doc = "Register `I2C_TIMINGR` reader"]
pub type R = crate::R<I2C_TIMINGR_SPEC>;
#[doc = "Register `I2C_TIMINGR` writer"]
pub type W = crate::W<I2C_TIMINGR_SPEC>;
#[doc = "Field `SCLL` reader - SCL low period (master mode)"]
pub type SCLL_R = crate::FieldReader;
#[doc = "Field `SCLL` writer - SCL low period (master mode)"]
pub type SCLL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLH` reader - SCL high period (master mode)"]
pub type SCLH_R = crate::FieldReader;
#[doc = "Field `SCLH` writer - SCL high period (master mode)"]
pub type SCLH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SDADEL` reader - Data hold time"]
pub type SDADEL_R = crate::FieldReader;
#[doc = "Field `SDADEL` writer - Data hold time"]
pub type SDADEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCLDEL` reader - Data setup time"]
pub type SCLDEL_R = crate::FieldReader;
#[doc = "Field `SCLDEL` writer - Data setup time"]
pub type SCLDEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRESC` reader - Timing prescaler"]
pub type PRESC_R = crate::FieldReader;
#[doc = "Field `PRESC` writer - Timing prescaler"]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - SCL low period (master mode)"]
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCL high period (master mode)"]
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data hold time"]
    #[inline(always)]
    pub fn sdadel(&self) -> SDADEL_R {
        SDADEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Data setup time"]
    #[inline(always)]
    pub fn scldel(&self) -> SCLDEL_R {
        SCLDEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Timing prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCL low period (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn scll(&mut self) -> SCLL_W<I2C_TIMINGR_SPEC> {
        SCLL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SCL high period (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn sclh(&mut self) -> SCLH_W<I2C_TIMINGR_SPEC> {
        SCLH_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Data hold time"]
    #[inline(always)]
    #[must_use]
    pub fn sdadel(&mut self) -> SDADEL_W<I2C_TIMINGR_SPEC> {
        SDADEL_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Data setup time"]
    #[inline(always)]
    #[must_use]
    pub fn scldel(&mut self) -> SCLDEL_W<I2C_TIMINGR_SPEC> {
        SCLDEL_W::new(self, 20)
    }
    #[doc = "Bits 28:31 - Timing prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<I2C_TIMINGR_SPEC> {
        PRESC_W::new(self, 28)
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
#[doc = "Timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_timingr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_timingr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_TIMINGR_SPEC;
impl crate::RegisterSpec for I2C_TIMINGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_timingr::R`](R) reader structure"]
impl crate::Readable for I2C_TIMINGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_timingr::W`](W) writer structure"]
impl crate::Writable for I2C_TIMINGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_TIMINGR to value 0"]
impl crate::Resettable for I2C_TIMINGR_SPEC {
    const RESET_VALUE: u32 = 0;
}
