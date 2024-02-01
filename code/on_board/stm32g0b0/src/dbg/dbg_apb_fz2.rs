#[doc = "Register `DBG_APB_FZ2` reader"]
pub type R = crate::R<DBG_APB_FZ2_SPEC>;
#[doc = "Register `DBG_APB_FZ2` writer"]
pub type W = crate::W<DBG_APB_FZ2_SPEC>;
#[doc = "Field `DBG_TIM1_STOP` reader - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:"]
pub type DBG_TIM1_STOP_R = crate::BitReader<DBG_TIM1_STOP_A>;
#[doc = "Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM1_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_TIM1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_TIM1_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM1_STOP_A {
        match self.bits {
            false => DBG_TIM1_STOP_A::B_0X0,
            true => DBG_TIM1_STOP_A::B_0X1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM1_STOP_A::B_0X0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM1_STOP_A::B_0X1
    }
}
#[doc = "Field `DBG_TIM1_STOP` writer - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:"]
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM1_STOP_A>;
impl<'a, REG> DBG_TIM1_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM1_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM1_STOP_A::B_0X1)
    }
}
#[doc = "Field `DBG_TIM14_STOP` reader - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:"]
pub type DBG_TIM14_STOP_R = crate::BitReader<DBG_TIM14_STOP_A>;
#[doc = "Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM14_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_TIM14_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM14_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_TIM14_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM14_STOP_A {
        match self.bits {
            false => DBG_TIM14_STOP_A::B_0X0,
            true => DBG_TIM14_STOP_A::B_0X1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM14_STOP_A::B_0X0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM14_STOP_A::B_0X1
    }
}
#[doc = "Field `DBG_TIM14_STOP` writer - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:"]
pub type DBG_TIM14_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM14_STOP_A>;
impl<'a, REG> DBG_TIM14_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM14_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM14_STOP_A::B_0X1)
    }
}
#[doc = "Field `DBG_TIM15_STOP` reader - Clocking of TIM15 counter when the core is halted This bit enables/disables the clock to the counter of TIM15 when the core is halted: Only available on STM32G071xx and STM32G081xx, reserved on STM32G031xx and STM32G041xx."]
pub type DBG_TIM15_STOP_R = crate::BitReader<DBG_TIM15_STOP_A>;
#[doc = "Clocking of TIM15 counter when the core is halted This bit enables/disables the clock to the counter of TIM15 when the core is halted: Only available on STM32G071xx and STM32G081xx, reserved on STM32G031xx and STM32G041xx.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM15_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_TIM15_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM15_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_TIM15_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM15_STOP_A {
        match self.bits {
            false => DBG_TIM15_STOP_A::B_0X0,
            true => DBG_TIM15_STOP_A::B_0X1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM15_STOP_A::B_0X0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM15_STOP_A::B_0X1
    }
}
#[doc = "Field `DBG_TIM15_STOP` writer - Clocking of TIM15 counter when the core is halted This bit enables/disables the clock to the counter of TIM15 when the core is halted: Only available on STM32G071xx and STM32G081xx, reserved on STM32G031xx and STM32G041xx."]
pub type DBG_TIM15_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM15_STOP_A>;
impl<'a, REG> DBG_TIM15_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM15_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM15_STOP_A::B_0X1)
    }
}
#[doc = "Field `DBG_TIM16_STOP` reader - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:"]
pub type DBG_TIM16_STOP_R = crate::BitReader<DBG_TIM16_STOP_A>;
#[doc = "Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM16_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_TIM16_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM16_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_TIM16_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM16_STOP_A {
        match self.bits {
            false => DBG_TIM16_STOP_A::B_0X0,
            true => DBG_TIM16_STOP_A::B_0X1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM16_STOP_A::B_0X0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM16_STOP_A::B_0X1
    }
}
#[doc = "Field `DBG_TIM16_STOP` writer - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:"]
pub type DBG_TIM16_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM16_STOP_A>;
impl<'a, REG> DBG_TIM16_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM16_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM16_STOP_A::B_0X1)
    }
}
#[doc = "Field `DBG_TIM17_STOP` reader - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:"]
pub type DBG_TIM17_STOP_R = crate::BitReader<DBG_TIM17_STOP_A>;
#[doc = "Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM17_STOP_A {
    #[doc = "0: Enable"]
    B_0X0 = 0,
    #[doc = "1: Disable"]
    B_0X1 = 1,
}
impl From<DBG_TIM17_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM17_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_TIM17_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM17_STOP_A {
        match self.bits {
            false => DBG_TIM17_STOP_A::B_0X0,
            true => DBG_TIM17_STOP_A::B_0X1,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM17_STOP_A::B_0X0
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM17_STOP_A::B_0X1
    }
}
#[doc = "Field `DBG_TIM17_STOP` writer - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:"]
pub type DBG_TIM17_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM17_STOP_A>;
impl<'a, REG> DBG_TIM17_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM17_STOP_A::B_0X0)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM17_STOP_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 11 - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Clocking of TIM15 counter when the core is halted This bit enables/disables the clock to the counter of TIM15 when the core is halted: Only available on STM32G071xx and STM32G081xx, reserved on STM32G031xx and STM32G041xx."]
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DBG_TIM15_STOP_R {
        DBG_TIM15_STOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<DBG_APB_FZ2_SPEC> {
        DBG_TIM1_STOP_W::new(self, 11)
    }
    #[doc = "Bit 15 - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W<DBG_APB_FZ2_SPEC> {
        DBG_TIM14_STOP_W::new(self, 15)
    }
    #[doc = "Bit 16 - Clocking of TIM15 counter when the core is halted This bit enables/disables the clock to the counter of TIM15 when the core is halted: Only available on STM32G071xx and STM32G081xx, reserved on STM32G031xx and STM32G041xx."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim15_stop(&mut self) -> DBG_TIM15_STOP_W<DBG_APB_FZ2_SPEC> {
        DBG_TIM15_STOP_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<DBG_APB_FZ2_SPEC> {
        DBG_TIM16_STOP_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<DBG_APB_FZ2_SPEC> {
        DBG_TIM17_STOP_W::new(self, 18)
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
#[doc = "DBG APB freeze register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_apb_fz2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_apb_fz2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_APB_FZ2_SPEC;
impl crate::RegisterSpec for DBG_APB_FZ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_apb_fz2::R`](R) reader structure"]
impl crate::Readable for DBG_APB_FZ2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbg_apb_fz2::W`](W) writer structure"]
impl crate::Writable for DBG_APB_FZ2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_APB_FZ2 to value 0"]
impl crate::Resettable for DBG_APB_FZ2_SPEC {
    const RESET_VALUE: u32 = 0;
}
