#[doc = "Register `DMAMUX_RG2CR` reader"]
pub type R = crate::R<DMAMUX_RG2CR_SPEC>;
#[doc = "Register `DMAMUX_RG2CR` writer"]
pub type W = crate::W<DMAMUX_RG2CR_SPEC>;
#[doc = "Field `SIG_ID` reader - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator"]
pub type SIG_ID_R = crate::FieldReader;
#[doc = "Field `SIG_ID` writer - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator"]
pub type SIG_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OIE` reader - Trigger overrun interrupt enable"]
pub type OIE_R = crate::BitReader<OIE_A>;
#[doc = "Trigger overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIE_A {
    #[doc = "0: interrupt on a trigger overrun event occurrence is disabled"]
    B_0X0 = 0,
    #[doc = "1: interrupt on a trigger overrun event occurrence is enabled"]
    B_0X1 = 1,
}
impl From<OIE_A> for bool {
    #[inline(always)]
    fn from(variant: OIE_A) -> Self {
        variant as u8 != 0
    }
}
impl OIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OIE_A {
        match self.bits {
            false => OIE_A::B_0X0,
            true => OIE_A::B_0X1,
        }
    }
    #[doc = "interrupt on a trigger overrun event occurrence is disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OIE_A::B_0X0
    }
    #[doc = "interrupt on a trigger overrun event occurrence is enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OIE_A::B_0X1
    }
}
#[doc = "Field `OIE` writer - Trigger overrun interrupt enable"]
pub type OIE_W<'a, REG> = crate::BitWriter<'a, REG, OIE_A>;
impl<'a, REG> OIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt on a trigger overrun event occurrence is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OIE_A::B_0X0)
    }
    #[doc = "interrupt on a trigger overrun event occurrence is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OIE_A::B_0X1)
    }
}
#[doc = "Field `GE` reader - DMA request generator channel x enable"]
pub type GE_R = crate::BitReader<GE_A>;
#[doc = "DMA request generator channel x enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GE_A {
    #[doc = "0: DMA request generator channel x disabled"]
    B_0X0 = 0,
    #[doc = "1: DMA request generator channel x enabled"]
    B_0X1 = 1,
}
impl From<GE_A> for bool {
    #[inline(always)]
    fn from(variant: GE_A) -> Self {
        variant as u8 != 0
    }
}
impl GE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GE_A {
        match self.bits {
            false => GE_A::B_0X0,
            true => GE_A::B_0X1,
        }
    }
    #[doc = "DMA request generator channel x disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GE_A::B_0X0
    }
    #[doc = "DMA request generator channel x enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GE_A::B_0X1
    }
}
#[doc = "Field `GE` writer - DMA request generator channel x enable"]
pub type GE_W<'a, REG> = crate::BitWriter<'a, REG, GE_A>;
impl<'a, REG> GE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA request generator channel x disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GE_A::B_0X0)
    }
    #[doc = "DMA request generator channel x enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GE_A::B_0X1)
    }
}
#[doc = "Field `GPOL` reader - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input"]
pub type GPOL_R = crate::FieldReader<GPOL_A>;
#[doc = "DMA request generator trigger polarity Defines the edge polarity of the selected trigger input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPOL_A {
    #[doc = "0: no event. I.e. none trigger detection nor generation."]
    B_0X0 = 0,
    #[doc = "1: rising edge"]
    B_0X1 = 1,
    #[doc = "2: falling edge"]
    B_0X2 = 2,
    #[doc = "3: rising and falling edge"]
    B_0X3 = 3,
}
impl From<GPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: GPOL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GPOL_A {
    type Ux = u8;
}
impl GPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPOL_A {
        match self.bits {
            0 => GPOL_A::B_0X0,
            1 => GPOL_A::B_0X1,
            2 => GPOL_A::B_0X2,
            3 => GPOL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "no event. I.e. none trigger detection nor generation."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPOL_A::B_0X0
    }
    #[doc = "rising edge"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPOL_A::B_0X1
    }
    #[doc = "falling edge"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == GPOL_A::B_0X2
    }
    #[doc = "rising and falling edge"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == GPOL_A::B_0X3
    }
}
#[doc = "Field `GPOL` writer - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input"]
pub type GPOL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, GPOL_A>;
impl<'a, REG> GPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no event. I.e. none trigger detection nor generation."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL_A::B_0X0)
    }
    #[doc = "rising edge"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL_A::B_0X1)
    }
    #[doc = "falling edge"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL_A::B_0X2)
    }
    #[doc = "rising and falling edge"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(GPOL_A::B_0X3)
    }
}
#[doc = "Field `GNBREQ` reader - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field shall only be written when GE bit is disabled."]
pub type GNBREQ_R = crate::FieldReader;
#[doc = "Field `GNBREQ` writer - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field shall only be written when GE bit is disabled."]
pub type GNBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator"]
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA request generator channel x enable"]
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input"]
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field shall only be written when GE bit is disabled."]
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator"]
    #[inline(always)]
    #[must_use]
    pub fn sig_id(&mut self) -> SIG_ID_W<DMAMUX_RG2CR_SPEC> {
        SIG_ID_W::new(self, 0)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OIE_W<DMAMUX_RG2CR_SPEC> {
        OIE_W::new(self, 8)
    }
    #[doc = "Bit 16 - DMA request generator channel x enable"]
    #[inline(always)]
    #[must_use]
    pub fn ge(&mut self) -> GE_W<DMAMUX_RG2CR_SPEC> {
        GE_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input"]
    #[inline(always)]
    #[must_use]
    pub fn gpol(&mut self) -> GPOL_W<DMAMUX_RG2CR_SPEC> {
        GPOL_W::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field shall only be written when GE bit is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn gnbreq(&mut self) -> GNBREQ_W<DMAMUX_RG2CR_SPEC> {
        GNBREQ_W::new(self, 19)
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
#[doc = "DMAMUX request generator channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg2cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg2cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMUX_RG2CR_SPEC;
impl crate::RegisterSpec for DMAMUX_RG2CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamux_rg2cr::R`](R) reader structure"]
impl crate::Readable for DMAMUX_RG2CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmamux_rg2cr::W`](W) writer structure"]
impl crate::Writable for DMAMUX_RG2CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAMUX_RG2CR to value 0"]
impl crate::Resettable for DMAMUX_RG2CR_SPEC {
    const RESET_VALUE: u32 = 0;
}
