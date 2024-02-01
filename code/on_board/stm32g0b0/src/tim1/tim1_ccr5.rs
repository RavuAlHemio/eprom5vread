#[doc = "Register `TIM1_CCR5` reader"]
pub type R = crate::R<TIM1_CCR5_SPEC>;
#[doc = "Register `TIM1_CCR5` writer"]
pub type W = crate::W<TIM1_CCR5_SPEC>;
#[doc = "Field `CCR5` reader - Capture/Compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC5 output."]
pub type CCR5_R = crate::FieldReader<u16>;
#[doc = "Field `CCR5` writer - Capture/Compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC5 output."]
pub type CCR5_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `GC5C1` reader - Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C1_R = crate::BitReader<GC5C1_A>;
#[doc = "Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC5C1_A {
    #[doc = "0: No effect of OC5REF on OC1REFC5"]
    B_0X0 = 0,
    #[doc = "1: OC1REFC is the logical AND of OC1REFC and OC5REF"]
    B_0X1 = 1,
}
impl From<GC5C1_A> for bool {
    #[inline(always)]
    fn from(variant: GC5C1_A) -> Self {
        variant as u8 != 0
    }
}
impl GC5C1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GC5C1_A {
        match self.bits {
            false => GC5C1_A::B_0X0,
            true => GC5C1_A::B_0X1,
        }
    }
    #[doc = "No effect of OC5REF on OC1REFC5"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GC5C1_A::B_0X0
    }
    #[doc = "OC1REFC is the logical AND of OC1REFC and OC5REF"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GC5C1_A::B_0X1
    }
}
#[doc = "Field `GC5C1` writer - Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C1_W<'a, REG> = crate::BitWriter<'a, REG, GC5C1_A>;
impl<'a, REG> GC5C1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect of OC5REF on OC1REFC5"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C1_A::B_0X0)
    }
    #[doc = "OC1REFC is the logical AND of OC1REFC and OC5REF"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C1_A::B_0X1)
    }
}
#[doc = "Field `GC5C2` reader - Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C2_R = crate::BitReader<GC5C2_A>;
#[doc = "Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC5C2_A {
    #[doc = "0: No effect of OC5REF on OC2REFC"]
    B_0X0 = 0,
    #[doc = "1: OC2REFC is the logical AND of OC2REFC and OC5REF"]
    B_0X1 = 1,
}
impl From<GC5C2_A> for bool {
    #[inline(always)]
    fn from(variant: GC5C2_A) -> Self {
        variant as u8 != 0
    }
}
impl GC5C2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GC5C2_A {
        match self.bits {
            false => GC5C2_A::B_0X0,
            true => GC5C2_A::B_0X1,
        }
    }
    #[doc = "No effect of OC5REF on OC2REFC"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GC5C2_A::B_0X0
    }
    #[doc = "OC2REFC is the logical AND of OC2REFC and OC5REF"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GC5C2_A::B_0X1
    }
}
#[doc = "Field `GC5C2` writer - Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C2_W<'a, REG> = crate::BitWriter<'a, REG, GC5C2_A>;
impl<'a, REG> GC5C2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect of OC5REF on OC2REFC"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C2_A::B_0X0)
    }
    #[doc = "OC2REFC is the logical AND of OC2REFC and OC5REF"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C2_A::B_0X1)
    }
}
#[doc = "Field `GC5C3` reader - Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C3_R = crate::BitReader<GC5C3_A>;
#[doc = "Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC5C3_A {
    #[doc = "0: No effect of OC5REF on OC3REFC"]
    B_0X0 = 0,
    #[doc = "1: OC3REFC is the logical AND of OC3REFC and OC5REF"]
    B_0X1 = 1,
}
impl From<GC5C3_A> for bool {
    #[inline(always)]
    fn from(variant: GC5C3_A) -> Self {
        variant as u8 != 0
    }
}
impl GC5C3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GC5C3_A {
        match self.bits {
            false => GC5C3_A::B_0X0,
            true => GC5C3_A::B_0X1,
        }
    }
    #[doc = "No effect of OC5REF on OC3REFC"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GC5C3_A::B_0X0
    }
    #[doc = "OC3REFC is the logical AND of OC3REFC and OC5REF"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GC5C3_A::B_0X1
    }
}
#[doc = "Field `GC5C3` writer - Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C3_W<'a, REG> = crate::BitWriter<'a, REG, GC5C3_A>;
impl<'a, REG> GC5C3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect of OC5REF on OC3REFC"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C3_A::B_0X0)
    }
    #[doc = "OC3REFC is the logical AND of OC3REFC and OC5REF"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C3_A::B_0X1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC5 output."]
    #[inline(always)]
    pub fn ccr5(&self) -> CCR5_R {
        CCR5_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 29 - Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn gc5c1(&self) -> GC5C1_R {
        GC5C1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn gc5c2(&self) -> GC5C2_R {
        GC5C2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn gc5c3(&self) -> GC5C3_R {
        GC5C3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC5 output."]
    #[inline(always)]
    #[must_use]
    pub fn ccr5(&mut self) -> CCR5_W<TIM1_CCR5_SPEC> {
        CCR5_W::new(self, 0)
    }
    #[doc = "Bit 29 - Group Channel 5 and Channel 1 Distortion on Channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    #[must_use]
    pub fn gc5c1(&mut self) -> GC5C1_W<TIM1_CCR5_SPEC> {
        GC5C1_W::new(self, 29)
    }
    #[doc = "Bit 30 - Group Channel 5 and Channel 2 Distortion on Channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    #[must_use]
    pub fn gc5c2(&mut self) -> GC5C2_W<TIM1_CCR5_SPEC> {
        GC5C2_W::new(self, 30)
    }
    #[doc = "Bit 31 - Group Channel 5 and Channel 3 Distortion on Channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    #[must_use]
    pub fn gc5c3(&mut self) -> GC5C3_W<TIM1_CCR5_SPEC> {
        GC5C3_W::new(self, 31)
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
#[doc = "capture/compare register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_CCR5_SPEC;
impl crate::RegisterSpec for TIM1_CCR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_ccr5::R`](R) reader structure"]
impl crate::Readable for TIM1_CCR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tim1_ccr5::W`](W) writer structure"]
impl crate::Writable for TIM1_CCR5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_CCR5 to value 0"]
impl crate::Resettable for TIM1_CCR5_SPEC {
    const RESET_VALUE: u32 = 0;
}
