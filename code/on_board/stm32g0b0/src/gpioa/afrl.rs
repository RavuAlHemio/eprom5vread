#[doc = "Register `AFRL` reader"]
pub type R = crate::R<AFRL_SPEC>;
#[doc = "Register `AFRL` writer"]
pub type W = crate::W<AFRL_SPEC>;
#[doc = "Field `AFSEL0` reader - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub type AFSEL0_R = crate::FieldReader<AFSEL0_A>;
#[doc = "Selection of the specific alternate function (AF) if this pin is set to alternate function mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL0_A {
    #[doc = "0: Alternate function 0"]
    AF0 = 0,
    #[doc = "1: Alternate function 1"]
    AF1 = 1,
    #[doc = "2: Alternate function 2"]
    AF2 = 2,
    #[doc = "3: Alternate function 3"]
    AF3 = 3,
    #[doc = "4: Alternate function 4"]
    AF4 = 4,
    #[doc = "5: Alternate function 5"]
    AF5 = 5,
    #[doc = "6: Alternate function 6"]
    AF6 = 6,
    #[doc = "7: Alternate function 7"]
    AF7 = 7,
}
impl From<AFSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL0_A {
    type Ux = u8;
}
impl AFSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AFSEL0_A> {
        match self.bits {
            0 => Some(AFSEL0_A::AF0),
            1 => Some(AFSEL0_A::AF1),
            2 => Some(AFSEL0_A::AF2),
            3 => Some(AFSEL0_A::AF3),
            4 => Some(AFSEL0_A::AF4),
            5 => Some(AFSEL0_A::AF5),
            6 => Some(AFSEL0_A::AF6),
            7 => Some(AFSEL0_A::AF7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == AFSEL0_A::AF0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == AFSEL0_A::AF1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == AFSEL0_A::AF2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == AFSEL0_A::AF3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == AFSEL0_A::AF4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == AFSEL0_A::AF5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == AFSEL0_A::AF6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == AFSEL0_A::AF7
    }
}
#[doc = "Field `AFSEL0` writer - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub type AFSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL0_A>;
impl<'a, REG> AFSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::AF0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::AF1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::AF2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::AF3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::AF4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::AF5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::AF6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0_A::AF7)
    }
}
#[doc = "Field `AFSEL1` reader - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL0_R as AFSEL1_R;
#[doc = "Field `AFSEL2` reader - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL0_R as AFSEL2_R;
#[doc = "Field `AFSEL3` reader - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL0_R as AFSEL3_R;
#[doc = "Field `AFSEL4` reader - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL0_R as AFSEL4_R;
#[doc = "Field `AFSEL5` reader - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL0_R as AFSEL5_R;
#[doc = "Field `AFSEL6` reader - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL0_R as AFSEL6_R;
#[doc = "Field `AFSEL7` reader - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL0_R as AFSEL7_R;
#[doc = "Field `AFSEL1` writer - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL0_W as AFSEL1_W;
#[doc = "Field `AFSEL2` writer - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL0_W as AFSEL2_W;
#[doc = "Field `AFSEL3` writer - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL0_W as AFSEL3_W;
#[doc = "Field `AFSEL4` writer - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL0_W as AFSEL4_W;
#[doc = "Field `AFSEL5` writer - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL0_W as AFSEL5_W;
#[doc = "Field `AFSEL6` writer - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL0_W as AFSEL6_W;
#[doc = "Field `AFSEL7` writer - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL0_W as AFSEL7_W;
impl R {
    #[doc = "Bits 0:3 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    pub fn afsel0(&self) -> AFSEL0_R {
        AFSEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    pub fn afsel1(&self) -> AFSEL1_R {
        AFSEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    pub fn afsel2(&self) -> AFSEL2_R {
        AFSEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    pub fn afsel3(&self) -> AFSEL3_R {
        AFSEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    pub fn afsel4(&self) -> AFSEL4_R {
        AFSEL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    pub fn afsel5(&self) -> AFSEL5_R {
        AFSEL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    pub fn afsel6(&self) -> AFSEL6_R {
        AFSEL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    pub fn afsel7(&self) -> AFSEL7_R {
        AFSEL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    #[must_use]
    pub fn afsel0(&mut self) -> AFSEL0_W<AFRL_SPEC> {
        AFSEL0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    #[must_use]
    pub fn afsel1(&mut self) -> AFSEL1_W<AFRL_SPEC> {
        AFSEL1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    #[must_use]
    pub fn afsel2(&mut self) -> AFSEL2_W<AFRL_SPEC> {
        AFSEL2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    #[must_use]
    pub fn afsel3(&mut self) -> AFSEL3_W<AFRL_SPEC> {
        AFSEL3_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    #[must_use]
    pub fn afsel4(&mut self) -> AFSEL4_W<AFRL_SPEC> {
        AFSEL4_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    #[must_use]
    pub fn afsel5(&mut self) -> AFSEL5_W<AFRL_SPEC> {
        AFSEL5_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    #[must_use]
    pub fn afsel6(&mut self) -> AFSEL6_W<AFRL_SPEC> {
        AFSEL6_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    #[must_use]
    pub fn afsel7(&mut self) -> AFSEL7_W<AFRL_SPEC> {
        AFSEL7_W::new(self, 28)
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
#[doc = "GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFRL_SPEC;
impl crate::RegisterSpec for AFRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrl::R`](R) reader structure"]
impl crate::Readable for AFRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`afrl::W`](W) writer structure"]
impl crate::Writable for AFRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::Resettable for AFRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
