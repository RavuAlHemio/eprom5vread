#[doc = "Register `I2C_PECR` reader"]
pub type R = crate::R<I2C_PECR_SPEC>;
#[doc = "Field `PEC` reader - Packet error checking register"]
pub type PEC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Packet error checking register"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PEC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_pecr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_PECR_SPEC;
impl crate::RegisterSpec for I2C_PECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_pecr::R`](R) reader structure"]
impl crate::Readable for I2C_PECR_SPEC {}
#[doc = "`reset()` method sets I2C_PECR to value 0"]
impl crate::Resettable for I2C_PECR_SPEC {
    const RESET_VALUE: u32 = 0;
}
