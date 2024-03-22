#[doc = "Register `MODER` reader"]
pub type R = crate::R<MODER_SPEC>;
#[doc = "Register `MODER` writer"]
pub type W = crate::W<MODER_SPEC>;
#[doc = "Field `MODER0` reader - Port mode"]
pub type MODER0_R = crate::FieldReader<MODER0_A>;
#[doc = "Port mode\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODER0_A {
    #[doc = "0: Input mode"]
    INPUT = 0,
    #[doc = "1: General purpose output mode"]
    OUTPUT = 1,
    #[doc = "2: Alternate function mode"]
    ALTERNATE_FUNCTION = 2,
    #[doc = "3: Analog mode (reset value for most ports)"]
    ANALOG = 3,
}
impl From<MODER0_A> for u8 {
    #[inline(always)]
    fn from(variant: MODER0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODER0_A {
    type Ux = u8;
}
impl MODER0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODER0_A {
        match self.bits {
            0 => MODER0_A::INPUT,
            1 => MODER0_A::OUTPUT,
            2 => MODER0_A::ALTERNATE_FUNCTION,
            3 => MODER0_A::ANALOG,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODER0_A::INPUT
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MODER0_A::OUTPUT
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_alternate_function(&self) -> bool {
        *self == MODER0_A::ALTERNATE_FUNCTION
    }
    #[doc = "Analog mode (reset value for most ports)"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == MODER0_A::ANALOG
    }
}
#[doc = "Field `MODER0` writer - Port mode"]
pub type MODER0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODER0_A>;
impl<'a, REG> MODER0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODER0_A::INPUT)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(MODER0_A::OUTPUT)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn alternate_function(self) -> &'a mut crate::W<REG> {
        self.variant(MODER0_A::ALTERNATE_FUNCTION)
    }
    #[doc = "Analog mode (reset value for most ports)"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(MODER0_A::ANALOG)
    }
}
#[doc = "Field `MODER1` reader - Port mode"]
pub use MODER0_R as MODER1_R;
#[doc = "Field `MODER2` reader - Port mode"]
pub use MODER0_R as MODER2_R;
#[doc = "Field `MODER3` reader - Port mode"]
pub use MODER0_R as MODER3_R;
#[doc = "Field `MODER4` reader - Port mode"]
pub use MODER0_R as MODER4_R;
#[doc = "Field `MODER5` reader - Port mode"]
pub use MODER0_R as MODER5_R;
#[doc = "Field `MODER6` reader - Port mode"]
pub use MODER0_R as MODER6_R;
#[doc = "Field `MODER7` reader - Port mode"]
pub use MODER0_R as MODER7_R;
#[doc = "Field `MODER8` reader - Port mode"]
pub use MODER0_R as MODER8_R;
#[doc = "Field `MODER9` reader - Port mode"]
pub use MODER0_R as MODER9_R;
#[doc = "Field `MODER10` reader - Port mode"]
pub use MODER0_R as MODER10_R;
#[doc = "Field `MODER11` reader - Port mode"]
pub use MODER0_R as MODER11_R;
#[doc = "Field `MODER12` reader - Port mode"]
pub use MODER0_R as MODER12_R;
#[doc = "Field `MODER13` reader - Port mode"]
pub use MODER0_R as MODER13_R;
#[doc = "Field `MODER14` reader - Port mode"]
pub use MODER0_R as MODER14_R;
#[doc = "Field `MODER15` reader - Port mode"]
pub use MODER0_R as MODER15_R;
#[doc = "Field `MODER1` writer - Port mode"]
pub use MODER0_W as MODER1_W;
#[doc = "Field `MODER2` writer - Port mode"]
pub use MODER0_W as MODER2_W;
#[doc = "Field `MODER3` writer - Port mode"]
pub use MODER0_W as MODER3_W;
#[doc = "Field `MODER4` writer - Port mode"]
pub use MODER0_W as MODER4_W;
#[doc = "Field `MODER5` writer - Port mode"]
pub use MODER0_W as MODER5_W;
#[doc = "Field `MODER6` writer - Port mode"]
pub use MODER0_W as MODER6_W;
#[doc = "Field `MODER7` writer - Port mode"]
pub use MODER0_W as MODER7_W;
#[doc = "Field `MODER8` writer - Port mode"]
pub use MODER0_W as MODER8_W;
#[doc = "Field `MODER9` writer - Port mode"]
pub use MODER0_W as MODER9_W;
#[doc = "Field `MODER10` writer - Port mode"]
pub use MODER0_W as MODER10_W;
#[doc = "Field `MODER11` writer - Port mode"]
pub use MODER0_W as MODER11_W;
#[doc = "Field `MODER12` writer - Port mode"]
pub use MODER0_W as MODER12_W;
#[doc = "Field `MODER13` writer - Port mode"]
pub use MODER0_W as MODER13_W;
#[doc = "Field `MODER14` writer - Port mode"]
pub use MODER0_W as MODER14_W;
#[doc = "Field `MODER15` writer - Port mode"]
pub use MODER0_W as MODER15_W;
impl R {
    #[doc = "Bits 0:1 - Port mode"]
    #[inline(always)]
    pub fn moder0(&self) -> MODER0_R {
        MODER0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port mode"]
    #[inline(always)]
    pub fn moder1(&self) -> MODER1_R {
        MODER1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port mode"]
    #[inline(always)]
    pub fn moder2(&self) -> MODER2_R {
        MODER2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port mode"]
    #[inline(always)]
    pub fn moder3(&self) -> MODER3_R {
        MODER3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port mode"]
    #[inline(always)]
    pub fn moder4(&self) -> MODER4_R {
        MODER4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port mode"]
    #[inline(always)]
    pub fn moder5(&self) -> MODER5_R {
        MODER5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port mode"]
    #[inline(always)]
    pub fn moder6(&self) -> MODER6_R {
        MODER6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port mode"]
    #[inline(always)]
    pub fn moder7(&self) -> MODER7_R {
        MODER7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port mode"]
    #[inline(always)]
    pub fn moder8(&self) -> MODER8_R {
        MODER8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port mode"]
    #[inline(always)]
    pub fn moder9(&self) -> MODER9_R {
        MODER9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port mode"]
    #[inline(always)]
    pub fn moder10(&self) -> MODER10_R {
        MODER10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port mode"]
    #[inline(always)]
    pub fn moder11(&self) -> MODER11_R {
        MODER11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port mode"]
    #[inline(always)]
    pub fn moder12(&self) -> MODER12_R {
        MODER12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port mode"]
    #[inline(always)]
    pub fn moder13(&self) -> MODER13_R {
        MODER13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port mode"]
    #[inline(always)]
    pub fn moder14(&self) -> MODER14_R {
        MODER14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port mode"]
    #[inline(always)]
    pub fn moder15(&self) -> MODER15_R {
        MODER15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port mode"]
    #[inline(always)]
    #[must_use]
    pub fn moder0(&mut self) -> MODER0_W<MODER_SPEC> {
        MODER0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port mode"]
    #[inline(always)]
    #[must_use]
    pub fn moder1(&mut self) -> MODER1_W<MODER_SPEC> {
        MODER1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port mode"]
    #[inline(always)]
    #[must_use]
    pub fn moder2(&mut self) -> MODER2_W<MODER_SPEC> {
        MODER2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port mode"]
    #[inline(always)]
    #[must_use]
    pub fn moder3(&mut self) -> MODER3_W<MODER_SPEC> {
        MODER3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port mode"]
    #[inline(always)]
    #[must_use]
    pub fn moder4(&mut self) -> MODER4_W<MODER_SPEC> {
        MODER4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port mode"]
    #[inline(always)]
    #[must_use]
    pub fn moder5(&mut self) -> MODER5_W<MODER_SPEC> {
        MODER5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port mode"]
    #[inline(always)]
    #[must_use]
    pub fn moder6(&mut self) -> MODER6_W<MODER_SPEC> {
        MODER6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port mode"]
    #[inline(always)]
    #[must_use]
    pub fn moder7(&mut self) -> MODER7_W<MODER_SPEC> {
        MODER7_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port mode"]
    #[inline(always)]
    #[must_use]
    pub fn moder8(&mut self) -> MODER8_W<MODER_SPEC> {
        MODER8_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port mode"]
    #[inline(always)]
    #[must_use]
    pub fn moder9(&mut self) -> MODER9_W<MODER_SPEC> {
        MODER9_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port mode"]
    #[inline(always)]
    #[must_use]
    pub fn moder10(&mut self) -> MODER10_W<MODER_SPEC> {
        MODER10_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port mode"]
    #[inline(always)]
    #[must_use]
    pub fn moder11(&mut self) -> MODER11_W<MODER_SPEC> {
        MODER11_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port mode"]
    #[inline(always)]
    #[must_use]
    pub fn moder12(&mut self) -> MODER12_W<MODER_SPEC> {
        MODER12_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port mode"]
    #[inline(always)]
    #[must_use]
    pub fn moder13(&mut self) -> MODER13_W<MODER_SPEC> {
        MODER13_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port mode"]
    #[inline(always)]
    #[must_use]
    pub fn moder14(&mut self) -> MODER14_W<MODER_SPEC> {
        MODER14_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port mode"]
    #[inline(always)]
    #[must_use]
    pub fn moder15(&mut self) -> MODER15_W<MODER_SPEC> {
        MODER15_W::new(self, 30)
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
#[doc = "GPIO port mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moder::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODER_SPEC;
impl crate::RegisterSpec for MODER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moder::R`](R) reader structure"]
impl crate::Readable for MODER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`moder::W`](W) writer structure"]
impl crate::Writable for MODER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODER to value 0xffff_ffff"]
impl crate::Resettable for MODER_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
