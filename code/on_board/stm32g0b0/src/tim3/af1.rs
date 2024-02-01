#[doc = "Register `AF1` reader"]
pub type R = crate::R<AF1_SPEC>;
#[doc = "Register `AF1` writer"]
pub type W = crate::W<AF1_SPEC>;
#[doc = "Field `ETRSEL` reader - ETR source selection These bits select the ETR input source. Others: Reserved"]
pub type ETRSEL_R = crate::FieldReader<ETRSEL_A>;
#[doc = "ETR source selection These bits select the ETR input source. Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETRSEL_A {
    #[doc = "0: ETR legacy mode"]
    B_0X0 = 0,
    #[doc = "1: COMP1"]
    B_0X1 = 1,
    #[doc = "2: COMP2"]
    B_0X2 = 2,
    #[doc = "3: LSE"]
    B_0X3 = 3,
}
impl From<ETRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ETRSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETRSEL_A {
    type Ux = u8;
}
impl ETRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ETRSEL_A> {
        match self.bits {
            0 => Some(ETRSEL_A::B_0X0),
            1 => Some(ETRSEL_A::B_0X1),
            2 => Some(ETRSEL_A::B_0X2),
            3 => Some(ETRSEL_A::B_0X3),
            _ => None,
        }
    }
    #[doc = "ETR legacy mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETRSEL_A::B_0X0
    }
    #[doc = "COMP1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETRSEL_A::B_0X1
    }
    #[doc = "COMP2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ETRSEL_A::B_0X2
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ETRSEL_A::B_0X3
    }
}
#[doc = "Field `ETRSEL` writer - ETR source selection These bits select the ETR input source. Others: Reserved"]
pub type ETRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ETRSEL_A>;
impl<'a, REG> ETRSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ETR legacy mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL_A::B_0X0)
    }
    #[doc = "COMP1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL_A::B_0X1)
    }
    #[doc = "COMP2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL_A::B_0X2)
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL_A::B_0X3)
    }
}
impl R {
    #[doc = "Bits 14:17 - ETR source selection These bits select the ETR input source. Others: Reserved"]
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 14:17 - ETR source selection These bits select the ETR input source. Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn etrsel(&mut self) -> ETRSEL_W<AF1_SPEC> {
        ETRSEL_W::new(self, 14)
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
#[doc = "TIM alternate function option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF1_SPEC;
impl crate::RegisterSpec for AF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af1::R`](R) reader structure"]
impl crate::Readable for AF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`af1::W`](W) writer structure"]
impl crate::Writable for AF1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AF1 to value 0"]
impl crate::Resettable for AF1_SPEC {
    const RESET_VALUE: u32 = 0;
}
