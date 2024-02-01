#[doc = "Register `RTC_CALR` reader"]
pub type R = crate::R<RTC_CALR_SPEC>;
#[doc = "Register `RTC_CALR` writer"]
pub type W = crate::W<RTC_CALR_SPEC>;
#[doc = "Field `CALM` reader - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768Hz). This decreases the frequency of the calendar with a resolution of 0.9537ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See ."]
pub type CALM_R = crate::FieldReader<u16>;
#[doc = "Field `CALM` writer - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768Hz). This decreases the frequency of the calendar with a resolution of 0.9537ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See ."]
pub type CALM_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CALW16` reader - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\]
is stuck at 0 when CALW16 = 1. Refer to calibration."]
pub type CALW16_R = crate::BitReader;
#[doc = "Field `CALW16` writer - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\]
is stuck at 0 when CALW16 = 1. Refer to calibration."]
pub type CALW16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALW8` reader - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00 when CALW8 = 1. Refer to digital calibration."]
pub type CALW8_R = crate::BitReader;
#[doc = "Field `CALW8` writer - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00 when CALW8 = 1. Refer to digital calibration."]
pub type CALW8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALP` reader - Increase frequency of RTC by 488"]
pub type CALP_R = crate::BitReader<CALP_A>;
#[doc = "Increase frequency of RTC by 488\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALP_A {
    #[doc = "0: No RTCCLK pulses are added."]
    B_0X0 = 0,
    #[doc = "1: One RTCCLK pulse is effectively inserted every 211 pulses (frequency increased by 488.5ppm)."]
    B_0X1 = 1,
}
impl From<CALP_A> for bool {
    #[inline(always)]
    fn from(variant: CALP_A) -> Self {
        variant as u8 != 0
    }
}
impl CALP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CALP_A {
        match self.bits {
            false => CALP_A::B_0X0,
            true => CALP_A::B_0X1,
        }
    }
    #[doc = "No RTCCLK pulses are added."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CALP_A::B_0X0
    }
    #[doc = "One RTCCLK pulse is effectively inserted every 211 pulses (frequency increased by 488.5ppm)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CALP_A::B_0X1
    }
}
#[doc = "Field `CALP` writer - Increase frequency of RTC by 488"]
pub type CALP_W<'a, REG> = crate::BitWriter<'a, REG, CALP_A>;
impl<'a, REG> CALP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No RTCCLK pulses are added."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CALP_A::B_0X0)
    }
    #[doc = "One RTCCLK pulse is effectively inserted every 211 pulses (frequency increased by 488.5ppm)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CALP_A::B_0X1)
    }
}
impl R {
    #[doc = "Bits 0:8 - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768Hz). This decreases the frequency of the calendar with a resolution of 0.9537ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See ."]
    #[inline(always)]
    pub fn calm(&self) -> CALM_R {
        CALM_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\]
is stuck at 0 when CALW16 = 1. Refer to calibration."]
    #[inline(always)]
    pub fn calw16(&self) -> CALW16_R {
        CALW16_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00 when CALW8 = 1. Refer to digital calibration."]
    #[inline(always)]
    pub fn calw8(&self) -> CALW8_R {
        CALW8_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Increase frequency of RTC by 488"]
    #[inline(always)]
    pub fn calp(&self) -> CALP_R {
        CALP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768Hz). This decreases the frequency of the calendar with a resolution of 0.9537ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See ."]
    #[inline(always)]
    #[must_use]
    pub fn calm(&mut self) -> CALM_W<RTC_CALR_SPEC> {
        CALM_W::new(self, 0)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\\[0\\]
is stuck at 0 when CALW16 = 1. Refer to calibration."]
    #[inline(always)]
    #[must_use]
    pub fn calw16(&mut self) -> CALW16_W<RTC_CALR_SPEC> {
        CALW16_W::new(self, 13)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\\[1:0\\]
are stuck at 00 when CALW8 = 1. Refer to digital calibration."]
    #[inline(always)]
    #[must_use]
    pub fn calw8(&mut self) -> CALW8_W<RTC_CALR_SPEC> {
        CALW8_W::new(self, 14)
    }
    #[doc = "Bit 15 - Increase frequency of RTC by 488"]
    #[inline(always)]
    #[must_use]
    pub fn calp(&mut self) -> CALP_W<RTC_CALR_SPEC> {
        CALP_W::new(self, 15)
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
#[doc = "RTC calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_calr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_calr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_CALR_SPEC;
impl crate::RegisterSpec for RTC_CALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_calr::R`](R) reader structure"]
impl crate::Readable for RTC_CALR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_calr::W`](W) writer structure"]
impl crate::Writable for RTC_CALR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_CALR to value 0"]
impl crate::Resettable for RTC_CALR_SPEC {
    const RESET_VALUE: u32 = 0;
}
