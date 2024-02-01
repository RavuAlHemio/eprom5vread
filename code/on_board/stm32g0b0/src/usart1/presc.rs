#[doc = "Register `PRESC` reader"]
pub type R = crate::R<PRESC_SPEC>;
#[doc = "Register `PRESC` writer"]
pub type W = crate::W<PRESC_SPEC>;
#[doc = "Field `PRESCALER` reader - Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256."]
pub type PRESCALER_R = crate::FieldReader<PRESCALER_A>;
#[doc = "Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALER_A {
    #[doc = "0: input clock not divided"]
    B_0X0 = 0,
    #[doc = "1: input clock divided by 2"]
    B_0X1 = 1,
    #[doc = "2: input clock divided by 4"]
    B_0X2 = 2,
    #[doc = "3: input clock divided by 6"]
    B_0X3 = 3,
    #[doc = "4: input clock divided by 8"]
    B_0X4 = 4,
    #[doc = "5: input clock divided by 10"]
    B_0X5 = 5,
    #[doc = "6: input clock divided by 12"]
    B_0X6 = 6,
    #[doc = "7: input clock divided by 16"]
    B_0X7 = 7,
    #[doc = "8: input clock divided by 32"]
    B_0X8 = 8,
    #[doc = "9: input clock divided by 64"]
    B_0X9 = 9,
    #[doc = "10: input clock divided by 128"]
    B_0X_A = 10,
    #[doc = "11: input clock divided by 256"]
    B_0X_B = 11,
}
impl From<PRESCALER_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESCALER_A {
    type Ux = u8;
}
impl PRESCALER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESCALER_A> {
        match self.bits {
            0 => Some(PRESCALER_A::B_0X0),
            1 => Some(PRESCALER_A::B_0X1),
            2 => Some(PRESCALER_A::B_0X2),
            3 => Some(PRESCALER_A::B_0X3),
            4 => Some(PRESCALER_A::B_0X4),
            5 => Some(PRESCALER_A::B_0X5),
            6 => Some(PRESCALER_A::B_0X6),
            7 => Some(PRESCALER_A::B_0X7),
            8 => Some(PRESCALER_A::B_0X8),
            9 => Some(PRESCALER_A::B_0X9),
            10 => Some(PRESCALER_A::B_0X_A),
            11 => Some(PRESCALER_A::B_0X_B),
            _ => None,
        }
    }
    #[doc = "input clock not divided"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PRESCALER_A::B_0X0
    }
    #[doc = "input clock divided by 2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PRESCALER_A::B_0X1
    }
    #[doc = "input clock divided by 4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PRESCALER_A::B_0X2
    }
    #[doc = "input clock divided by 6"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PRESCALER_A::B_0X3
    }
    #[doc = "input clock divided by 8"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == PRESCALER_A::B_0X4
    }
    #[doc = "input clock divided by 10"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == PRESCALER_A::B_0X5
    }
    #[doc = "input clock divided by 12"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == PRESCALER_A::B_0X6
    }
    #[doc = "input clock divided by 16"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == PRESCALER_A::B_0X7
    }
    #[doc = "input clock divided by 32"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == PRESCALER_A::B_0X8
    }
    #[doc = "input clock divided by 64"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == PRESCALER_A::B_0X9
    }
    #[doc = "input clock divided by 128"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == PRESCALER_A::B_0X_A
    }
    #[doc = "input clock divided by 256"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == PRESCALER_A::B_0X_B
    }
}
#[doc = "Field `PRESCALER` writer - Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256."]
pub type PRESCALER_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRESCALER_A>;
impl<'a, REG> PRESCALER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "input clock not divided"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0X0)
    }
    #[doc = "input clock divided by 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0X1)
    }
    #[doc = "input clock divided by 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0X2)
    }
    #[doc = "input clock divided by 6"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0X3)
    }
    #[doc = "input clock divided by 8"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0X4)
    }
    #[doc = "input clock divided by 10"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0X5)
    }
    #[doc = "input clock divided by 12"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0X6)
    }
    #[doc = "input clock divided by 16"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0X7)
    }
    #[doc = "input clock divided by 32"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0X8)
    }
    #[doc = "input clock divided by 64"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0X9)
    }
    #[doc = "input clock divided by 128"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0X_A)
    }
    #[doc = "input clock divided by 256"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::B_0X_B)
    }
}
impl R {
    #[doc = "Bits 0:3 - Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256."]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256."]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<PRESC_SPEC> {
        PRESCALER_W::new(self, 0)
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
#[doc = "Prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`presc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`presc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRESC_SPEC;
impl crate::RegisterSpec for PRESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`presc::R`](R) reader structure"]
impl crate::Readable for PRESC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`presc::W`](W) writer structure"]
impl crate::Writable for PRESC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRESC to value 0"]
impl crate::Resettable for PRESC_SPEC {
    const RESET_VALUE: u32 = 0;
}
