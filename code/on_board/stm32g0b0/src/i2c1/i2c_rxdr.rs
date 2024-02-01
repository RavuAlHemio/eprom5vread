#[doc = "Register `I2C_RXDR` reader"]
pub type R = crate::R<I2C_RXDR_SPEC>;
#[doc = "Field `RXDATA` reader - 8-bit receive data"]
pub type RXDATA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 8-bit receive data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_rxdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_RXDR_SPEC;
impl crate::RegisterSpec for I2C_RXDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_rxdr::R`](R) reader structure"]
impl crate::Readable for I2C_RXDR_SPEC {}
#[doc = "`reset()` method sets I2C_RXDR to value 0"]
impl crate::Resettable for I2C_RXDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
