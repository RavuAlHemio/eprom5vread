#[doc = "Register `WWDG_CFR` reader"]
pub type R = crate::R<WWDG_CFR_SPEC>;
#[doc = "Register `WWDG_CFR` writer"]
pub type W = crate::W<WWDG_CFR_SPEC>;
#[doc = "Field `W` reader - 7-bit window value These bits contain the window value to be compared with the down-counter."]
pub type W_R = crate::FieldReader;
#[doc = "Field `W` writer - 7-bit window value These bits contain the window value to be compared with the down-counter."]
pub type W_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EWI` reader - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset."]
pub type EWI_R = crate::BitReader;
#[doc = "Field `EWI` writer - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset."]
pub type EWI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGTB` reader - Timer base The timebase of the prescaler can be modified as follows:"]
pub type WDGTB_R = crate::FieldReader<WDGTB_A>;
#[doc = "Timer base The timebase of the prescaler can be modified as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDGTB_A {
    #[doc = "0: CK Counter Clock (PCLK div 4096) div 1"]
    B_0X0 = 0,
    #[doc = "1: CK Counter Clock (PCLK div 4096) div 2"]
    B_0X1 = 1,
    #[doc = "2: CK Counter Clock (PCLK div 4096) div 4"]
    B_0X2 = 2,
    #[doc = "3: CK Counter Clock (PCLK div 4096) div 8"]
    B_0X3 = 3,
    #[doc = "4: CK Counter Clock (PCLK div 4096) div 16"]
    B_0X4 = 4,
    #[doc = "5: CK Counter Clock (PCLK div 4096) div 32"]
    B_0X5 = 5,
    #[doc = "6: CK Counter Clock (PCLK div 4096) div 64"]
    B_0X6 = 6,
    #[doc = "7: CK Counter Clock (PCLK div 4096) div 128"]
    B_0X7 = 7,
}
impl From<WDGTB_A> for u8 {
    #[inline(always)]
    fn from(variant: WDGTB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDGTB_A {
    type Ux = u8;
}
impl WDGTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDGTB_A {
        match self.bits {
            0 => WDGTB_A::B_0X0,
            1 => WDGTB_A::B_0X1,
            2 => WDGTB_A::B_0X2,
            3 => WDGTB_A::B_0X3,
            4 => WDGTB_A::B_0X4,
            5 => WDGTB_A::B_0X5,
            6 => WDGTB_A::B_0X6,
            7 => WDGTB_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 1"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WDGTB_A::B_0X0
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WDGTB_A::B_0X1
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == WDGTB_A::B_0X2
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 8"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == WDGTB_A::B_0X3
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 16"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == WDGTB_A::B_0X4
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 32"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == WDGTB_A::B_0X5
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 64"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == WDGTB_A::B_0X6
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 128"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == WDGTB_A::B_0X7
    }
}
#[doc = "Field `WDGTB` writer - Timer base The timebase of the prescaler can be modified as follows:"]
pub type WDGTB_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, WDGTB_A>;
impl<'a, REG> WDGTB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CK Counter Clock (PCLK div 4096) div 1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB_A::B_0X0)
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB_A::B_0X1)
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB_A::B_0X2)
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB_A::B_0X3)
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 16"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB_A::B_0X4)
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 32"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB_A::B_0X5)
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 64"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB_A::B_0X6)
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 128"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB_A::B_0X7)
    }
}
impl R {
    #[doc = "Bits 0:6 - 7-bit window value These bits contain the window value to be compared with the down-counter."]
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 9 - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset."]
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Timer base The timebase of the prescaler can be modified as follows:"]
    #[inline(always)]
    pub fn wdgtb(&self) -> WDGTB_R {
        WDGTB_R::new(((self.bits >> 11) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit window value These bits contain the window value to be compared with the down-counter."]
    #[inline(always)]
    #[must_use]
    pub fn w(&mut self) -> W_W<WWDG_CFR_SPEC> {
        W_W::new(self, 0)
    }
    #[doc = "Bit 9 - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset."]
    #[inline(always)]
    #[must_use]
    pub fn ewi(&mut self) -> EWI_W<WWDG_CFR_SPEC> {
        EWI_W::new(self, 9)
    }
    #[doc = "Bits 11:13 - Timer base The timebase of the prescaler can be modified as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn wdgtb(&mut self) -> WDGTB_W<WWDG_CFR_SPEC> {
        WDGTB_W::new(self, 11)
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
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_cfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wwdg_cfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WWDG_CFR_SPEC;
impl crate::RegisterSpec for WWDG_CFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdg_cfr::R`](R) reader structure"]
impl crate::Readable for WWDG_CFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wwdg_cfr::W`](W) writer structure"]
impl crate::Writable for WWDG_CFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WWDG_CFR to value 0x7f"]
impl crate::Resettable for WWDG_CFR_SPEC {
    const RESET_VALUE: u32 = 0x7f;
}
