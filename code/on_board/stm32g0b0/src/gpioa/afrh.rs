#[doc = "Register `AFRH` reader"]
pub type R = crate::R<AFRH_SPEC>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::W<AFRH_SPEC>;
#[doc = "Field `AFSEL8` reader - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub type AFSEL8_R = crate::FieldReader<AFSEL8_A>;
#[doc = "Selection of the specific alternate function (AF) if this pin is set to alternate function mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL8_A {
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
impl From<AFSEL8_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL8_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL8_A {
    type Ux = u8;
}
impl AFSEL8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AFSEL8_A> {
        match self.bits {
            0 => Some(AFSEL8_A::AF0),
            1 => Some(AFSEL8_A::AF1),
            2 => Some(AFSEL8_A::AF2),
            3 => Some(AFSEL8_A::AF3),
            4 => Some(AFSEL8_A::AF4),
            5 => Some(AFSEL8_A::AF5),
            6 => Some(AFSEL8_A::AF6),
            7 => Some(AFSEL8_A::AF7),
            _ => None,
        }
    }
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        *self == AFSEL8_A::AF0
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        *self == AFSEL8_A::AF1
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        *self == AFSEL8_A::AF2
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        *self == AFSEL8_A::AF3
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        *self == AFSEL8_A::AF4
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        *self == AFSEL8_A::AF5
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        *self == AFSEL8_A::AF6
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        *self == AFSEL8_A::AF7
    }
}
#[doc = "Field `AFSEL8` writer - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub type AFSEL8_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL8_A>;
impl<'a, REG> AFSEL8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Alternate function 0"]
    #[inline(always)]
    pub fn af0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::AF0)
    }
    #[doc = "Alternate function 1"]
    #[inline(always)]
    pub fn af1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::AF1)
    }
    #[doc = "Alternate function 2"]
    #[inline(always)]
    pub fn af2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::AF2)
    }
    #[doc = "Alternate function 3"]
    #[inline(always)]
    pub fn af3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::AF3)
    }
    #[doc = "Alternate function 4"]
    #[inline(always)]
    pub fn af4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::AF4)
    }
    #[doc = "Alternate function 5"]
    #[inline(always)]
    pub fn af5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::AF5)
    }
    #[doc = "Alternate function 6"]
    #[inline(always)]
    pub fn af6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::AF6)
    }
    #[doc = "Alternate function 7"]
    #[inline(always)]
    pub fn af7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8_A::AF7)
    }
}
#[doc = "Field `AFSEL9` reader - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL8_R as AFSEL9_R;
#[doc = "Field `AFSEL10` reader - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL8_R as AFSEL10_R;
#[doc = "Field `AFSEL11` reader - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL8_R as AFSEL11_R;
#[doc = "Field `AFSEL12` reader - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL8_R as AFSEL12_R;
#[doc = "Field `AFSEL13` reader - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL8_R as AFSEL13_R;
#[doc = "Field `AFSEL14` reader - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL8_R as AFSEL14_R;
#[doc = "Field `AFSEL15` reader - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL8_R as AFSEL15_R;
#[doc = "Field `AFSEL9` writer - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL8_W as AFSEL9_W;
#[doc = "Field `AFSEL10` writer - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL8_W as AFSEL10_W;
#[doc = "Field `AFSEL11` writer - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL8_W as AFSEL11_W;
#[doc = "Field `AFSEL12` writer - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL8_W as AFSEL12_W;
#[doc = "Field `AFSEL13` writer - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL8_W as AFSEL13_W;
#[doc = "Field `AFSEL14` writer - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL8_W as AFSEL14_W;
#[doc = "Field `AFSEL15` writer - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
pub use AFSEL8_W as AFSEL15_W;
impl R {
    #[doc = "Bits 0:3 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    pub fn afsel8(&self) -> AFSEL8_R {
        AFSEL8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    pub fn afsel9(&self) -> AFSEL9_R {
        AFSEL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    pub fn afsel10(&self) -> AFSEL10_R {
        AFSEL10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    pub fn afsel11(&self) -> AFSEL11_R {
        AFSEL11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    pub fn afsel12(&self) -> AFSEL12_R {
        AFSEL12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    pub fn afsel13(&self) -> AFSEL13_R {
        AFSEL13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    pub fn afsel14(&self) -> AFSEL14_R {
        AFSEL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    pub fn afsel15(&self) -> AFSEL15_R {
        AFSEL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    #[must_use]
    pub fn afsel8(&mut self) -> AFSEL8_W<AFRH_SPEC> {
        AFSEL8_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    #[must_use]
    pub fn afsel9(&mut self) -> AFSEL9_W<AFRH_SPEC> {
        AFSEL9_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    #[must_use]
    pub fn afsel10(&mut self) -> AFSEL10_W<AFRH_SPEC> {
        AFSEL10_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    #[must_use]
    pub fn afsel11(&mut self) -> AFSEL11_W<AFRH_SPEC> {
        AFSEL11_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    #[must_use]
    pub fn afsel12(&mut self) -> AFSEL12_W<AFRH_SPEC> {
        AFSEL12_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    #[must_use]
    pub fn afsel13(&mut self) -> AFSEL13_W<AFRH_SPEC> {
        AFSEL13_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    #[must_use]
    pub fn afsel14(&mut self) -> AFSEL14_W<AFRH_SPEC> {
        AFSEL14_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Selection of the specific alternate function (AF) if this pin is set to alternate function mode."]
    #[inline(always)]
    #[must_use]
    pub fn afsel15(&mut self) -> AFSEL15_W<AFRH_SPEC> {
        AFSEL15_W::new(self, 28)
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
#[doc = "GPIO alternate function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFRH_SPEC;
impl crate::RegisterSpec for AFRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrh::R`](R) reader structure"]
impl crate::Readable for AFRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`afrh::W`](W) writer structure"]
impl crate::Writable for AFRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFRH to value 0"]
impl crate::Resettable for AFRH_SPEC {
    const RESET_VALUE: u32 = 0;
}
