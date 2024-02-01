#[doc = "Register `DBG_CR` reader"]
pub type R = crate::R<DBG_CR_SPEC>;
#[doc = "Register `DBG_CR` writer"]
pub type W = crate::W<DBG_CR_SPEC>;
#[doc = "Field `DBG_STOP` reader - Debug Stop mode Debug options in Stop mode. Upon Stop mode exit, the software must re-establish the desired clock configuration."]
pub type DBG_STOP_R = crate::BitReader<DBG_STOP_A>;
#[doc = "Debug Stop mode Debug options in Stop mode. Upon Stop mode exit, the software must re-establish the desired clock configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_STOP_A {
    #[doc = "0: All clocks disabled, including FCLK and HCLK. Upon Stop mode exit, the CPU is clocked by the HSI internal RC oscillator."]
    B_0X0 = 0,
    #[doc = "1: FCLK and HCLK running, derived from the internal RC oscillator remaining active. If Systick is enabled, it may generate periodic interrupt and wake up events."]
    B_0X1 = 1,
}
impl From<DBG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_STOP_A {
        match self.bits {
            false => DBG_STOP_A::B_0X0,
            true => DBG_STOP_A::B_0X1,
        }
    }
    #[doc = "All clocks disabled, including FCLK and HCLK. Upon Stop mode exit, the CPU is clocked by the HSI internal RC oscillator."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_STOP_A::B_0X0
    }
    #[doc = "FCLK and HCLK running, derived from the internal RC oscillator remaining active. If Systick is enabled, it may generate periodic interrupt and wake up events."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_STOP_A::B_0X1
    }
}
#[doc = "Field `DBG_STOP` writer - Debug Stop mode Debug options in Stop mode. Upon Stop mode exit, the software must re-establish the desired clock configuration."]
pub type DBG_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_STOP_A>;
impl<'a, REG> DBG_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All clocks disabled, including FCLK and HCLK. Upon Stop mode exit, the CPU is clocked by the HSI internal RC oscillator."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_STOP_A::B_0X0)
    }
    #[doc = "FCLK and HCLK running, derived from the internal RC oscillator remaining active. If Systick is enabled, it may generate periodic interrupt and wake up events."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_STOP_A::B_0X1)
    }
}
#[doc = "Field `DBG_STANDBY` reader - Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode."]
pub type DBG_STANDBY_R = crate::BitReader<DBG_STANDBY_A>;
#[doc = "Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_STANDBY_A {
    #[doc = "0: Digital part powered. From software point of view, exiting Standby and Shutdown modes is identical as fetching reset vector (except for status bits indicating that the MCU exits Standby)"]
    B_0X0 = 0,
    #[doc = "1: Digital part powered and FCLK and HCLK running, derived from the internal RC oscillator remaining active. The MCU generates a system reset so that exiting Standby and Shutdown has the same effect as starting from reset."]
    B_0X1 = 1,
}
impl From<DBG_STANDBY_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_STANDBY_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_STANDBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_STANDBY_A {
        match self.bits {
            false => DBG_STANDBY_A::B_0X0,
            true => DBG_STANDBY_A::B_0X1,
        }
    }
    #[doc = "Digital part powered. From software point of view, exiting Standby and Shutdown modes is identical as fetching reset vector (except for status bits indicating that the MCU exits Standby)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_STANDBY_A::B_0X0
    }
    #[doc = "Digital part powered and FCLK and HCLK running, derived from the internal RC oscillator remaining active. The MCU generates a system reset so that exiting Standby and Shutdown has the same effect as starting from reset."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_STANDBY_A::B_0X1
    }
}
#[doc = "Field `DBG_STANDBY` writer - Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode."]
pub type DBG_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG, DBG_STANDBY_A>;
impl<'a, REG> DBG_STANDBY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Digital part powered. From software point of view, exiting Standby and Shutdown modes is identical as fetching reset vector (except for status bits indicating that the MCU exits Standby)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_STANDBY_A::B_0X0)
    }
    #[doc = "Digital part powered and FCLK and HCLK running, derived from the internal RC oscillator remaining active. The MCU generates a system reset so that exiting Standby and Shutdown has the same effect as starting from reset."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_STANDBY_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 1 - Debug Stop mode Debug options in Stop mode. Upon Stop mode exit, the software must re-establish the desired clock configuration."]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode."]
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Debug Stop mode Debug options in Stop mode. Upon Stop mode exit, the software must re-establish the desired clock configuration."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<DBG_CR_SPEC> {
        DBG_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Debug Standby and Shutdown modes Debug options in Standby or Shutdown mode."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<DBG_CR_SPEC> {
        DBG_STANDBY_W::new(self, 2)
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
#[doc = "DBG configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_CR_SPEC;
impl crate::RegisterSpec for DBG_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_cr::R`](R) reader structure"]
impl crate::Readable for DBG_CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbg_cr::W`](W) writer structure"]
impl crate::Writable for DBG_CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_CR to value 0"]
impl crate::Resettable for DBG_CR_SPEC {
    const RESET_VALUE: u32 = 0;
}
