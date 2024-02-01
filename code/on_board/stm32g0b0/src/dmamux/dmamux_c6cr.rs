#[doc = "Register `DMAMUX_C6CR` reader"]
pub type R = crate::R<DMAMUX_C6CR_SPEC>;
#[doc = "Register `DMAMUX_C6CR` writer"]
pub type W = crate::W<DMAMUX_C6CR_SPEC>;
#[doc = "Field `DMAREQ_ID` reader - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources."]
pub type DMAREQ_ID_R = crate::FieldReader;
#[doc = "Field `DMAREQ_ID` writer - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources."]
pub type DMAREQ_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SOIE` reader - Synchronization overrun interrupt enable"]
pub type SOIE_R = crate::BitReader<SOIE_A>;
#[doc = "Synchronization overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOIE_A {
    #[doc = "0: interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0X1 = 1,
}
impl From<SOIE_A> for bool {
    #[inline(always)]
    fn from(variant: SOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOIE_A {
        match self.bits {
            false => SOIE_A::B_0X0,
            true => SOIE_A::B_0X1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SOIE_A::B_0X0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SOIE_A::B_0X1
    }
}
#[doc = "Field `SOIE` writer - Synchronization overrun interrupt enable"]
pub type SOIE_W<'a, REG> = crate::BitWriter<'a, REG, SOIE_A>;
impl<'a, REG> SOIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SOIE_A::B_0X0)
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SOIE_A::B_0X1)
    }
}
#[doc = "Field `EGE` reader - Event generation enable"]
pub type EGE_R = crate::BitReader<EGE_A>;
#[doc = "Event generation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EGE_A {
    #[doc = "0: event generation disabled"]
    B_0X0 = 0,
    #[doc = "1: event generation enabled"]
    B_0X1 = 1,
}
impl From<EGE_A> for bool {
    #[inline(always)]
    fn from(variant: EGE_A) -> Self {
        variant as u8 != 0
    }
}
impl EGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EGE_A {
        match self.bits {
            false => EGE_A::B_0X0,
            true => EGE_A::B_0X1,
        }
    }
    #[doc = "event generation disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EGE_A::B_0X0
    }
    #[doc = "event generation enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EGE_A::B_0X1
    }
}
#[doc = "Field `EGE` writer - Event generation enable"]
pub type EGE_W<'a, REG> = crate::BitWriter<'a, REG, EGE_A>;
impl<'a, REG> EGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "event generation disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EGE_A::B_0X0)
    }
    #[doc = "event generation enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EGE_A::B_0X1)
    }
}
#[doc = "Field `SE` reader - Synchronization enable"]
pub type SE_R = crate::BitReader<SE_A>;
#[doc = "Synchronization enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SE_A {
    #[doc = "0: synchronization disabled"]
    B_0X0 = 0,
    #[doc = "1: synchronization enabled"]
    B_0X1 = 1,
}
impl From<SE_A> for bool {
    #[inline(always)]
    fn from(variant: SE_A) -> Self {
        variant as u8 != 0
    }
}
impl SE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SE_A {
        match self.bits {
            false => SE_A::B_0X0,
            true => SE_A::B_0X1,
        }
    }
    #[doc = "synchronization disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SE_A::B_0X0
    }
    #[doc = "synchronization enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SE_A::B_0X1
    }
}
#[doc = "Field `SE` writer - Synchronization enable"]
pub type SE_W<'a, REG> = crate::BitWriter<'a, REG, SE_A>;
impl<'a, REG> SE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "synchronization disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SE_A::B_0X0)
    }
    #[doc = "synchronization enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SE_A::B_0X1)
    }
}
#[doc = "Field `SPOL` reader - Synchronization polarity Defines the edge polarity of the selected synchronization input:"]
pub type SPOL_R = crate::FieldReader<SPOL_A>;
#[doc = "Synchronization polarity Defines the edge polarity of the selected synchronization input:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPOL_A {
    #[doc = "0: no event, i.e. no synchronization nor detection."]
    B_0X0 = 0,
    #[doc = "1: rising edge"]
    B_0X1 = 1,
    #[doc = "2: falling edge"]
    B_0X2 = 2,
    #[doc = "3: rising and falling edge"]
    B_0X3 = 3,
}
impl From<SPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPOL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPOL_A {
    type Ux = u8;
}
impl SPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPOL_A {
        match self.bits {
            0 => SPOL_A::B_0X0,
            1 => SPOL_A::B_0X1,
            2 => SPOL_A::B_0X2,
            3 => SPOL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "no event, i.e. no synchronization nor detection."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPOL_A::B_0X0
    }
    #[doc = "rising edge"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPOL_A::B_0X1
    }
    #[doc = "falling edge"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SPOL_A::B_0X2
    }
    #[doc = "rising and falling edge"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SPOL_A::B_0X3
    }
}
#[doc = "Field `SPOL` writer - Synchronization polarity Defines the edge polarity of the selected synchronization input:"]
pub type SPOL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SPOL_A>;
impl<'a, REG> SPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no event, i.e. no synchronization nor detection."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL_A::B_0X0)
    }
    #[doc = "rising edge"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL_A::B_0X1)
    }
    #[doc = "falling edge"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL_A::B_0X2)
    }
    #[doc = "rising and falling edge"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL_A::B_0X3)
    }
}
#[doc = "Field `NBREQ` reader - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field shall only be written when both SE and EGE bits are low."]
pub type NBREQ_R = crate::FieldReader;
#[doc = "Field `NBREQ` writer - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field shall only be written when both SE and EGE bits are low."]
pub type NBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SYNC_ID` reader - Synchronization identification Selects the synchronization input (see inputs to resources STM32G0)."]
pub type SYNC_ID_R = crate::FieldReader;
#[doc = "Field `SYNC_ID` writer - Synchronization identification Selects the synchronization input (see inputs to resources STM32G0)."]
pub type SYNC_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources."]
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Synchronization enable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization polarity Defines the edge polarity of the selected synchronization input:"]
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field shall only be written when both SE and EGE bits are low."]
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Synchronization identification Selects the synchronization input (see inputs to resources STM32G0)."]
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - DMA request identification Selects the input DMA request. See the DMAMUX table about assignments of multiplexer inputs to resources."]
    #[inline(always)]
    #[must_use]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<DMAMUX_C6CR_SPEC> {
        DMAREQ_ID_W::new(self, 0)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn soie(&mut self) -> SOIE_W<DMAMUX_C6CR_SPEC> {
        SOIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn ege(&mut self) -> EGE_W<DMAMUX_C6CR_SPEC> {
        EGE_W::new(self, 9)
    }
    #[doc = "Bit 16 - Synchronization enable"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<DMAMUX_C6CR_SPEC> {
        SE_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - Synchronization polarity Defines the edge polarity of the selected synchronization input:"]
    #[inline(always)]
    #[must_use]
    pub fn spol(&mut self) -> SPOL_W<DMAMUX_C6CR_SPEC> {
        SPOL_W::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests minus 1 to forward Defines the number of DMA requests to forward to the DMA controller after a synchronization event, and/or the number of DMA requests before an output event is generated. This field shall only be written when both SE and EGE bits are low."]
    #[inline(always)]
    #[must_use]
    pub fn nbreq(&mut self) -> NBREQ_W<DMAMUX_C6CR_SPEC> {
        NBREQ_W::new(self, 19)
    }
    #[doc = "Bits 24:28 - Synchronization identification Selects the synchronization input (see inputs to resources STM32G0)."]
    #[inline(always)]
    #[must_use]
    pub fn sync_id(&mut self) -> SYNC_ID_W<DMAMUX_C6CR_SPEC> {
        SYNC_ID_W::new(self, 24)
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
#[doc = "DMAMUX request line multiplexer channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c6cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c6cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMUX_C6CR_SPEC;
impl crate::RegisterSpec for DMAMUX_C6CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamux_c6cr::R`](R) reader structure"]
impl crate::Readable for DMAMUX_C6CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmamux_c6cr::W`](W) writer structure"]
impl crate::Writable for DMAMUX_C6CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAMUX_C6CR to value 0"]
impl crate::Resettable for DMAMUX_C6CR_SPEC {
    const RESET_VALUE: u32 = 0;
}
