#[doc = "Register `CRC_CR` reader"]
pub type R = crate::R<CRC_CR_SPEC>;
#[doc = "Register `CRC_CR` writer"]
pub type W = crate::W<CRC_CR_SPEC>;
#[doc = "Field `RESET` writer - RESET bit"]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLYSIZE` reader - Polynomial size These bits control the size of the polynomial."]
pub type POLYSIZE_R = crate::FieldReader<POLYSIZE_A>;
#[doc = "Polynomial size These bits control the size of the polynomial.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POLYSIZE_A {
    #[doc = "0: 32 bit polynomial"]
    B_0X0 = 0,
    #[doc = "1: 16 bit polynomial"]
    B_0X1 = 1,
    #[doc = "2: 8 bit polynomial"]
    B_0X2 = 2,
    #[doc = "3: 7 bit polynomial"]
    B_0X3 = 3,
}
impl From<POLYSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: POLYSIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for POLYSIZE_A {
    type Ux = u8;
}
impl POLYSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POLYSIZE_A {
        match self.bits {
            0 => POLYSIZE_A::B_0X0,
            1 => POLYSIZE_A::B_0X1,
            2 => POLYSIZE_A::B_0X2,
            3 => POLYSIZE_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "32 bit polynomial"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == POLYSIZE_A::B_0X0
    }
    #[doc = "16 bit polynomial"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == POLYSIZE_A::B_0X1
    }
    #[doc = "8 bit polynomial"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == POLYSIZE_A::B_0X2
    }
    #[doc = "7 bit polynomial"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == POLYSIZE_A::B_0X3
    }
}
#[doc = "Field `POLYSIZE` writer - Polynomial size These bits control the size of the polynomial."]
pub type POLYSIZE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, POLYSIZE_A>;
impl<'a, REG> POLYSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32 bit polynomial"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE_A::B_0X0)
    }
    #[doc = "16 bit polynomial"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE_A::B_0X1)
    }
    #[doc = "8 bit polynomial"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE_A::B_0X2)
    }
    #[doc = "7 bit polynomial"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(POLYSIZE_A::B_0X3)
    }
}
#[doc = "Field `REV_IN` reader - Reverse input data These bits control the reversal of the bit order of the input data"]
pub type REV_IN_R = crate::FieldReader<REV_IN_A>;
#[doc = "Reverse input data These bits control the reversal of the bit order of the input data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV_IN_A {
    #[doc = "0: Bit order not affected"]
    B_0X0 = 0,
    #[doc = "1: Bit reversal done by byte"]
    B_0X1 = 1,
    #[doc = "2: Bit reversal done by half-word"]
    B_0X2 = 2,
    #[doc = "3: Bit reversal done by word"]
    B_0X3 = 3,
}
impl From<REV_IN_A> for u8 {
    #[inline(always)]
    fn from(variant: REV_IN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REV_IN_A {
    type Ux = u8;
}
impl REV_IN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REV_IN_A {
        match self.bits {
            0 => REV_IN_A::B_0X0,
            1 => REV_IN_A::B_0X1,
            2 => REV_IN_A::B_0X2,
            3 => REV_IN_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == REV_IN_A::B_0X0
    }
    #[doc = "Bit reversal done by byte"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == REV_IN_A::B_0X1
    }
    #[doc = "Bit reversal done by half-word"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == REV_IN_A::B_0X2
    }
    #[doc = "Bit reversal done by word"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == REV_IN_A::B_0X3
    }
}
#[doc = "Field `REV_IN` writer - Reverse input data These bits control the reversal of the bit order of the input data"]
pub type REV_IN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, REV_IN_A>;
impl<'a, REG> REV_IN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN_A::B_0X0)
    }
    #[doc = "Bit reversal done by byte"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN_A::B_0X1)
    }
    #[doc = "Bit reversal done by half-word"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN_A::B_0X2)
    }
    #[doc = "Bit reversal done by word"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(REV_IN_A::B_0X3)
    }
}
#[doc = "Field `REV_OUT` reader - Reverse output data This bit controls the reversal of the bit order of the output data."]
pub type REV_OUT_R = crate::BitReader<REV_OUT_A>;
#[doc = "Reverse output data This bit controls the reversal of the bit order of the output data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV_OUT_A {
    #[doc = "0: Bit order not affected"]
    B_0X0 = 0,
    #[doc = "1: Bit-reversed output format"]
    B_0X1 = 1,
}
impl From<REV_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: REV_OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REV_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REV_OUT_A {
        match self.bits {
            false => REV_OUT_A::B_0X0,
            true => REV_OUT_A::B_0X1,
        }
    }
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == REV_OUT_A::B_0X0
    }
    #[doc = "Bit-reversed output format"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == REV_OUT_A::B_0X1
    }
}
#[doc = "Field `REV_OUT` writer - Reverse output data This bit controls the reversal of the bit order of the output data."]
pub type REV_OUT_W<'a, REG> = crate::BitWriter<'a, REG, REV_OUT_A>;
impl<'a, REG> REV_OUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(REV_OUT_A::B_0X0)
    }
    #[doc = "Bit-reversed output format"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(REV_OUT_A::B_0X1)
    }
}
impl R {
    #[doc = "Bits 3:4 - Polynomial size These bits control the size of the polynomial."]
    #[inline(always)]
    pub fn polysize(&self) -> POLYSIZE_R {
        POLYSIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Reverse input data These bits control the reversal of the bit order of the input data"]
    #[inline(always)]
    pub fn rev_in(&self) -> REV_IN_R {
        REV_IN_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Reverse output data This bit controls the reversal of the bit order of the output data."]
    #[inline(always)]
    pub fn rev_out(&self) -> REV_OUT_R {
        REV_OUT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RESET bit"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<CRC_CR_SPEC> {
        RESET_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Polynomial size These bits control the size of the polynomial."]
    #[inline(always)]
    #[must_use]
    pub fn polysize(&mut self) -> POLYSIZE_W<CRC_CR_SPEC> {
        POLYSIZE_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - Reverse input data These bits control the reversal of the bit order of the input data"]
    #[inline(always)]
    #[must_use]
    pub fn rev_in(&mut self) -> REV_IN_W<CRC_CR_SPEC> {
        REV_IN_W::new(self, 5)
    }
    #[doc = "Bit 7 - Reverse output data This bit controls the reversal of the bit order of the output data."]
    #[inline(always)]
    #[must_use]
    pub fn rev_out(&mut self) -> REV_OUT_W<CRC_CR_SPEC> {
        REV_OUT_W::new(self, 7)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRC_CR_SPEC;
impl crate::RegisterSpec for CRC_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_cr::R`](R) reader structure"]
impl crate::Readable for CRC_CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crc_cr::W`](W) writer structure"]
impl crate::Writable for CRC_CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_CR to value 0"]
impl crate::Resettable for CRC_CR_SPEC {
    const RESET_VALUE: u32 = 0;
}
