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
    DIV1 = 0,
    #[doc = "1: input clock divided by 2"]
    DIV2 = 1,
    #[doc = "2: input clock divided by 4"]
    DIV4 = 2,
    #[doc = "3: input clock divided by 6"]
    DIV6 = 3,
    #[doc = "4: input clock divided by 8"]
    DIV8 = 4,
    #[doc = "5: input clock divided by 10"]
    DIV10 = 5,
    #[doc = "6: input clock divided by 12"]
    DIV12 = 6,
    #[doc = "7: input clock divided by 16"]
    DIV16 = 7,
    #[doc = "8: input clock divided by 32"]
    DIV32 = 8,
    #[doc = "9: input clock divided by 64"]
    DIV64 = 9,
    #[doc = "10: input clock divided by 128"]
    DIV128 = 10,
    #[doc = "11: input clock divided by 256"]
    DIV256 = 11,
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
            0 => Some(PRESCALER_A::DIV1),
            1 => Some(PRESCALER_A::DIV2),
            2 => Some(PRESCALER_A::DIV4),
            3 => Some(PRESCALER_A::DIV6),
            4 => Some(PRESCALER_A::DIV8),
            5 => Some(PRESCALER_A::DIV10),
            6 => Some(PRESCALER_A::DIV12),
            7 => Some(PRESCALER_A::DIV16),
            8 => Some(PRESCALER_A::DIV32),
            9 => Some(PRESCALER_A::DIV64),
            10 => Some(PRESCALER_A::DIV128),
            11 => Some(PRESCALER_A::DIV256),
            _ => None,
        }
    }
    #[doc = "input clock not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESCALER_A::DIV1
    }
    #[doc = "input clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALER_A::DIV2
    }
    #[doc = "input clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALER_A::DIV4
    }
    #[doc = "input clock divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PRESCALER_A::DIV6
    }
    #[doc = "input clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALER_A::DIV8
    }
    #[doc = "input clock divided by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PRESCALER_A::DIV10
    }
    #[doc = "input clock divided by 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PRESCALER_A::DIV12
    }
    #[doc = "input clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALER_A::DIV16
    }
    #[doc = "input clock divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALER_A::DIV32
    }
    #[doc = "input clock divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALER_A::DIV64
    }
    #[doc = "input clock divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALER_A::DIV128
    }
    #[doc = "input clock divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALER_A::DIV256
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
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV1)
    }
    #[doc = "input clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV2)
    }
    #[doc = "input clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV4)
    }
    #[doc = "input clock divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV6)
    }
    #[doc = "input clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV8)
    }
    #[doc = "input clock divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV10)
    }
    #[doc = "input clock divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV12)
    }
    #[doc = "input clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV16)
    }
    #[doc = "input clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV32)
    }
    #[doc = "input clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV64)
    }
    #[doc = "input clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV128)
    }
    #[doc = "input clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALER_A::DIV256)
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
