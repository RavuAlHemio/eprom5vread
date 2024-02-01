#[doc = "Register `AFRH` reader"]
pub type R = crate::R<AFRH_SPEC>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::W<AFRH_SPEC>;
#[doc = "Field `AFSEL8` reader - "]
pub type AFSEL8_R = crate::FieldReader;
#[doc = "Field `AFSEL8` writer - "]
pub type AFSEL8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL9` reader - "]
pub type AFSEL9_R = crate::FieldReader;
#[doc = "Field `AFSEL9` writer - "]
pub type AFSEL9_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL10` reader - "]
pub type AFSEL10_R = crate::FieldReader;
#[doc = "Field `AFSEL10` writer - "]
pub type AFSEL10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL11` reader - "]
pub type AFSEL11_R = crate::FieldReader;
#[doc = "Field `AFSEL11` writer - "]
pub type AFSEL11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL12` reader - "]
pub type AFSEL12_R = crate::FieldReader;
#[doc = "Field `AFSEL12` writer - "]
pub type AFSEL12_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL13` reader - "]
pub type AFSEL13_R = crate::FieldReader;
#[doc = "Field `AFSEL13` writer - "]
pub type AFSEL13_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL14` reader - "]
pub type AFSEL14_R = crate::FieldReader;
#[doc = "Field `AFSEL14` writer - "]
pub type AFSEL14_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL15` reader - "]
pub type AFSEL15_R = crate::FieldReader;
#[doc = "Field `AFSEL15` writer - "]
pub type AFSEL15_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn afsel8(&self) -> AFSEL8_R {
        AFSEL8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn afsel9(&self) -> AFSEL9_R {
        AFSEL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn afsel10(&self) -> AFSEL10_R {
        AFSEL10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn afsel11(&self) -> AFSEL11_R {
        AFSEL11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn afsel12(&self) -> AFSEL12_R {
        AFSEL12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn afsel13(&self) -> AFSEL13_R {
        AFSEL13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn afsel14(&self) -> AFSEL14_R {
        AFSEL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn afsel15(&self) -> AFSEL15_R {
        AFSEL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn afsel8(&mut self) -> AFSEL8_W<AFRH_SPEC> {
        AFSEL8_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn afsel9(&mut self) -> AFSEL9_W<AFRH_SPEC> {
        AFSEL9_W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn afsel10(&mut self) -> AFSEL10_W<AFRH_SPEC> {
        AFSEL10_W::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn afsel11(&mut self) -> AFSEL11_W<AFRH_SPEC> {
        AFSEL11_W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn afsel12(&mut self) -> AFSEL12_W<AFRH_SPEC> {
        AFSEL12_W::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn afsel13(&mut self) -> AFSEL13_W<AFRH_SPEC> {
        AFSEL13_W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn afsel14(&mut self) -> AFSEL14_W<AFRH_SPEC> {
        AFSEL14_W::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn afsel15(&mut self) -> AFSEL15_W<AFRH_SPEC> {
        AFSEL15_W::new(self, 28)
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
#[doc = "GPIO alternate function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFRH_SPEC;
impl crate::RegisterSpec for AFRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrh::R`](R) reader structure"]
impl crate::Readable for AFRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`afrh::W`](W) writer structure"]
impl crate::Writable for AFRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFRH to value 0"]
impl crate::Resettable for AFRH_SPEC {
    const RESET_VALUE: u32 = 0;
}
