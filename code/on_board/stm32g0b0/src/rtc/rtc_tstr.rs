#[doc = "Register `RTC_TSTR` reader"]
pub type R = crate::R<RTC_TSTR_SPEC>;
#[doc = "Field `SU` reader - Second units in BCD format."]
pub type SU_R = crate::FieldReader;
#[doc = "Field `ST` reader - Second tens in BCD format."]
pub type ST_R = crate::FieldReader;
#[doc = "Field `MNU` reader - Minute units in BCD format."]
pub type MNU_R = crate::FieldReader;
#[doc = "Field `MNT` reader - Minute tens in BCD format."]
pub type MNT_R = crate::FieldReader;
#[doc = "Field `HU` reader - Hour units in BCD format."]
pub type HU_R = crate::FieldReader;
#[doc = "Field `HT` reader - Hour tens in BCD format."]
pub type HT_R = crate::FieldReader;
#[doc = "Field `PM` reader - AM/PM notation"]
pub type PM_R = crate::BitReader<PM_A>;
#[doc = "AM/PM notation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PM_A {
    #[doc = "0: AM or 24-hour format"]
    B_0X0 = 0,
    #[doc = "1: PM"]
    B_0X1 = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
impl PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::B_0X0,
            true => PM_A::B_0X1,
        }
    }
    #[doc = "AM or 24-hour format"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PM_A::B_0X0
    }
    #[doc = "PM"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PM_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:3 - Second units in BCD format."]
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format."]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format."]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format."]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format."]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format."]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "RTC timestamp time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_tstr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_TSTR_SPEC;
impl crate::RegisterSpec for RTC_TSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_tstr::R`](R) reader structure"]
impl crate::Readable for RTC_TSTR_SPEC {}
#[doc = "`reset()` method sets RTC_TSTR to value 0"]
impl crate::Resettable for RTC_TSTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
