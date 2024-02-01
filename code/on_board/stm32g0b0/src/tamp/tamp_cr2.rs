#[doc = "Register `TAMP_CR2` reader"]
pub type R = crate::R<TAMP_CR2_SPEC>;
#[doc = "Register `TAMP_CR2` writer"]
pub type W = crate::W<TAMP_CR2_SPEC>;
#[doc = "Field `TAMP1NOER` reader - Tamper 1 no erase"]
pub type TAMP1NOER_R = crate::BitReader<TAMP1NOER_A>;
#[doc = "Tamper 1 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1NOER_A {
    #[doc = "0: Tamper 1 event erases the backup registers."]
    B_0X0 = 0,
    #[doc = "1: Tamper 1 event does not erase the backup registers."]
    B_0X1 = 1,
}
impl From<TAMP1NOER_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1NOER_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMP1NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1NOER_A {
        match self.bits {
            false => TAMP1NOER_A::B_0X0,
            true => TAMP1NOER_A::B_0X1,
        }
    }
    #[doc = "Tamper 1 event erases the backup registers."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TAMP1NOER_A::B_0X0
    }
    #[doc = "Tamper 1 event does not erase the backup registers."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TAMP1NOER_A::B_0X1
    }
}
#[doc = "Field `TAMP1NOER` writer - Tamper 1 no erase"]
pub type TAMP1NOER_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1NOER_A>;
impl<'a, REG> TAMP1NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 1 event erases the backup registers."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1NOER_A::B_0X0)
    }
    #[doc = "Tamper 1 event does not erase the backup registers."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1NOER_A::B_0X1)
    }
}
#[doc = "Field `TAMP2NOER` reader - Tamper 2 no erase"]
pub type TAMP2NOER_R = crate::BitReader<TAMP2NOER_A>;
#[doc = "Tamper 2 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP2NOER_A {
    #[doc = "0: Tamper 2 event erases the backup registers."]
    B_0X0 = 0,
    #[doc = "1: Tamper 2 event does not erase the backup registers."]
    B_0X1 = 1,
}
impl From<TAMP2NOER_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP2NOER_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMP2NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP2NOER_A {
        match self.bits {
            false => TAMP2NOER_A::B_0X0,
            true => TAMP2NOER_A::B_0X1,
        }
    }
    #[doc = "Tamper 2 event erases the backup registers."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TAMP2NOER_A::B_0X0
    }
    #[doc = "Tamper 2 event does not erase the backup registers."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TAMP2NOER_A::B_0X1
    }
}
#[doc = "Field `TAMP2NOER` writer - Tamper 2 no erase"]
pub type TAMP2NOER_W<'a, REG> = crate::BitWriter<'a, REG, TAMP2NOER_A>;
impl<'a, REG> TAMP2NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 2 event erases the backup registers."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP2NOER_A::B_0X0)
    }
    #[doc = "Tamper 2 event does not erase the backup registers."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP2NOER_A::B_0X1)
    }
}
#[doc = "Field `TAMP3NOER` reader - Tamper 3 no erase"]
pub type TAMP3NOER_R = crate::BitReader<TAMP3NOER_A>;
#[doc = "Tamper 3 no erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP3NOER_A {
    #[doc = "0: Tamper 3 event erases the backup registers."]
    B_0X0 = 0,
    #[doc = "1: Tamper 3 event does not erase the backup registers."]
    B_0X1 = 1,
}
impl From<TAMP3NOER_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP3NOER_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMP3NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP3NOER_A {
        match self.bits {
            false => TAMP3NOER_A::B_0X0,
            true => TAMP3NOER_A::B_0X1,
        }
    }
    #[doc = "Tamper 3 event erases the backup registers."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TAMP3NOER_A::B_0X0
    }
    #[doc = "Tamper 3 event does not erase the backup registers."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TAMP3NOER_A::B_0X1
    }
}
#[doc = "Field `TAMP3NOER` writer - Tamper 3 no erase"]
pub type TAMP3NOER_W<'a, REG> = crate::BitWriter<'a, REG, TAMP3NOER_A>;
impl<'a, REG> TAMP3NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 3 event erases the backup registers."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP3NOER_A::B_0X0)
    }
    #[doc = "Tamper 3 event does not erase the backup registers."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP3NOER_A::B_0X1)
    }
}
#[doc = "Field `TAMP1MSK` reader - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
pub type TAMP1MSK_R = crate::BitReader<TAMP1MSK_A>;
#[doc = "Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1MSK_A {
    #[doc = "0: Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection."]
    B_0X0 = 0,
    #[doc = "1: Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware. The backup registers are not erased."]
    B_0X1 = 1,
}
impl From<TAMP1MSK_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1MSK_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMP1MSK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1MSK_A {
        match self.bits {
            false => TAMP1MSK_A::B_0X0,
            true => TAMP1MSK_A::B_0X1,
        }
    }
    #[doc = "Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TAMP1MSK_A::B_0X0
    }
    #[doc = "Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware. The backup registers are not erased."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TAMP1MSK_A::B_0X1
    }
}
#[doc = "Field `TAMP1MSK` writer - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
pub type TAMP1MSK_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1MSK_A>;
impl<'a, REG> TAMP1MSK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1MSK_A::B_0X0)
    }
    #[doc = "Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware. The backup registers are not erased."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1MSK_A::B_0X1)
    }
}
#[doc = "Field `TAMP2MSK` reader - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
pub type TAMP2MSK_R = crate::BitReader<TAMP2MSK_A>;
#[doc = "Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP2MSK_A {
    #[doc = "0: Tamper 2 event generates a trigger event and TAMP2F must be cleared by software to allow next tamper event detection."]
    B_0X0 = 0,
    #[doc = "1: Tamper 2 event generates a trigger event. TAMP2F is masked and internally cleared by hardware. The backup registers are not erased."]
    B_0X1 = 1,
}
impl From<TAMP2MSK_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP2MSK_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMP2MSK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP2MSK_A {
        match self.bits {
            false => TAMP2MSK_A::B_0X0,
            true => TAMP2MSK_A::B_0X1,
        }
    }
    #[doc = "Tamper 2 event generates a trigger event and TAMP2F must be cleared by software to allow next tamper event detection."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TAMP2MSK_A::B_0X0
    }
    #[doc = "Tamper 2 event generates a trigger event. TAMP2F is masked and internally cleared by hardware. The backup registers are not erased."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TAMP2MSK_A::B_0X1
    }
}
#[doc = "Field `TAMP2MSK` writer - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
pub type TAMP2MSK_W<'a, REG> = crate::BitWriter<'a, REG, TAMP2MSK_A>;
impl<'a, REG> TAMP2MSK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 2 event generates a trigger event and TAMP2F must be cleared by software to allow next tamper event detection."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP2MSK_A::B_0X0)
    }
    #[doc = "Tamper 2 event generates a trigger event. TAMP2F is masked and internally cleared by hardware. The backup registers are not erased."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP2MSK_A::B_0X1)
    }
}
#[doc = "Field `TAMP3MSK` reader - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set."]
pub type TAMP3MSK_R = crate::BitReader<TAMP3MSK_A>;
#[doc = "Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP3MSK_A {
    #[doc = "0: Tamper 3 event generates a trigger event and TAMP3F must be cleared by software to allow next tamper event detection."]
    B_0X0 = 0,
    #[doc = "1: Tamper 3 event generates a trigger event. TAMP3F is masked and internally cleared by hardware. The backup registers are not erased."]
    B_0X1 = 1,
}
impl From<TAMP3MSK_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP3MSK_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMP3MSK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP3MSK_A {
        match self.bits {
            false => TAMP3MSK_A::B_0X0,
            true => TAMP3MSK_A::B_0X1,
        }
    }
    #[doc = "Tamper 3 event generates a trigger event and TAMP3F must be cleared by software to allow next tamper event detection."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TAMP3MSK_A::B_0X0
    }
    #[doc = "Tamper 3 event generates a trigger event. TAMP3F is masked and internally cleared by hardware. The backup registers are not erased."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TAMP3MSK_A::B_0X1
    }
}
#[doc = "Field `TAMP3MSK` writer - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set."]
pub type TAMP3MSK_W<'a, REG> = crate::BitWriter<'a, REG, TAMP3MSK_A>;
impl<'a, REG> TAMP3MSK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 3 event generates a trigger event and TAMP3F must be cleared by software to allow next tamper event detection."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP3MSK_A::B_0X0)
    }
    #[doc = "Tamper 3 event generates a trigger event. TAMP3F is masked and internally cleared by hardware. The backup registers are not erased."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP3MSK_A::B_0X1)
    }
}
#[doc = "Field `TAMP1TRG` reader - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event."]
pub type TAMP1TRG_R = crate::BitReader<TAMP1TRG_A>;
#[doc = "Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1TRG_A {
    #[doc = "0: If TAMPFLT different 00 Tamper 1 input staying low triggers a tamper detection event."]
    B_0X0 = 0,
    #[doc = "1: If TAMPFLT = 00 Tamper 1 input staying high triggers a tamper detection event."]
    B_0X1 = 1,
}
impl From<TAMP1TRG_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1TRG_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMP1TRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1TRG_A {
        match self.bits {
            false => TAMP1TRG_A::B_0X0,
            true => TAMP1TRG_A::B_0X1,
        }
    }
    #[doc = "If TAMPFLT different 00 Tamper 1 input staying low triggers a tamper detection event."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TAMP1TRG_A::B_0X0
    }
    #[doc = "If TAMPFLT = 00 Tamper 1 input staying high triggers a tamper detection event."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TAMP1TRG_A::B_0X1
    }
}
#[doc = "Field `TAMP1TRG` writer - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event."]
pub type TAMP1TRG_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1TRG_A>;
impl<'a, REG> TAMP1TRG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If TAMPFLT different 00 Tamper 1 input staying low triggers a tamper detection event."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1TRG_A::B_0X0)
    }
    #[doc = "If TAMPFLT = 00 Tamper 1 input staying high triggers a tamper detection event."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1TRG_A::B_0X1)
    }
}
#[doc = "Field `TAMP2TRG` reader - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event."]
pub type TAMP2TRG_R = crate::BitReader<TAMP2TRG_A>;
#[doc = "Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP2TRG_A {
    #[doc = "0: If TAMPFLT different 00 Tamper 2 input staying low triggers a tamper detection event."]
    B_0X0 = 0,
    #[doc = "1: If TAMPFLT = 00 Tamper 2 input staying high triggers a tamper detection event."]
    B_0X1 = 1,
}
impl From<TAMP2TRG_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP2TRG_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMP2TRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP2TRG_A {
        match self.bits {
            false => TAMP2TRG_A::B_0X0,
            true => TAMP2TRG_A::B_0X1,
        }
    }
    #[doc = "If TAMPFLT different 00 Tamper 2 input staying low triggers a tamper detection event."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TAMP2TRG_A::B_0X0
    }
    #[doc = "If TAMPFLT = 00 Tamper 2 input staying high triggers a tamper detection event."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TAMP2TRG_A::B_0X1
    }
}
#[doc = "Field `TAMP2TRG` writer - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event."]
pub type TAMP2TRG_W<'a, REG> = crate::BitWriter<'a, REG, TAMP2TRG_A>;
impl<'a, REG> TAMP2TRG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If TAMPFLT different 00 Tamper 2 input staying low triggers a tamper detection event."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP2TRG_A::B_0X0)
    }
    #[doc = "If TAMPFLT = 00 Tamper 2 input staying high triggers a tamper detection event."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP2TRG_A::B_0X1)
    }
}
#[doc = "Field `TAMP3TRG` reader - Active level for tamper 3 input (active mode disabled) If TAMPFLT = 00 Tamper 3 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 3 input falling edge and low level triggers a tamper detection event."]
pub type TAMP3TRG_R = crate::BitReader<TAMP3TRG_A>;
#[doc = "Active level for tamper 3 input (active mode disabled) If TAMPFLT = 00 Tamper 3 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 3 input falling edge and low level triggers a tamper detection event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP3TRG_A {
    #[doc = "0: If TAMPFLT different 00 Tamper 3 input staying low triggers a tamper detection event."]
    B_0X0 = 0,
    #[doc = "1: If TAMPFLT =00 Tamper 3 input staying high triggers a tamper detection event."]
    B_0X1 = 1,
}
impl From<TAMP3TRG_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP3TRG_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMP3TRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP3TRG_A {
        match self.bits {
            false => TAMP3TRG_A::B_0X0,
            true => TAMP3TRG_A::B_0X1,
        }
    }
    #[doc = "If TAMPFLT different 00 Tamper 3 input staying low triggers a tamper detection event."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TAMP3TRG_A::B_0X0
    }
    #[doc = "If TAMPFLT =00 Tamper 3 input staying high triggers a tamper detection event."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TAMP3TRG_A::B_0X1
    }
}
#[doc = "Field `TAMP3TRG` writer - Active level for tamper 3 input (active mode disabled) If TAMPFLT = 00 Tamper 3 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 3 input falling edge and low level triggers a tamper detection event."]
pub type TAMP3TRG_W<'a, REG> = crate::BitWriter<'a, REG, TAMP3TRG_A>;
impl<'a, REG> TAMP3TRG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If TAMPFLT different 00 Tamper 3 input staying low triggers a tamper detection event."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP3TRG_A::B_0X0)
    }
    #[doc = "If TAMPFLT =00 Tamper 3 input staying high triggers a tamper detection event."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP3TRG_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - Tamper 1 no erase"]
    #[inline(always)]
    pub fn tamp1noer(&self) -> TAMP1NOER_R {
        TAMP1NOER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper 2 no erase"]
    #[inline(always)]
    pub fn tamp2noer(&self) -> TAMP2NOER_R {
        TAMP2NOER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper 3 no erase"]
    #[inline(always)]
    pub fn tamp3noer(&self) -> TAMP3NOER_R {
        TAMP3NOER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
    #[inline(always)]
    pub fn tamp1msk(&self) -> TAMP1MSK_R {
        TAMP1MSK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
    #[inline(always)]
    pub fn tamp2msk(&self) -> TAMP2MSK_R {
        TAMP2MSK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set."]
    #[inline(always)]
    pub fn tamp3msk(&self) -> TAMP3MSK_R {
        TAMP3MSK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Active level for tamper 3 input (active mode disabled) If TAMPFLT = 00 Tamper 3 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 3 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 1 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1noer(&mut self) -> TAMP1NOER_W<TAMP_CR2_SPEC> {
        TAMP1NOER_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper 2 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2noer(&mut self) -> TAMP2NOER_W<TAMP_CR2_SPEC> {
        TAMP2NOER_W::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper 3 no erase"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3noer(&mut self) -> TAMP3NOER_W<TAMP_CR2_SPEC> {
        TAMP3NOER_W::new(self, 2)
    }
    #[doc = "Bit 16 - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
    #[inline(always)]
    #[must_use]
    pub fn tamp1msk(&mut self) -> TAMP1MSK_W<TAMP_CR2_SPEC> {
        TAMP1MSK_W::new(self, 16)
    }
    #[doc = "Bit 17 - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
    #[inline(always)]
    #[must_use]
    pub fn tamp2msk(&mut self) -> TAMP2MSK_W<TAMP_CR2_SPEC> {
        TAMP2MSK_W::new(self, 17)
    }
    #[doc = "Bit 18 - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set."]
    #[inline(always)]
    #[must_use]
    pub fn tamp3msk(&mut self) -> TAMP3MSK_W<TAMP_CR2_SPEC> {
        TAMP3MSK_W::new(self, 18)
    }
    #[doc = "Bit 24 - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    #[must_use]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<TAMP_CR2_SPEC> {
        TAMP1TRG_W::new(self, 24)
    }
    #[doc = "Bit 25 - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    #[must_use]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W<TAMP_CR2_SPEC> {
        TAMP2TRG_W::new(self, 25)
    }
    #[doc = "Bit 26 - Active level for tamper 3 input (active mode disabled) If TAMPFLT = 00 Tamper 3 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 3 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    #[must_use]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W<TAMP_CR2_SPEC> {
        TAMP3TRG_W::new(self, 26)
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
#[doc = "TAMP control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAMP_CR2_SPEC;
impl crate::RegisterSpec for TAMP_CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp_cr2::R`](R) reader structure"]
impl crate::Readable for TAMP_CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tamp_cr2::W`](W) writer structure"]
impl crate::Writable for TAMP_CR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMP_CR2 to value 0"]
impl crate::Resettable for TAMP_CR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
