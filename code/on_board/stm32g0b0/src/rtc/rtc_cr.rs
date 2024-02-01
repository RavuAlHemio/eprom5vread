#[doc = "Register `RTC_CR` reader"]
pub type R = crate::R<RTC_CR_SPEC>;
#[doc = "Register `RTC_CR` writer"]
pub type W = crate::W<RTC_CR_SPEC>;
#[doc = "Field `WUCKSEL` reader - ck_wut wakeup clock selection 10x: ck_spre (usually 1Hz) clock is selected 11x: ck_spre (usually 1Hz) clock is selected and 216is added to the WUT counter value"]
pub type WUCKSEL_R = crate::FieldReader<WUCKSEL_A>;
#[doc = "ck_wut wakeup clock selection 10x: ck_spre (usually 1Hz) clock is selected 11x: ck_spre (usually 1Hz) clock is selected and 216is added to the WUT counter value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUCKSEL_A {
    #[doc = "0: RTC/16 clock is selected"]
    B_0X0 = 0,
    #[doc = "1: RTC/8 clock is selected"]
    B_0X1 = 1,
    #[doc = "2: RTC/4 clock is selected"]
    B_0X2 = 2,
    #[doc = "3: RTC/2 clock is selected"]
    B_0X3 = 3,
}
impl From<WUCKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WUCKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WUCKSEL_A {
    type Ux = u8;
}
impl WUCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUCKSEL_A> {
        match self.bits {
            0 => Some(WUCKSEL_A::B_0X0),
            1 => Some(WUCKSEL_A::B_0X1),
            2 => Some(WUCKSEL_A::B_0X2),
            3 => Some(WUCKSEL_A::B_0X3),
            _ => None,
        }
    }
    #[doc = "RTC/16 clock is selected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WUCKSEL_A::B_0X0
    }
    #[doc = "RTC/8 clock is selected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WUCKSEL_A::B_0X1
    }
    #[doc = "RTC/4 clock is selected"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == WUCKSEL_A::B_0X2
    }
    #[doc = "RTC/2 clock is selected"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == WUCKSEL_A::B_0X3
    }
}
#[doc = "Field `WUCKSEL` writer - ck_wut wakeup clock selection 10x: ck_spre (usually 1Hz) clock is selected 11x: ck_spre (usually 1Hz) clock is selected and 216is added to the WUT counter value"]
pub type WUCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WUCKSEL_A>;
impl<'a, REG> WUCKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RTC/16 clock is selected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUCKSEL_A::B_0X0)
    }
    #[doc = "RTC/8 clock is selected"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUCKSEL_A::B_0X1)
    }
    #[doc = "RTC/4 clock is selected"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(WUCKSEL_A::B_0X2)
    }
    #[doc = "RTC/2 clock is selected"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(WUCKSEL_A::B_0X3)
    }
}
#[doc = "Field `TSEDGE` reader - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
pub type TSEDGE_R = crate::BitReader<TSEDGE_A>;
#[doc = "Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEDGE_A {
    #[doc = "0: RTC_TS input rising edge generates a timestamp event"]
    B_0X0 = 0,
    #[doc = "1: RTC_TS input falling edge generates a timestamp event"]
    B_0X1 = 1,
}
impl From<TSEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: TSEDGE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSEDGE_A {
        match self.bits {
            false => TSEDGE_A::B_0X0,
            true => TSEDGE_A::B_0X1,
        }
    }
    #[doc = "RTC_TS input rising edge generates a timestamp event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSEDGE_A::B_0X0
    }
    #[doc = "RTC_TS input falling edge generates a timestamp event"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSEDGE_A::B_0X1
    }
}
#[doc = "Field `TSEDGE` writer - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
pub type TSEDGE_W<'a, REG> = crate::BitWriter<'a, REG, TSEDGE_A>;
impl<'a, REG> TSEDGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC_TS input rising edge generates a timestamp event"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSEDGE_A::B_0X0)
    }
    #[doc = "RTC_TS input falling edge generates a timestamp event"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSEDGE_A::B_0X1)
    }
}
#[doc = "Field `REFCKON` reader - RTC_REFIN reference clock detection enable (50 or 60Hz) Note: PREDIV_S must be 0x00FF."]
pub type REFCKON_R = crate::BitReader<REFCKON_A>;
#[doc = "RTC_REFIN reference clock detection enable (50 or 60Hz) Note: PREDIV_S must be 0x00FF.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFCKON_A {
    #[doc = "0: RTC_REFIN detection disabled"]
    B_0X0 = 0,
    #[doc = "1: RTC_REFIN detection enabled"]
    B_0X1 = 1,
}
impl From<REFCKON_A> for bool {
    #[inline(always)]
    fn from(variant: REFCKON_A) -> Self {
        variant as u8 != 0
    }
}
impl REFCKON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REFCKON_A {
        match self.bits {
            false => REFCKON_A::B_0X0,
            true => REFCKON_A::B_0X1,
        }
    }
    #[doc = "RTC_REFIN detection disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == REFCKON_A::B_0X0
    }
    #[doc = "RTC_REFIN detection enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == REFCKON_A::B_0X1
    }
}
#[doc = "Field `REFCKON` writer - RTC_REFIN reference clock detection enable (50 or 60Hz) Note: PREDIV_S must be 0x00FF."]
pub type REFCKON_W<'a, REG> = crate::BitWriter<'a, REG, REFCKON_A>;
impl<'a, REG> REFCKON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC_REFIN detection disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(REFCKON_A::B_0X0)
    }
    #[doc = "RTC_REFIN detection enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(REFCKON_A::B_0X1)
    }
}
#[doc = "Field `BYPSHAD` reader - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
pub type BYPSHAD_R = crate::BitReader<BYPSHAD_A>;
#[doc = "Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPSHAD_A {
    #[doc = "0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles."]
    B_0X0 = 0,
    #[doc = "1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters."]
    B_0X1 = 1,
}
impl From<BYPSHAD_A> for bool {
    #[inline(always)]
    fn from(variant: BYPSHAD_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPSHAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BYPSHAD_A {
        match self.bits {
            false => BYPSHAD_A::B_0X0,
            true => BYPSHAD_A::B_0X1,
        }
    }
    #[doc = "Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BYPSHAD_A::B_0X0
    }
    #[doc = "Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BYPSHAD_A::B_0X1
    }
}
#[doc = "Field `BYPSHAD` writer - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
pub type BYPSHAD_W<'a, REG> = crate::BitWriter<'a, REG, BYPSHAD_A>;
impl<'a, REG> BYPSHAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BYPSHAD_A::B_0X0)
    }
    #[doc = "Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BYPSHAD_A::B_0X1)
    }
}
#[doc = "Field `FMT` reader - Hour format"]
pub type FMT_R = crate::BitReader<FMT_A>;
#[doc = "Hour format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMT_A {
    #[doc = "0: 24 hour/day format"]
    B_0X0 = 0,
    #[doc = "1: AM/PM hour format"]
    B_0X1 = 1,
}
impl From<FMT_A> for bool {
    #[inline(always)]
    fn from(variant: FMT_A) -> Self {
        variant as u8 != 0
    }
}
impl FMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FMT_A {
        match self.bits {
            false => FMT_A::B_0X0,
            true => FMT_A::B_0X1,
        }
    }
    #[doc = "24 hour/day format"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FMT_A::B_0X0
    }
    #[doc = "AM/PM hour format"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FMT_A::B_0X1
    }
}
#[doc = "Field `FMT` writer - Hour format"]
pub type FMT_W<'a, REG> = crate::BitWriter<'a, REG, FMT_A>;
impl<'a, REG> FMT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "24 hour/day format"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FMT_A::B_0X0)
    }
    #[doc = "AM/PM hour format"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FMT_A::B_0X1)
    }
}
#[doc = "Field `ALRAE` reader - Alarm A enable"]
pub type ALRAE_R = crate::BitReader<ALRAE_A>;
#[doc = "Alarm A enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAE_A {
    #[doc = "0: Alarm A disabled"]
    B_0X0 = 0,
    #[doc = "1: Alarm A enabled"]
    B_0X1 = 1,
}
impl From<ALRAE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALRAE_A {
        match self.bits {
            false => ALRAE_A::B_0X0,
            true => ALRAE_A::B_0X1,
        }
    }
    #[doc = "Alarm A disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ALRAE_A::B_0X0
    }
    #[doc = "Alarm A enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ALRAE_A::B_0X1
    }
}
#[doc = "Field `ALRAE` writer - Alarm A enable"]
pub type ALRAE_W<'a, REG> = crate::BitWriter<'a, REG, ALRAE_A>;
impl<'a, REG> ALRAE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm A disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAE_A::B_0X0)
    }
    #[doc = "Alarm A enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAE_A::B_0X1)
    }
}
#[doc = "Field `ALRBE` reader - Alarm B enable"]
pub type ALRBE_R = crate::BitReader<ALRBE_A>;
#[doc = "Alarm B enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRBE_A {
    #[doc = "0: Alarm B disabled"]
    B_0X0 = 0,
    #[doc = "1: Alarm B enabled"]
    B_0X1 = 1,
}
impl From<ALRBE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALRBE_A {
        match self.bits {
            false => ALRBE_A::B_0X0,
            true => ALRBE_A::B_0X1,
        }
    }
    #[doc = "Alarm B disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ALRBE_A::B_0X0
    }
    #[doc = "Alarm B enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ALRBE_A::B_0X1
    }
}
#[doc = "Field `ALRBE` writer - Alarm B enable"]
pub type ALRBE_W<'a, REG> = crate::BitWriter<'a, REG, ALRBE_A>;
impl<'a, REG> ALRBE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm B disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ALRBE_A::B_0X0)
    }
    #[doc = "Alarm B enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ALRBE_A::B_0X1)
    }
}
#[doc = "Field `WUTE` reader - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again."]
pub type WUTE_R = crate::BitReader<WUTE_A>;
#[doc = "Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTE_A {
    #[doc = "0: Wakeup timer disabled"]
    B_0X0 = 0,
    #[doc = "1: Wakeup timer enabled"]
    B_0X1 = 1,
}
impl From<WUTE_A> for bool {
    #[inline(always)]
    fn from(variant: WUTE_A) -> Self {
        variant as u8 != 0
    }
}
impl WUTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUTE_A {
        match self.bits {
            false => WUTE_A::B_0X0,
            true => WUTE_A::B_0X1,
        }
    }
    #[doc = "Wakeup timer disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WUTE_A::B_0X0
    }
    #[doc = "Wakeup timer enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WUTE_A::B_0X1
    }
}
#[doc = "Field `WUTE` writer - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again."]
pub type WUTE_W<'a, REG> = crate::BitWriter<'a, REG, WUTE_A>;
impl<'a, REG> WUTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup timer disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUTE_A::B_0X0)
    }
    #[doc = "Wakeup timer enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUTE_A::B_0X1)
    }
}
#[doc = "Field `TSE` reader - timestamp enable"]
pub type TSE_R = crate::BitReader<TSE_A>;
#[doc = "timestamp enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSE_A {
    #[doc = "0: timestamp disable"]
    B_0X0 = 0,
    #[doc = "1: timestamp enable"]
    B_0X1 = 1,
}
impl From<TSE_A> for bool {
    #[inline(always)]
    fn from(variant: TSE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSE_A {
        match self.bits {
            false => TSE_A::B_0X0,
            true => TSE_A::B_0X1,
        }
    }
    #[doc = "timestamp disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSE_A::B_0X0
    }
    #[doc = "timestamp enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSE_A::B_0X1
    }
}
#[doc = "Field `TSE` writer - timestamp enable"]
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG, TSE_A>;
impl<'a, REG> TSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "timestamp disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSE_A::B_0X0)
    }
    #[doc = "timestamp enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSE_A::B_0X1)
    }
}
#[doc = "Field `ALRAIE` reader - Alarm A interrupt enable"]
pub type ALRAIE_R = crate::BitReader<ALRAIE_A>;
#[doc = "Alarm A interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAIE_A {
    #[doc = "0: Alarm A interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Alarm A interrupt enabled"]
    B_0X1 = 1,
}
impl From<ALRAIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRAIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALRAIE_A {
        match self.bits {
            false => ALRAIE_A::B_0X0,
            true => ALRAIE_A::B_0X1,
        }
    }
    #[doc = "Alarm A interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ALRAIE_A::B_0X0
    }
    #[doc = "Alarm A interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ALRAIE_A::B_0X1
    }
}
#[doc = "Field `ALRAIE` writer - Alarm A interrupt enable"]
pub type ALRAIE_W<'a, REG> = crate::BitWriter<'a, REG, ALRAIE_A>;
impl<'a, REG> ALRAIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm A interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAIE_A::B_0X0)
    }
    #[doc = "Alarm A interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAIE_A::B_0X1)
    }
}
#[doc = "Field `ALRBIE` reader - Alarm B interrupt enable"]
pub type ALRBIE_R = crate::BitReader<ALRBIE_A>;
#[doc = "Alarm B interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRBIE_A {
    #[doc = "0: Alarm B interrupt disable"]
    B_0X0 = 0,
    #[doc = "1: Alarm B interrupt enable"]
    B_0X1 = 1,
}
impl From<ALRBIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALRBIE_A {
        match self.bits {
            false => ALRBIE_A::B_0X0,
            true => ALRBIE_A::B_0X1,
        }
    }
    #[doc = "Alarm B interrupt disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ALRBIE_A::B_0X0
    }
    #[doc = "Alarm B interrupt enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ALRBIE_A::B_0X1
    }
}
#[doc = "Field `ALRBIE` writer - Alarm B interrupt enable"]
pub type ALRBIE_W<'a, REG> = crate::BitWriter<'a, REG, ALRBIE_A>;
impl<'a, REG> ALRBIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm B interrupt disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ALRBIE_A::B_0X0)
    }
    #[doc = "Alarm B interrupt enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ALRBIE_A::B_0X1)
    }
}
#[doc = "Field `WUTIE` reader - Wakeup timer interrupt enable"]
pub type WUTIE_R = crate::BitReader<WUTIE_A>;
#[doc = "Wakeup timer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTIE_A {
    #[doc = "0: Wakeup timer interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: Wakeup timer interrupt enabled"]
    B_0X1 = 1,
}
impl From<WUTIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl WUTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUTIE_A {
        match self.bits {
            false => WUTIE_A::B_0X0,
            true => WUTIE_A::B_0X1,
        }
    }
    #[doc = "Wakeup timer interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WUTIE_A::B_0X0
    }
    #[doc = "Wakeup timer interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WUTIE_A::B_0X1
    }
}
#[doc = "Field `WUTIE` writer - Wakeup timer interrupt enable"]
pub type WUTIE_W<'a, REG> = crate::BitWriter<'a, REG, WUTIE_A>;
impl<'a, REG> WUTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup timer interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUTIE_A::B_0X0)
    }
    #[doc = "Wakeup timer interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUTIE_A::B_0X1)
    }
}
#[doc = "Field `TSIE` reader - Timestamp interrupt enable"]
pub type TSIE_R = crate::BitReader<TSIE_A>;
#[doc = "Timestamp interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSIE_A {
    #[doc = "0: Timestamp interrupt disable"]
    B_0X0 = 0,
    #[doc = "1: Timestamp interrupt enable"]
    B_0X1 = 1,
}
impl From<TSIE_A> for bool {
    #[inline(always)]
    fn from(variant: TSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSIE_A {
        match self.bits {
            false => TSIE_A::B_0X0,
            true => TSIE_A::B_0X1,
        }
    }
    #[doc = "Timestamp interrupt disable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSIE_A::B_0X0
    }
    #[doc = "Timestamp interrupt enable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSIE_A::B_0X1
    }
}
#[doc = "Field `TSIE` writer - Timestamp interrupt enable"]
pub type TSIE_W<'a, REG> = crate::BitWriter<'a, REG, TSIE_A>;
impl<'a, REG> TSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timestamp interrupt disable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSIE_A::B_0X0)
    }
    #[doc = "Timestamp interrupt enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSIE_A::B_0X1)
    }
}
#[doc = "Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD1H_AW {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Adds 1 hour to the current time. This can be used for summer time change"]
    B_0X1 = 1,
}
impl From<ADD1H_AW> for bool {
    #[inline(always)]
    fn from(variant: ADD1H_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD1H` writer - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0."]
pub type ADD1H_W<'a, REG> = crate::BitWriter<'a, REG, ADD1H_AW>;
impl<'a, REG> ADD1H_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADD1H_AW::B_0X0)
    }
    #[doc = "Adds 1 hour to the current time. This can be used for summer time change"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADD1H_AW::B_0X1)
    }
}
#[doc = "Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUB1H_AW {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Subtracts 1 hour to the current time. This can be used for winter time change."]
    B_0X1 = 1,
}
impl From<SUB1H_AW> for bool {
    #[inline(always)]
    fn from(variant: SUB1H_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUB1H` writer - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0."]
pub type SUB1H_W<'a, REG> = crate::BitWriter<'a, REG, SUB1H_AW>;
impl<'a, REG> SUB1H_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SUB1H_AW::B_0X0)
    }
    #[doc = "Subtracts 1 hour to the current time. This can be used for winter time change."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SUB1H_AW::B_0X1)
    }
}
#[doc = "Field `BKP` reader - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
pub type BKP_R = crate::BitReader;
#[doc = "Field `BKP` writer - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COSEL` reader - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to ."]
pub type COSEL_R = crate::BitReader<COSEL_A>;
#[doc = "Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COSEL_A {
    #[doc = "0: Calibration output is 512Hz"]
    B_0X0 = 0,
    #[doc = "1: Calibration output is 1Hz"]
    B_0X1 = 1,
}
impl From<COSEL_A> for bool {
    #[inline(always)]
    fn from(variant: COSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl COSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COSEL_A {
        match self.bits {
            false => COSEL_A::B_0X0,
            true => COSEL_A::B_0X1,
        }
    }
    #[doc = "Calibration output is 512Hz"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == COSEL_A::B_0X0
    }
    #[doc = "Calibration output is 1Hz"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == COSEL_A::B_0X1
    }
}
#[doc = "Field `COSEL` writer - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to ."]
pub type COSEL_W<'a, REG> = crate::BitWriter<'a, REG, COSEL_A>;
impl<'a, REG> COSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration output is 512Hz"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COSEL_A::B_0X0)
    }
    #[doc = "Calibration output is 1Hz"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COSEL_A::B_0X1)
    }
}
#[doc = "Field `POL` reader - Output polarity This bit is used to configure the polarity of TAMPALRM output."]
pub type POL_R = crate::BitReader<POL_A>;
#[doc = "Output polarity This bit is used to configure the polarity of TAMPALRM output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_A {
    #[doc = "0: The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1)."]
    B_0X0 = 0,
    #[doc = "1: The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1)."]
    B_0X1 = 1,
}
impl From<POL_A> for bool {
    #[inline(always)]
    fn from(variant: POL_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POL_A {
        match self.bits {
            false => POL_A::B_0X0,
            true => POL_A::B_0X1,
        }
    }
    #[doc = "The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1)."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == POL_A::B_0X0
    }
    #[doc = "The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == POL_A::B_0X1
    }
}
#[doc = "Field `POL` writer - Output polarity This bit is used to configure the polarity of TAMPALRM output."]
pub type POL_W<'a, REG> = crate::BitWriter<'a, REG, POL_A>;
impl<'a, REG> POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(POL_A::B_0X0)
    }
    #[doc = "The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(POL_A::B_0X1)
    }
}
#[doc = "Field `OSEL` reader - Output selection These bits are used to select the flag to be routed to TAMPALRM output."]
pub type OSEL_R = crate::FieldReader<OSEL_A>;
#[doc = "Output selection These bits are used to select the flag to be routed to TAMPALRM output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSEL_A {
    #[doc = "0: Output disabled"]
    B_0X0 = 0,
    #[doc = "1: Alarm A output enabled"]
    B_0X1 = 1,
    #[doc = "2: Alarm B output enabled"]
    B_0X2 = 2,
    #[doc = "3: Wakeup output enabled"]
    B_0X3 = 3,
}
impl From<OSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSEL_A {
    type Ux = u8;
}
impl OSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSEL_A {
        match self.bits {
            0 => OSEL_A::B_0X0,
            1 => OSEL_A::B_0X1,
            2 => OSEL_A::B_0X2,
            3 => OSEL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSEL_A::B_0X0
    }
    #[doc = "Alarm A output enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSEL_A::B_0X1
    }
    #[doc = "Alarm B output enabled"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OSEL_A::B_0X2
    }
    #[doc = "Wakeup output enabled"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OSEL_A::B_0X3
    }
}
#[doc = "Field `OSEL` writer - Output selection These bits are used to select the flag to be routed to TAMPALRM output."]
pub type OSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, OSEL_A>;
impl<'a, REG> OSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSEL_A::B_0X0)
    }
    #[doc = "Alarm A output enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSEL_A::B_0X1)
    }
    #[doc = "Alarm B output enabled"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSEL_A::B_0X2)
    }
    #[doc = "Wakeup output enabled"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSEL_A::B_0X3)
    }
}
#[doc = "Field `COE` reader - Calibration output enable This bit enables the CALIB output"]
pub type COE_R = crate::BitReader<COE_A>;
#[doc = "Calibration output enable This bit enables the CALIB output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COE_A {
    #[doc = "0: Calibration output disabled"]
    B_0X0 = 0,
    #[doc = "1: Calibration output enabled"]
    B_0X1 = 1,
}
impl From<COE_A> for bool {
    #[inline(always)]
    fn from(variant: COE_A) -> Self {
        variant as u8 != 0
    }
}
impl COE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COE_A {
        match self.bits {
            false => COE_A::B_0X0,
            true => COE_A::B_0X1,
        }
    }
    #[doc = "Calibration output disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == COE_A::B_0X0
    }
    #[doc = "Calibration output enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == COE_A::B_0X1
    }
}
#[doc = "Field `COE` writer - Calibration output enable This bit enables the CALIB output"]
pub type COE_W<'a, REG> = crate::BitWriter<'a, REG, COE_A>;
impl<'a, REG> COE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration output disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COE_A::B_0X0)
    }
    #[doc = "Calibration output enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COE_A::B_0X1)
    }
}
#[doc = "Field `ITSE` reader - timestamp on internal event enable"]
pub type ITSE_R = crate::BitReader<ITSE_A>;
#[doc = "timestamp on internal event enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITSE_A {
    #[doc = "0: internal event timestamp disabled"]
    B_0X0 = 0,
    #[doc = "1: internal event timestamp enabled"]
    B_0X1 = 1,
}
impl From<ITSE_A> for bool {
    #[inline(always)]
    fn from(variant: ITSE_A) -> Self {
        variant as u8 != 0
    }
}
impl ITSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITSE_A {
        match self.bits {
            false => ITSE_A::B_0X0,
            true => ITSE_A::B_0X1,
        }
    }
    #[doc = "internal event timestamp disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ITSE_A::B_0X0
    }
    #[doc = "internal event timestamp enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ITSE_A::B_0X1
    }
}
#[doc = "Field `ITSE` writer - timestamp on internal event enable"]
pub type ITSE_W<'a, REG> = crate::BitWriter<'a, REG, ITSE_A>;
impl<'a, REG> ITSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "internal event timestamp disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITSE_A::B_0X0)
    }
    #[doc = "internal event timestamp enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITSE_A::B_0X1)
    }
}
#[doc = "Field `TAMPTS` reader - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts."]
pub type TAMPTS_R = crate::BitReader<TAMPTS_A>;
#[doc = "Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPTS_A {
    #[doc = "0: Tamper detection event does not cause a RTC timestamp to be saved"]
    B_0X0 = 0,
    #[doc = "1: Save RTC timestamp on tamper detection event"]
    B_0X1 = 1,
}
impl From<TAMPTS_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPTS_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMPTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMPTS_A {
        match self.bits {
            false => TAMPTS_A::B_0X0,
            true => TAMPTS_A::B_0X1,
        }
    }
    #[doc = "Tamper detection event does not cause a RTC timestamp to be saved"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TAMPTS_A::B_0X0
    }
    #[doc = "Save RTC timestamp on tamper detection event"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TAMPTS_A::B_0X1
    }
}
#[doc = "Field `TAMPTS` writer - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts."]
pub type TAMPTS_W<'a, REG> = crate::BitWriter<'a, REG, TAMPTS_A>;
impl<'a, REG> TAMPTS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper detection event does not cause a RTC timestamp to be saved"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPTS_A::B_0X0)
    }
    #[doc = "Save RTC timestamp on tamper detection event"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPTS_A::B_0X1)
    }
}
#[doc = "Field `TAMPOE` reader - Tamper detection output enable on TAMPALRM"]
pub type TAMPOE_R = crate::BitReader<TAMPOE_A>;
#[doc = "Tamper detection output enable on TAMPALRM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPOE_A {
    #[doc = "0: The tamper flag is not routed on TAMPALRM"]
    B_0X0 = 0,
    #[doc = "1: The tamper flag is routed on TAMPALRM, combined with the signal provided by OSEL and with the polarity provided by POL."]
    B_0X1 = 1,
}
impl From<TAMPOE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPOE_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMPOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMPOE_A {
        match self.bits {
            false => TAMPOE_A::B_0X0,
            true => TAMPOE_A::B_0X1,
        }
    }
    #[doc = "The tamper flag is not routed on TAMPALRM"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TAMPOE_A::B_0X0
    }
    #[doc = "The tamper flag is routed on TAMPALRM, combined with the signal provided by OSEL and with the polarity provided by POL."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TAMPOE_A::B_0X1
    }
}
#[doc = "Field `TAMPOE` writer - Tamper detection output enable on TAMPALRM"]
pub type TAMPOE_W<'a, REG> = crate::BitWriter<'a, REG, TAMPOE_A>;
impl<'a, REG> TAMPOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The tamper flag is not routed on TAMPALRM"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPOE_A::B_0X0)
    }
    #[doc = "The tamper flag is routed on TAMPALRM, combined with the signal provided by OSEL and with the polarity provided by POL."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPOE_A::B_0X1)
    }
}
#[doc = "Field `TAMPALRM_PU` reader - TAMPALRM pull-up enable"]
pub type TAMPALRM_PU_R = crate::BitReader<TAMPALRM_PU_A>;
#[doc = "TAMPALRM pull-up enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPALRM_PU_A {
    #[doc = "0: No pull-up is applied on TAMPALRM output"]
    B_0X0 = 0,
    #[doc = "1: A pull-up is applied on TAMPALRM output"]
    B_0X1 = 1,
}
impl From<TAMPALRM_PU_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPALRM_PU_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMPALRM_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMPALRM_PU_A {
        match self.bits {
            false => TAMPALRM_PU_A::B_0X0,
            true => TAMPALRM_PU_A::B_0X1,
        }
    }
    #[doc = "No pull-up is applied on TAMPALRM output"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TAMPALRM_PU_A::B_0X0
    }
    #[doc = "A pull-up is applied on TAMPALRM output"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TAMPALRM_PU_A::B_0X1
    }
}
#[doc = "Field `TAMPALRM_PU` writer - TAMPALRM pull-up enable"]
pub type TAMPALRM_PU_W<'a, REG> = crate::BitWriter<'a, REG, TAMPALRM_PU_A>;
impl<'a, REG> TAMPALRM_PU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No pull-up is applied on TAMPALRM output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPALRM_PU_A::B_0X0)
    }
    #[doc = "A pull-up is applied on TAMPALRM output"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPALRM_PU_A::B_0X1)
    }
}
#[doc = "Field `TAMPALRM_TYPE` reader - TAMPALRM output type"]
pub type TAMPALRM_TYPE_R = crate::BitReader<TAMPALRM_TYPE_A>;
#[doc = "TAMPALRM output type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPALRM_TYPE_A {
    #[doc = "0: TAMPALRM is push-pull output"]
    B_0X0 = 0,
    #[doc = "1: TAMPALRM is open-drain output"]
    B_0X1 = 1,
}
impl From<TAMPALRM_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPALRM_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMPALRM_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMPALRM_TYPE_A {
        match self.bits {
            false => TAMPALRM_TYPE_A::B_0X0,
            true => TAMPALRM_TYPE_A::B_0X1,
        }
    }
    #[doc = "TAMPALRM is push-pull output"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TAMPALRM_TYPE_A::B_0X0
    }
    #[doc = "TAMPALRM is open-drain output"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TAMPALRM_TYPE_A::B_0X1
    }
}
#[doc = "Field `TAMPALRM_TYPE` writer - TAMPALRM output type"]
pub type TAMPALRM_TYPE_W<'a, REG> = crate::BitWriter<'a, REG, TAMPALRM_TYPE_A>;
impl<'a, REG> TAMPALRM_TYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TAMPALRM is push-pull output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPALRM_TYPE_A::B_0X0)
    }
    #[doc = "TAMPALRM is open-drain output"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPALRM_TYPE_A::B_0X1)
    }
}
#[doc = "Field `OUT2EN` reader - RTC_OUT2 output enable Setting this bit allows to remap the RTC outputs on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL different 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL different 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSEL different 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1."]
pub type OUT2EN_R = crate::BitReader;
#[doc = "Field `OUT2EN` writer - RTC_OUT2 output enable Setting this bit allows to remap the RTC outputs on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL different 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL different 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSEL different 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1."]
pub type OUT2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - ck_wut wakeup clock selection 10x: ck_spre (usually 1Hz) clock is selected 11x: ck_spre (usually 1Hz) clock is selected and 216is added to the WUT counter value"]
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60Hz) Note: PREDIV_S must be 0x00FF."]
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    pub fn alrbe(&self) -> ALRBE_R {
        ALRBE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again."]
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - timestamp enable"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    pub fn alrbie(&self) -> ALRBIE_R {
        ALRBIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to ."]
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output polarity This bit is used to configure the polarity of TAMPALRM output."]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Output selection These bits are used to select the flag to be routed to TAMPALRM output."]
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Calibration output enable This bit enables the CALIB output"]
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - timestamp on internal event enable"]
    #[inline(always)]
    pub fn itse(&self) -> ITSE_R {
        ITSE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts."]
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Tamper detection output enable on TAMPALRM"]
    #[inline(always)]
    pub fn tampoe(&self) -> TAMPOE_R {
        TAMPOE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - TAMPALRM pull-up enable"]
    #[inline(always)]
    pub fn tampalrm_pu(&self) -> TAMPALRM_PU_R {
        TAMPALRM_PU_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TAMPALRM output type"]
    #[inline(always)]
    pub fn tampalrm_type(&self) -> TAMPALRM_TYPE_R {
        TAMPALRM_TYPE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RTC_OUT2 output enable Setting this bit allows to remap the RTC outputs on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL different 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL different 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSEL different 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1."]
    #[inline(always)]
    pub fn out2en(&self) -> OUT2EN_R {
        OUT2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - ck_wut wakeup clock selection 10x: ck_spre (usually 1Hz) clock is selected 11x: ck_spre (usually 1Hz) clock is selected and 216is added to the WUT counter value"]
    #[inline(always)]
    #[must_use]
    pub fn wucksel(&mut self) -> WUCKSEL_W<RTC_CR_SPEC> {
        WUCKSEL_W::new(self, 0)
    }
    #[doc = "Bit 3 - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
    #[inline(always)]
    #[must_use]
    pub fn tsedge(&mut self) -> TSEDGE_W<RTC_CR_SPEC> {
        TSEDGE_W::new(self, 3)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60Hz) Note: PREDIV_S must be 0x00FF."]
    #[inline(always)]
    #[must_use]
    pub fn refckon(&mut self) -> REFCKON_W<RTC_CR_SPEC> {
        REFCKON_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn bypshad(&mut self) -> BYPSHAD_W<RTC_CR_SPEC> {
        BYPSHAD_W::new(self, 5)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    #[must_use]
    pub fn fmt(&mut self) -> FMT_W<RTC_CR_SPEC> {
        FMT_W::new(self, 6)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrae(&mut self) -> ALRAE_W<RTC_CR_SPEC> {
        ALRAE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrbe(&mut self) -> ALRBE_W<RTC_CR_SPEC> {
        ALRBE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again."]
    #[inline(always)]
    #[must_use]
    pub fn wute(&mut self) -> WUTE_W<RTC_CR_SPEC> {
        WUTE_W::new(self, 10)
    }
    #[doc = "Bit 11 - timestamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<RTC_CR_SPEC> {
        TSE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alraie(&mut self) -> ALRAIE_W<RTC_CR_SPEC> {
        ALRAIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrbie(&mut self) -> ALRBIE_W<RTC_CR_SPEC> {
        ALRBIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wutie(&mut self) -> WUTIE_W<RTC_CR_SPEC> {
        WUTIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TSIE_W<RTC_CR_SPEC> {
        TSIE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn add1h(&mut self) -> ADD1H_W<RTC_CR_SPEC> {
        ADD1H_W::new(self, 16)
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0."]
    #[inline(always)]
    #[must_use]
    pub fn sub1h(&mut self) -> SUB1H_W<RTC_CR_SPEC> {
        SUB1H_W::new(self, 17)
    }
    #[doc = "Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<RTC_CR_SPEC> {
        BKP_W::new(self, 18)
    }
    #[doc = "Bit 19 - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn cosel(&mut self) -> COSEL_W<RTC_CR_SPEC> {
        COSEL_W::new(self, 19)
    }
    #[doc = "Bit 20 - Output polarity This bit is used to configure the polarity of TAMPALRM output."]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<RTC_CR_SPEC> {
        POL_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - Output selection These bits are used to select the flag to be routed to TAMPALRM output."]
    #[inline(always)]
    #[must_use]
    pub fn osel(&mut self) -> OSEL_W<RTC_CR_SPEC> {
        OSEL_W::new(self, 21)
    }
    #[doc = "Bit 23 - Calibration output enable This bit enables the CALIB output"]
    #[inline(always)]
    #[must_use]
    pub fn coe(&mut self) -> COE_W<RTC_CR_SPEC> {
        COE_W::new(self, 23)
    }
    #[doc = "Bit 24 - timestamp on internal event enable"]
    #[inline(always)]
    #[must_use]
    pub fn itse(&mut self) -> ITSE_W<RTC_CR_SPEC> {
        ITSE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn tampts(&mut self) -> TAMPTS_W<RTC_CR_SPEC> {
        TAMPTS_W::new(self, 25)
    }
    #[doc = "Bit 26 - Tamper detection output enable on TAMPALRM"]
    #[inline(always)]
    #[must_use]
    pub fn tampoe(&mut self) -> TAMPOE_W<RTC_CR_SPEC> {
        TAMPOE_W::new(self, 26)
    }
    #[doc = "Bit 29 - TAMPALRM pull-up enable"]
    #[inline(always)]
    #[must_use]
    pub fn tampalrm_pu(&mut self) -> TAMPALRM_PU_W<RTC_CR_SPEC> {
        TAMPALRM_PU_W::new(self, 29)
    }
    #[doc = "Bit 30 - TAMPALRM output type"]
    #[inline(always)]
    #[must_use]
    pub fn tampalrm_type(&mut self) -> TAMPALRM_TYPE_W<RTC_CR_SPEC> {
        TAMPALRM_TYPE_W::new(self, 30)
    }
    #[doc = "Bit 31 - RTC_OUT2 output enable Setting this bit allows to remap the RTC outputs on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL different 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL different 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSEL different 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1."]
    #[inline(always)]
    #[must_use]
    pub fn out2en(&mut self) -> OUT2EN_W<RTC_CR_SPEC> {
        OUT2EN_W::new(self, 31)
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
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_CR_SPEC;
impl crate::RegisterSpec for RTC_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_cr::R`](R) reader structure"]
impl crate::Readable for RTC_CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_cr::W`](W) writer structure"]
impl crate::Writable for RTC_CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_CR to value 0"]
impl crate::Resettable for RTC_CR_SPEC {
    const RESET_VALUE: u32 = 0;
}
