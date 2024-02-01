#[doc = "Register `WWDG_CR` reader"]
pub type R = crate::R<WWDG_CR_SPEC>;
#[doc = "Register `WWDG_CR` writer"]
pub type W = crate::W<WWDG_CR_SPEC>;
#[doc = "Field `T` reader - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\\[1:0\\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared)."]
pub type T_R = crate::FieldReader;
#[doc = "Field `T` writer - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\\[1:0\\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared)."]
pub type T_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WDGA` reader - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA=1, the watchdog can generate a reset."]
pub type WDGA_R = crate::BitReader<WDGA_A>;
#[doc = "Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA=1, the watchdog can generate a reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDGA_A {
    #[doc = "0: Watchdog disabled"]
    B_0X0 = 0,
    #[doc = "1: Watchdog enabled"]
    B_0X1 = 1,
}
impl From<WDGA_A> for bool {
    #[inline(always)]
    fn from(variant: WDGA_A) -> Self {
        variant as u8 != 0
    }
}
impl WDGA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDGA_A {
        match self.bits {
            false => WDGA_A::B_0X0,
            true => WDGA_A::B_0X1,
        }
    }
    #[doc = "Watchdog disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WDGA_A::B_0X0
    }
    #[doc = "Watchdog enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WDGA_A::B_0X1
    }
}
#[doc = "Field `WDGA` writer - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA=1, the watchdog can generate a reset."]
pub type WDGA_W<'a, REG> = crate::BitWriter<'a, REG, WDGA_A>;
impl<'a, REG> WDGA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Watchdog disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WDGA_A::B_0X0)
    }
    #[doc = "Watchdog enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WDGA_A::B_0X1)
    }
}
impl R {
    #[doc = "Bits 0:6 - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\\[1:0\\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared)."]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA=1, the watchdog can generate a reset."]
    #[inline(always)]
    pub fn wdga(&self) -> WDGA_R {
        WDGA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\\[1:0\\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared)."]
    #[inline(always)]
    #[must_use]
    pub fn t(&mut self) -> T_W<WWDG_CR_SPEC> {
        T_W::new(self, 0)
    }
    #[doc = "Bit 7 - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGA=1, the watchdog can generate a reset."]
    #[inline(always)]
    #[must_use]
    pub fn wdga(&mut self) -> WDGA_W<WWDG_CR_SPEC> {
        WDGA_W::new(self, 7)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wwdg_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WWDG_CR_SPEC;
impl crate::RegisterSpec for WWDG_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdg_cr::R`](R) reader structure"]
impl crate::Readable for WWDG_CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wwdg_cr::W`](W) writer structure"]
impl crate::Writable for WWDG_CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WWDG_CR to value 0x7f"]
impl crate::Resettable for WWDG_CR_SPEC {
    const RESET_VALUE: u32 = 0x7f;
}
