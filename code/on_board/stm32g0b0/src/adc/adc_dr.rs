#[doc = "Register `ADC_DR` reader"]
pub type R = crate::R<ADC_DR_SPEC>;
#[doc = "Field `DATA` reader - Converted data These bits are read-only. They contain the conversion result from the last converted channel. The data are left- or right-aligned as shown in OVSE = 0) on page349. Just after a calibration is complete, DATA\\[6:0\\]
contains the calibration factor."]
pub type DATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Converted data These bits are read-only. They contain the conversion result from the last converted channel. The data are left- or right-aligned as shown in OVSE = 0) on page349. Just after a calibration is complete, DATA\\[6:0\\]
contains the calibration factor."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ADC data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_DR_SPEC;
impl crate::RegisterSpec for ADC_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_dr::R`](R) reader structure"]
impl crate::Readable for ADC_DR_SPEC {}
#[doc = "`reset()` method sets ADC_DR to value 0"]
impl crate::Resettable for ADC_DR_SPEC {
    const RESET_VALUE: u32 = 0;
}
