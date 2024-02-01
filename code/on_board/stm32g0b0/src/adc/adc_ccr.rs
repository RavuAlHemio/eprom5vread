#[doc = "Register `ADC_CCR` reader"]
pub type R = crate::R<ADC_CCR_SPEC>;
#[doc = "Register `ADC_CCR` writer"]
pub type W = crate::W<ADC_CCR_SPEC>;
#[doc = "Field `PRESC` reader - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0)."]
pub type PRESC_R = crate::FieldReader<PRESC_A>;
#[doc = "ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: input ADC clock not divided"]
    B_0X0 = 0,
    #[doc = "1: input ADC clock divided by 2"]
    B_0X1 = 1,
    #[doc = "2: input ADC clock divided by 4"]
    B_0X2 = 2,
    #[doc = "3: input ADC clock divided by 6"]
    B_0X3 = 3,
    #[doc = "4: input ADC clock divided by 8"]
    B_0X4 = 4,
    #[doc = "5: input ADC clock divided by 10"]
    B_0X5 = 5,
    #[doc = "6: input ADC clock divided by 12"]
    B_0X6 = 6,
    #[doc = "7: input ADC clock divided by 16"]
    B_0X7 = 7,
    #[doc = "8: input ADC clock divided by 32"]
    B_0X8 = 8,
    #[doc = "9: input ADC clock divided by 64"]
    B_0X9 = 9,
    #[doc = "10: input ADC clock divided by 128"]
    B_0X_A = 10,
    #[doc = "11: input ADC clock divided by 256"]
    B_0X_B = 11,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC_A {
    type Ux = u8;
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESC_A> {
        match self.bits {
            0 => Some(PRESC_A::B_0X0),
            1 => Some(PRESC_A::B_0X1),
            2 => Some(PRESC_A::B_0X2),
            3 => Some(PRESC_A::B_0X3),
            4 => Some(PRESC_A::B_0X4),
            5 => Some(PRESC_A::B_0X5),
            6 => Some(PRESC_A::B_0X6),
            7 => Some(PRESC_A::B_0X7),
            8 => Some(PRESC_A::B_0X8),
            9 => Some(PRESC_A::B_0X9),
            10 => Some(PRESC_A::B_0X_A),
            11 => Some(PRESC_A::B_0X_B),
            _ => None,
        }
    }
    #[doc = "input ADC clock not divided"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PRESC_A::B_0X0
    }
    #[doc = "input ADC clock divided by 2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PRESC_A::B_0X1
    }
    #[doc = "input ADC clock divided by 4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PRESC_A::B_0X2
    }
    #[doc = "input ADC clock divided by 6"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PRESC_A::B_0X3
    }
    #[doc = "input ADC clock divided by 8"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == PRESC_A::B_0X4
    }
    #[doc = "input ADC clock divided by 10"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == PRESC_A::B_0X5
    }
    #[doc = "input ADC clock divided by 12"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == PRESC_A::B_0X6
    }
    #[doc = "input ADC clock divided by 16"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == PRESC_A::B_0X7
    }
    #[doc = "input ADC clock divided by 32"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == PRESC_A::B_0X8
    }
    #[doc = "input ADC clock divided by 64"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == PRESC_A::B_0X9
    }
    #[doc = "input ADC clock divided by 128"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == PRESC_A::B_0X_A
    }
    #[doc = "input ADC clock divided by 256"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == PRESC_A::B_0X_B
    }
}
#[doc = "Field `PRESC` writer - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0)."]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRESC_A>;
impl<'a, REG> PRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "input ADC clock not divided"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0X0)
    }
    #[doc = "input ADC clock divided by 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0X1)
    }
    #[doc = "input ADC clock divided by 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0X2)
    }
    #[doc = "input ADC clock divided by 6"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0X3)
    }
    #[doc = "input ADC clock divided by 8"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0X4)
    }
    #[doc = "input ADC clock divided by 10"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0X5)
    }
    #[doc = "input ADC clock divided by 12"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0X6)
    }
    #[doc = "input ADC clock divided by 16"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0X7)
    }
    #[doc = "input ADC clock divided by 32"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0X8)
    }
    #[doc = "input ADC clock divided by 64"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0X9)
    }
    #[doc = "input ADC clock divided by 128"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0X_A)
    }
    #[doc = "input ADC clock divided by 256"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0X_B)
    }
}
#[doc = "Field `VREFEN` reader - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type VREFEN_R = crate::BitReader<VREFEN_A>;
#[doc = "VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFEN_A {
    #[doc = "0: VREFINT disabled"]
    B_0X0 = 0,
    #[doc = "1: VREFINT enabled"]
    B_0X1 = 1,
}
impl From<VREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VREFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VREFEN_A {
        match self.bits {
            false => VREFEN_A::B_0X0,
            true => VREFEN_A::B_0X1,
        }
    }
    #[doc = "VREFINT disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VREFEN_A::B_0X0
    }
    #[doc = "VREFINT enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VREFEN_A::B_0X1
    }
}
#[doc = "Field `VREFEN` writer - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type VREFEN_W<'a, REG> = crate::BitWriter<'a, REG, VREFEN_A>;
impl<'a, REG> VREFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VREFINT disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN_A::B_0X0)
    }
    #[doc = "VREFINT enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(VREFEN_A::B_0X1)
    }
}
#[doc = "Field `TSEN` reader - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type TSEN_R = crate::BitReader<TSEN_A>;
#[doc = "Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEN_A {
    #[doc = "0: Temperature sensor disabled"]
    B_0X0 = 0,
    #[doc = "1: Temperature sensor enabled"]
    B_0X1 = 1,
}
impl From<TSEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSEN_A {
        match self.bits {
            false => TSEN_A::B_0X0,
            true => TSEN_A::B_0X1,
        }
    }
    #[doc = "Temperature sensor disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSEN_A::B_0X0
    }
    #[doc = "Temperature sensor enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSEN_A::B_0X1
    }
}
#[doc = "Field `TSEN` writer - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
pub type TSEN_W<'a, REG> = crate::BitWriter<'a, REG, TSEN_A>;
impl<'a, REG> TSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Temperature sensor disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSEN_A::B_0X0)
    }
    #[doc = "Temperature sensor enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSEN_A::B_0X1)
    }
}
#[doc = "Field `VBATEN` reader - VBAT enable This bit is set and cleared by software to enable/disable the VBAT channel. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)"]
pub type VBATEN_R = crate::BitReader<VBATEN_A>;
#[doc = "VBAT enable This bit is set and cleared by software to enable/disable the VBAT channel. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATEN_A {
    #[doc = "0: VBAT channel disabled"]
    B_0X0 = 0,
    #[doc = "1: VBAT channel enabled"]
    B_0X1 = 1,
}
impl From<VBATEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBATEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VBATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBATEN_A {
        match self.bits {
            false => VBATEN_A::B_0X0,
            true => VBATEN_A::B_0X1,
        }
    }
    #[doc = "VBAT channel disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VBATEN_A::B_0X0
    }
    #[doc = "VBAT channel enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VBATEN_A::B_0X1
    }
}
#[doc = "Field `VBATEN` writer - VBAT enable This bit is set and cleared by software to enable/disable the VBAT channel. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)"]
pub type VBATEN_W<'a, REG> = crate::BitWriter<'a, REG, VBATEN_A>;
impl<'a, REG> VBATEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBAT channel disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VBATEN_A::B_0X0)
    }
    #[doc = "VBAT channel enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(VBATEN_A::B_0X1)
    }
}
impl R {
    #[doc = "Bits 18:21 - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0)."]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - VBAT enable This bit is set and cleared by software to enable/disable the VBAT channel. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)"]
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 18:21 - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0 and ADEN=0)."]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<ADC_CCR_SPEC> {
        PRESC_W::new(self, 18)
    }
    #[doc = "Bit 22 - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<ADC_CCR_SPEC> {
        VREFEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<ADC_CCR_SPEC> {
        TSEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - VBAT enable This bit is set and cleared by software to enable/disable the VBAT channel. Note: The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing)"]
    #[inline(always)]
    #[must_use]
    pub fn vbaten(&mut self) -> VBATEN_W<ADC_CCR_SPEC> {
        VBATEN_W::new(self, 24)
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
#[doc = "ADC common configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_CCR_SPEC;
impl crate::RegisterSpec for ADC_CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_ccr::R`](R) reader structure"]
impl crate::Readable for ADC_CCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc_ccr::W`](W) writer structure"]
impl crate::Writable for ADC_CCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CCR to value 0"]
impl crate::Resettable for ADC_CCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
