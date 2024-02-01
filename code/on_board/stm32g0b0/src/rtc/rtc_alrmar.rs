#[doc = "Register `RTC_ALRMAR` reader"]
pub type R = crate::R<RTC_ALRMAR_SPEC>;
#[doc = "Register `RTC_ALRMAR` writer"]
pub type W = crate::W<RTC_ALRMAR_SPEC>;
#[doc = "Field `SU` reader - Second units in BCD format."]
pub type SU_R = crate::FieldReader;
#[doc = "Field `SU` writer - Second units in BCD format."]
pub type SU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ST` reader - Second tens in BCD format."]
pub type ST_R = crate::FieldReader;
#[doc = "Field `ST` writer - Second tens in BCD format."]
pub type ST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSK1` reader - Alarm A seconds mask"]
pub type MSK1_R = crate::BitReader<MSK1_A>;
#[doc = "Alarm A seconds mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSK1_A {
    #[doc = "0: Alarm A set if the seconds match"]
    B_0X0 = 0,
    #[doc = "1: Seconds don't care in alarm A comparison"]
    B_0X1 = 1,
}
impl From<MSK1_A> for bool {
    #[inline(always)]
    fn from(variant: MSK1_A) -> Self {
        variant as u8 != 0
    }
}
impl MSK1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSK1_A {
        match self.bits {
            false => MSK1_A::B_0X0,
            true => MSK1_A::B_0X1,
        }
    }
    #[doc = "Alarm A set if the seconds match"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MSK1_A::B_0X0
    }
    #[doc = "Seconds don't care in alarm A comparison"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MSK1_A::B_0X1
    }
}
#[doc = "Field `MSK1` writer - Alarm A seconds mask"]
pub type MSK1_W<'a, REG> = crate::BitWriter<'a, REG, MSK1_A>;
impl<'a, REG> MSK1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm A set if the seconds match"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSK1_A::B_0X0)
    }
    #[doc = "Seconds don't care in alarm A comparison"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSK1_A::B_0X1)
    }
}
#[doc = "Field `MNU` reader - Minute units in BCD format"]
pub type MNU_R = crate::FieldReader;
#[doc = "Field `MNU` writer - Minute units in BCD format"]
pub type MNU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MNT` reader - Minute tens in BCD format"]
pub type MNT_R = crate::FieldReader;
#[doc = "Field `MNT` writer - Minute tens in BCD format"]
pub type MNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSK2` reader - Alarm A minutes mask"]
pub type MSK2_R = crate::BitReader<MSK2_A>;
#[doc = "Alarm A minutes mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSK2_A {
    #[doc = "0: Alarm A set if the minutes match"]
    B_0X0 = 0,
    #[doc = "1: Minutes don't care in alarm A comparison"]
    B_0X1 = 1,
}
impl From<MSK2_A> for bool {
    #[inline(always)]
    fn from(variant: MSK2_A) -> Self {
        variant as u8 != 0
    }
}
impl MSK2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSK2_A {
        match self.bits {
            false => MSK2_A::B_0X0,
            true => MSK2_A::B_0X1,
        }
    }
    #[doc = "Alarm A set if the minutes match"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MSK2_A::B_0X0
    }
    #[doc = "Minutes don't care in alarm A comparison"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MSK2_A::B_0X1
    }
}
#[doc = "Field `MSK2` writer - Alarm A minutes mask"]
pub type MSK2_W<'a, REG> = crate::BitWriter<'a, REG, MSK2_A>;
impl<'a, REG> MSK2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm A set if the minutes match"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSK2_A::B_0X0)
    }
    #[doc = "Minutes don't care in alarm A comparison"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSK2_A::B_0X1)
    }
}
#[doc = "Field `HU` reader - Hour units in BCD format"]
pub type HU_R = crate::FieldReader;
#[doc = "Field `HU` writer - Hour units in BCD format"]
pub type HU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HT` reader - Hour tens in BCD format"]
pub type HT_R = crate::FieldReader;
#[doc = "Field `HT` writer - Hour tens in BCD format"]
pub type HT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
#[doc = "Field `PM` writer - AM/PM notation"]
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG, PM_A>;
impl<'a, REG> PM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AM or 24-hour format"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::B_0X0)
    }
    #[doc = "PM"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::B_0X1)
    }
}
#[doc = "Field `MSK3` reader - Alarm A hours mask"]
pub type MSK3_R = crate::BitReader<MSK3_A>;
#[doc = "Alarm A hours mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSK3_A {
    #[doc = "0: Alarm A set if the hours match"]
    B_0X0 = 0,
    #[doc = "1: Hours don't care in alarm A comparison"]
    B_0X1 = 1,
}
impl From<MSK3_A> for bool {
    #[inline(always)]
    fn from(variant: MSK3_A) -> Self {
        variant as u8 != 0
    }
}
impl MSK3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSK3_A {
        match self.bits {
            false => MSK3_A::B_0X0,
            true => MSK3_A::B_0X1,
        }
    }
    #[doc = "Alarm A set if the hours match"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MSK3_A::B_0X0
    }
    #[doc = "Hours don't care in alarm A comparison"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MSK3_A::B_0X1
    }
}
#[doc = "Field `MSK3` writer - Alarm A hours mask"]
pub type MSK3_W<'a, REG> = crate::BitWriter<'a, REG, MSK3_A>;
impl<'a, REG> MSK3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm A set if the hours match"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSK3_A::B_0X0)
    }
    #[doc = "Hours don't care in alarm A comparison"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSK3_A::B_0X1)
    }
}
#[doc = "Field `DU` reader - Date units or day in BCD format"]
pub type DU_R = crate::FieldReader;
#[doc = "Field `DU` writer - Date units or day in BCD format"]
pub type DU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DT` reader - Date tens in BCD format"]
pub type DT_R = crate::FieldReader;
#[doc = "Field `DT` writer - Date tens in BCD format"]
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WDSEL` reader - Week day selection"]
pub type WDSEL_R = crate::BitReader<WDSEL_A>;
#[doc = "Week day selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDSEL_A {
    #[doc = "0: DU\\[3:0\\]
represents the date units"]
    B_0X0 = 0,
    #[doc = "1: DU\\[3:0\\]
represents the week day. DT\\[1:0\\]
is don't care."]
    B_0X1 = 1,
}
impl From<WDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: WDSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl WDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDSEL_A {
        match self.bits {
            false => WDSEL_A::B_0X0,
            true => WDSEL_A::B_0X1,
        }
    }
    #[doc = "DU\\[3:0\\]
represents the date units"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WDSEL_A::B_0X0
    }
    #[doc = "DU\\[3:0\\]
represents the week day. DT\\[1:0\\]
is don't care."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WDSEL_A::B_0X1
    }
}
#[doc = "Field `WDSEL` writer - Week day selection"]
pub type WDSEL_W<'a, REG> = crate::BitWriter<'a, REG, WDSEL_A>;
impl<'a, REG> WDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DU\\[3:0\\]
represents the date units"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WDSEL_A::B_0X0)
    }
    #[doc = "DU\\[3:0\\]
represents the week day. DT\\[1:0\\]
is don't care."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WDSEL_A::B_0X1)
    }
}
#[doc = "Field `MSK4` reader - Alarm A date mask"]
pub type MSK4_R = crate::BitReader<MSK4_A>;
#[doc = "Alarm A date mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSK4_A {
    #[doc = "0: Alarm A set if the date/day match"]
    B_0X0 = 0,
    #[doc = "1: Date/day don't care in alarm A comparison"]
    B_0X1 = 1,
}
impl From<MSK4_A> for bool {
    #[inline(always)]
    fn from(variant: MSK4_A) -> Self {
        variant as u8 != 0
    }
}
impl MSK4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSK4_A {
        match self.bits {
            false => MSK4_A::B_0X0,
            true => MSK4_A::B_0X1,
        }
    }
    #[doc = "Alarm A set if the date/day match"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MSK4_A::B_0X0
    }
    #[doc = "Date/day don't care in alarm A comparison"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MSK4_A::B_0X1
    }
}
#[doc = "Field `MSK4` writer - Alarm A date mask"]
pub type MSK4_W<'a, REG> = crate::BitWriter<'a, REG, MSK4_A>;
impl<'a, REG> MSK4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm A set if the date/day match"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSK4_A::B_0X0)
    }
    #[doc = "Date/day don't care in alarm A comparison"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSK4_A::B_0X1)
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
    #[doc = "Bit 7 - Alarm A seconds mask"]
    #[inline(always)]
    pub fn msk1(&self) -> MSK1_R {
        MSK1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Alarm A minutes mask"]
    #[inline(always)]
    pub fn msk2(&self) -> MSK2_R {
        MSK2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Alarm A hours mask"]
    #[inline(always)]
    pub fn msk3(&self) -> MSK3_R {
        MSK3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format"]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    pub fn wdsel(&self) -> WDSEL_R {
        WDSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Alarm A date mask"]
    #[inline(always)]
    pub fn msk4(&self) -> MSK4_R {
        MSK4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn su(&mut self) -> SU_W<RTC_ALRMAR_SPEC> {
        SU_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format."]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<RTC_ALRMAR_SPEC> {
        ST_W::new(self, 4)
    }
    #[doc = "Bit 7 - Alarm A seconds mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk1(&mut self) -> MSK1_W<RTC_ALRMAR_SPEC> {
        MSK1_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn mnu(&mut self) -> MNU_W<RTC_ALRMAR_SPEC> {
        MNU_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn mnt(&mut self) -> MNT_W<RTC_ALRMAR_SPEC> {
        MNT_W::new(self, 12)
    }
    #[doc = "Bit 15 - Alarm A minutes mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk2(&mut self) -> MSK2_W<RTC_ALRMAR_SPEC> {
        MSK2_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HU_W<RTC_ALRMAR_SPEC> {
        HU_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<RTC_ALRMAR_SPEC> {
        HT_W::new(self, 20)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<RTC_ALRMAR_SPEC> {
        PM_W::new(self, 22)
    }
    #[doc = "Bit 23 - Alarm A hours mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk3(&mut self) -> MSK3_W<RTC_ALRMAR_SPEC> {
        MSK3_W::new(self, 23)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DU_W<RTC_ALRMAR_SPEC> {
        DU_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<RTC_ALRMAR_SPEC> {
        DT_W::new(self, 28)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    #[must_use]
    pub fn wdsel(&mut self) -> WDSEL_W<RTC_ALRMAR_SPEC> {
        WDSEL_W::new(self, 30)
    }
    #[doc = "Bit 31 - Alarm A date mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk4(&mut self) -> MSK4_W<RTC_ALRMAR_SPEC> {
        MSK4_W::new(self, 31)
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
#[doc = "RTC alarm A register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_alrmar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_alrmar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_ALRMAR_SPEC;
impl crate::RegisterSpec for RTC_ALRMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_alrmar::R`](R) reader structure"]
impl crate::Readable for RTC_ALRMAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_alrmar::W`](W) writer structure"]
impl crate::Writable for RTC_ALRMAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_ALRMAR to value 0"]
impl crate::Resettable for RTC_ALRMAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
