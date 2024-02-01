#[doc = "Register `DBG_APB_FZ1` reader"]
pub type R = crate::R<DBG_APB_FZ1_SPEC>;
#[doc = "Register `DBG_APB_FZ1` writer"]
pub type W = crate::W<DBG_APB_FZ1_SPEC>;
#[doc = "Field `DBG_TIM2_STOP` reader - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted:"]
pub type DBG_TIM2_STOP_R = crate::BitReader<DBG_TIM2_STOP_A>;
#[doc = "Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM2_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_TIM2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_TIM2_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM2_STOP_A {
        match self.bits {
            false => DBG_TIM2_STOP_A::B_0X0,
            true => DBG_TIM2_STOP_A::B_0X1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM2_STOP_A::B_0X0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM2_STOP_A::B_0X1
    }
}
#[doc = "Field `DBG_TIM2_STOP` writer - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted:"]
pub type DBG_TIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM2_STOP_A>;
impl<'a, REG> DBG_TIM2_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM2_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM2_STOP_A::B_0X1)
    }
}
#[doc = "Field `DBG_TIM3_STOP` reader - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:"]
pub type DBG_TIM3_STOP_R = crate::BitReader<DBG_TIM3_STOP_A>;
#[doc = "Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM3_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_TIM3_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM3_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_TIM3_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM3_STOP_A {
        match self.bits {
            false => DBG_TIM3_STOP_A::B_0X0,
            true => DBG_TIM3_STOP_A::B_0X1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM3_STOP_A::B_0X0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM3_STOP_A::B_0X1
    }
}
#[doc = "Field `DBG_TIM3_STOP` writer - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:"]
pub type DBG_TIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM3_STOP_A>;
impl<'a, REG> DBG_TIM3_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM3_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM3_STOP_A::B_0X1)
    }
}
#[doc = "Field `DBG_TIM6_STOP` reader - Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:"]
pub type DBG_TIM6_STOP_R = crate::BitReader<DBG_TIM6_STOP_A>;
#[doc = "Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM6_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_TIM6_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM6_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_TIM6_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM6_STOP_A {
        match self.bits {
            false => DBG_TIM6_STOP_A::B_0X0,
            true => DBG_TIM6_STOP_A::B_0X1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM6_STOP_A::B_0X0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM6_STOP_A::B_0X1
    }
}
#[doc = "Field `DBG_TIM6_STOP` writer - Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:"]
pub type DBG_TIM6_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM6_STOP_A>;
impl<'a, REG> DBG_TIM6_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM6_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM6_STOP_A::B_0X1)
    }
}
#[doc = "Field `DBG_TIM7_STOP` reader - Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:"]
pub type DBG_TIM7_STOP_R = crate::BitReader<DBG_TIM7_STOP_A>;
#[doc = "Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM7_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_TIM7_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM7_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_TIM7_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM7_STOP_A {
        match self.bits {
            false => DBG_TIM7_STOP_A::B_0X0,
            true => DBG_TIM7_STOP_A::B_0X1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM7_STOP_A::B_0X0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM7_STOP_A::B_0X1
    }
}
#[doc = "Field `DBG_TIM7_STOP` writer - Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:"]
pub type DBG_TIM7_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM7_STOP_A>;
impl<'a, REG> DBG_TIM7_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM7_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM7_STOP_A::B_0X1)
    }
}
#[doc = "Field `DBG_RTC_STOP` reader - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:"]
pub type DBG_RTC_STOP_R = crate::BitReader<DBG_RTC_STOP_A>;
#[doc = "Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_RTC_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_RTC_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_RTC_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_RTC_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_RTC_STOP_A {
        match self.bits {
            false => DBG_RTC_STOP_A::B_0X0,
            true => DBG_RTC_STOP_A::B_0X1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_RTC_STOP_A::B_0X0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_RTC_STOP_A::B_0X1
    }
}
#[doc = "Field `DBG_RTC_STOP` writer - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:"]
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_RTC_STOP_A>;
impl<'a, REG> DBG_RTC_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_RTC_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_RTC_STOP_A::B_0X1)
    }
}
#[doc = "Field `DBG_WWDG_STOP` reader - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:"]
pub type DBG_WWDG_STOP_R = crate::BitReader<DBG_WWDG_STOP_A>;
#[doc = "Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_WWDG_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_WWDG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_WWDG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_WWDG_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_WWDG_STOP_A {
        match self.bits {
            false => DBG_WWDG_STOP_A::B_0X0,
            true => DBG_WWDG_STOP_A::B_0X1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_WWDG_STOP_A::B_0X0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_WWDG_STOP_A::B_0X1
    }
}
#[doc = "Field `DBG_WWDG_STOP` writer - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:"]
pub type DBG_WWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_WWDG_STOP_A>;
impl<'a, REG> DBG_WWDG_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_WWDG_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_WWDG_STOP_A::B_0X1)
    }
}
#[doc = "Field `DBG_IWDG_STOP` reader - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:"]
pub type DBG_IWDG_STOP_R = crate::BitReader<DBG_IWDG_STOP_A>;
#[doc = "Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_IWDG_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_IWDG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_IWDG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_IWDG_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_IWDG_STOP_A {
        match self.bits {
            false => DBG_IWDG_STOP_A::B_0X0,
            true => DBG_IWDG_STOP_A::B_0X1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_IWDG_STOP_A::B_0X0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_IWDG_STOP_A::B_0X1
    }
}
#[doc = "Field `DBG_IWDG_STOP` writer - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:"]
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_IWDG_STOP_A>;
impl<'a, REG> DBG_IWDG_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_IWDG_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_IWDG_STOP_A::B_0X1)
    }
}
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` reader - SMBUS timeout when core is halted"]
pub type DBG_I2C1_SMBUS_TIMEOUT_R = crate::BitReader<DBG_I2C1_SMBUS_TIMEOUT_A>;
#[doc = "SMBUS timeout when core is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_I2C1_SMBUS_TIMEOUT_A {
    #[doc = "0: Same behavior as in normal mode"]
    B_0X0 = 0,
    #[doc = "1: The SMBUS timeout is frozen"]
    B_0X1 = 1,
}
impl From<DBG_I2C1_SMBUS_TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_I2C1_SMBUS_TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_I2C1_SMBUS_TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_I2C1_SMBUS_TIMEOUT_A {
        match self.bits {
            false => DBG_I2C1_SMBUS_TIMEOUT_A::B_0X0,
            true => DBG_I2C1_SMBUS_TIMEOUT_A::B_0X1,
        }
    }
    #[doc = "Same behavior as in normal mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_I2C1_SMBUS_TIMEOUT_A::B_0X0
    }
    #[doc = "The SMBUS timeout is frozen"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_I2C1_SMBUS_TIMEOUT_A::B_0X1
    }
}
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` writer - SMBUS timeout when core is halted"]
pub type DBG_I2C1_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG, DBG_I2C1_SMBUS_TIMEOUT_A>;
impl<'a, REG> DBG_I2C1_SMBUS_TIMEOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Same behavior as in normal mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I2C1_SMBUS_TIMEOUT_A::B_0X0)
    }
    #[doc = "The SMBUS timeout is frozen"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I2C1_SMBUS_TIMEOUT_A::B_0X1)
    }
}
#[doc = "Field `DBG_LPTIM2_STOP` reader - Clocking of LPTIMER2 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER2 when the core is halted:"]
pub type DBG_LPTIM2_STOP_R = crate::BitReader<DBG_LPTIM2_STOP_A>;
#[doc = "Clocking of LPTIMER2 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER2 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_LPTIM2_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_LPTIM2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_LPTIM2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_LPTIM2_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_LPTIM2_STOP_A {
        match self.bits {
            false => DBG_LPTIM2_STOP_A::B_0X0,
            true => DBG_LPTIM2_STOP_A::B_0X1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_LPTIM2_STOP_A::B_0X0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_LPTIM2_STOP_A::B_0X1
    }
}
#[doc = "Field `DBG_LPTIM2_STOP` writer - Clocking of LPTIMER2 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER2 when the core is halted:"]
pub type DBG_LPTIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_LPTIM2_STOP_A>;
impl<'a, REG> DBG_LPTIM2_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_LPTIM2_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_LPTIM2_STOP_A::B_0X1)
    }
}
#[doc = "Field `DBG_LPTIM1_STOP` reader - Clocking of LPTIMER1 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER1 when the core is halted:"]
pub type DBG_LPTIM1_STOP_R = crate::BitReader<DBG_LPTIM1_STOP_A>;
#[doc = "Clocking of LPTIMER1 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER1 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_LPTIM1_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_LPTIM1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_LPTIM1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_LPTIM1_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_LPTIM1_STOP_A {
        match self.bits {
            false => DBG_LPTIM1_STOP_A::B_0X0,
            true => DBG_LPTIM1_STOP_A::B_0X1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_LPTIM1_STOP_A::B_0X0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_LPTIM1_STOP_A::B_0X1
    }
}
#[doc = "Field `DBG_LPTIM1_STOP` writer - Clocking of LPTIMER1 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER1 when the core is halted:"]
pub type DBG_LPTIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_LPTIM1_STOP_A>;
impl<'a, REG> DBG_LPTIM1_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_LPTIM1_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_LPTIM1_STOP_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBUS timeout when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&self) -> DBG_I2C1_SMBUS_TIMEOUT_R {
        DBG_I2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30 - Clocking of LPTIMER2 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER2 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Clocking of LPTIMER1 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER1 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<DBG_APB_FZ1_SPEC> {
        DBG_TIM2_STOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<DBG_APB_FZ1_SPEC> {
        DBG_TIM3_STOP_W::new(self, 1)
    }
    #[doc = "Bit 4 - Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim6_stop(&mut self) -> DBG_TIM6_STOP_W<DBG_APB_FZ1_SPEC> {
        DBG_TIM6_STOP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W<DBG_APB_FZ1_SPEC> {
        DBG_TIM7_STOP_W::new(self, 5)
    }
    #[doc = "Bit 10 - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<DBG_APB_FZ1_SPEC> {
        DBG_RTC_STOP_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<DBG_APB_FZ1_SPEC> {
        DBG_WWDG_STOP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<DBG_APB_FZ1_SPEC> {
        DBG_IWDG_STOP_W::new(self, 12)
    }
    #[doc = "Bit 21 - SMBUS timeout when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c1_smbus_timeout(&mut self) -> DBG_I2C1_SMBUS_TIMEOUT_W<DBG_APB_FZ1_SPEC> {
        DBG_I2C1_SMBUS_TIMEOUT_W::new(self, 21)
    }
    #[doc = "Bit 30 - Clocking of LPTIMER2 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER2 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim2_stop(&mut self) -> DBG_LPTIM2_STOP_W<DBG_APB_FZ1_SPEC> {
        DBG_LPTIM2_STOP_W::new(self, 30)
    }
    #[doc = "Bit 31 - Clocking of LPTIMER1 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER1 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W<DBG_APB_FZ1_SPEC> {
        DBG_LPTIM1_STOP_W::new(self, 31)
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
#[doc = "DBG APB freeze register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_apb_fz1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_apb_fz1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_APB_FZ1_SPEC;
impl crate::RegisterSpec for DBG_APB_FZ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_apb_fz1::R`](R) reader structure"]
impl crate::Readable for DBG_APB_FZ1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbg_apb_fz1::W`](W) writer structure"]
impl crate::Writable for DBG_APB_FZ1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_APB_FZ1 to value 0"]
impl crate::Resettable for DBG_APB_FZ1_SPEC {
    const RESET_VALUE: u32 = 0;
}
