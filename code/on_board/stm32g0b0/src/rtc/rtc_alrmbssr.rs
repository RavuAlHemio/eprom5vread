#[doc = "Register `RTC_ALRMBSSR` reader"]
pub type R = crate::R<RTC_ALRMBSSR_SPEC>;
#[doc = "Register `RTC_ALRMBSSR` writer"]
pub type W = crate::W<RTC_ALRMBSSR_SPEC>;
#[doc = "Field `SS` reader - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared."]
pub type SS_R = crate::FieldReader<u16>;
#[doc = "Field `SS` writer - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared."]
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `MASKSS` reader - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
pub type MASKSS_R = crate::FieldReader<MASKSS_A>;
#[doc = "Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MASKSS_A {
    #[doc = "0: No comparison on sub seconds for alarm B. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match)."]
    B_0X0 = 0,
    #[doc = "1: SS\\[14:1\\]
are don't care in alarm B comparison. Only SS\\[0\\]
is compared."]
    B_0X1 = 1,
    #[doc = "2: SS\\[14:2\\]
are don't care in alarm B comparison. Only SS\\[1:0\\]
are compared."]
    B_0X2 = 2,
    #[doc = "3: SS\\[14:3\\]
are don't care in alarm B comparison. Only SS\\[2:0\\]
are compared."]
    B_0X3 = 3,
    #[doc = "12: SS\\[14:12\\]
are don't care in alarm B comparison. SS\\[11:0\\]
are compared."]
    B_0X_C = 12,
    #[doc = "13: SS\\[14:13\\]
are don't care in alarm B comparison. SS\\[12:0\\]
are compared."]
    B_0X_D = 13,
    #[doc = "14: SS\\[14\\]
is don't care in alarm B comparison. SS\\[13:0\\]
are compared."]
    B_0X_E = 14,
    #[doc = "15: All 15 SS bits are compared and must match to activate alarm."]
    B_0X_F = 15,
}
impl From<MASKSS_A> for u8 {
    #[inline(always)]
    fn from(variant: MASKSS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MASKSS_A {
    type Ux = u8;
}
impl MASKSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MASKSS_A> {
        match self.bits {
            0 => Some(MASKSS_A::B_0X0),
            1 => Some(MASKSS_A::B_0X1),
            2 => Some(MASKSS_A::B_0X2),
            3 => Some(MASKSS_A::B_0X3),
            12 => Some(MASKSS_A::B_0X_C),
            13 => Some(MASKSS_A::B_0X_D),
            14 => Some(MASKSS_A::B_0X_E),
            15 => Some(MASKSS_A::B_0X_F),
            _ => None,
        }
    }
    #[doc = "No comparison on sub seconds for alarm B. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MASKSS_A::B_0X0
    }
    #[doc = "SS\\[14:1\\]
are don't care in alarm B comparison. Only SS\\[0\\]
is compared."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MASKSS_A::B_0X1
    }
    #[doc = "SS\\[14:2\\]
are don't care in alarm B comparison. Only SS\\[1:0\\]
are compared."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MASKSS_A::B_0X2
    }
    #[doc = "SS\\[14:3\\]
are don't care in alarm B comparison. Only SS\\[2:0\\]
are compared."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MASKSS_A::B_0X3
    }
    #[doc = "SS\\[14:12\\]
are don't care in alarm B comparison. SS\\[11:0\\]
are compared."]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == MASKSS_A::B_0X_C
    }
    #[doc = "SS\\[14:13\\]
are don't care in alarm B comparison. SS\\[12:0\\]
are compared."]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == MASKSS_A::B_0X_D
    }
    #[doc = "SS\\[14\\]
is don't care in alarm B comparison. SS\\[13:0\\]
are compared."]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == MASKSS_A::B_0X_E
    }
    #[doc = "All 15 SS bits are compared and must match to activate alarm."]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == MASKSS_A::B_0X_F
    }
}
#[doc = "Field `MASKSS` writer - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
pub type MASKSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MASKSS_A>;
impl<'a, REG> MASKSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No comparison on sub seconds for alarm B. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS_A::B_0X0)
    }
    #[doc = "SS\\[14:1\\]
are don't care in alarm B comparison. Only SS\\[0\\]
is compared."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS_A::B_0X1)
    }
    #[doc = "SS\\[14:2\\]
are don't care in alarm B comparison. Only SS\\[1:0\\]
are compared."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS_A::B_0X2)
    }
    #[doc = "SS\\[14:3\\]
are don't care in alarm B comparison. Only SS\\[2:0\\]
are compared."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS_A::B_0X3)
    }
    #[doc = "SS\\[14:12\\]
are don't care in alarm B comparison. SS\\[11:0\\]
are compared."]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS_A::B_0X_C)
    }
    #[doc = "SS\\[14:13\\]
are don't care in alarm B comparison. SS\\[12:0\\]
are compared."]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS_A::B_0X_D)
    }
    #[doc = "SS\\[14\\]
is don't care in alarm B comparison. SS\\[13:0\\]
are compared."]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS_A::B_0X_E)
    }
    #[doc = "All 15 SS bits are compared and must match to activate alarm."]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS_A::B_0X_F)
    }
}
impl R {
    #[doc = "Bits 0:14 - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared."]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
    #[inline(always)]
    pub fn maskss(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared."]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SS_W<RTC_ALRMBSSR_SPEC> {
        SS_W::new(self, 0)
    }
    #[doc = "Bits 24:27 - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
    #[inline(always)]
    #[must_use]
    pub fn maskss(&mut self) -> MASKSS_W<RTC_ALRMBSSR_SPEC> {
        MASKSS_W::new(self, 24)
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
#[doc = "RTC alarm B sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_alrmbssr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_alrmbssr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_ALRMBSSR_SPEC;
impl crate::RegisterSpec for RTC_ALRMBSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_alrmbssr::R`](R) reader structure"]
impl crate::Readable for RTC_ALRMBSSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_alrmbssr::W`](W) writer structure"]
impl crate::Writable for RTC_ALRMBSSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_ALRMBSSR to value 0"]
impl crate::Resettable for RTC_ALRMBSSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
