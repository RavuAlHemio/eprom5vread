#[doc = "Register `I2C_OAR2` reader"]
pub type R = crate::R<I2C_OAR2_SPEC>;
#[doc = "Register `I2C_OAR2` writer"]
pub type W = crate::W<I2C_OAR2_SPEC>;
#[doc = "Field `OA2` reader - Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN=0."]
pub type OA2_R = crate::FieldReader;
#[doc = "Field `OA2` writer - Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN=0."]
pub type OA2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `OA2MSK` reader - Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches."]
pub type OA2MSK_R = crate::FieldReader<OA2MSK_A>;
#[doc = "Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OA2MSK_A {
    #[doc = "0: No mask"]
    B_0X0 = 0,
    #[doc = "1: OA2\\[1\\]
is masked and don't care. Only OA2\\[7:2\\]
are compared."]
    B_0X1 = 1,
    #[doc = "2: OA2\\[2:1\\]
are masked and don't care. Only OA2\\[7:3\\]
are compared."]
    B_0X2 = 2,
    #[doc = "3: OA2\\[3:1\\]
are masked and don't care. Only OA2\\[7:4\\]
are compared."]
    B_0X3 = 3,
    #[doc = "4: OA2\\[4:1\\]
are masked and don't care. Only OA2\\[7:5\\]
are compared."]
    B_0X4 = 4,
    #[doc = "5: OA2\\[5:1\\]
are masked and don't care. Only OA2\\[7:6\\]
are compared."]
    B_0X5 = 5,
    #[doc = "6: OA2\\[6:1\\]
are masked and don't care. Only OA2\\[7\\]
is compared."]
    B_0X6 = 6,
    #[doc = "7: OA2\\[7:1\\]
are masked and don't care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged."]
    B_0X7 = 7,
}
impl From<OA2MSK_A> for u8 {
    #[inline(always)]
    fn from(variant: OA2MSK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OA2MSK_A {
    type Ux = u8;
}
impl OA2MSK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OA2MSK_A {
        match self.bits {
            0 => OA2MSK_A::B_0X0,
            1 => OA2MSK_A::B_0X1,
            2 => OA2MSK_A::B_0X2,
            3 => OA2MSK_A::B_0X3,
            4 => OA2MSK_A::B_0X4,
            5 => OA2MSK_A::B_0X5,
            6 => OA2MSK_A::B_0X6,
            7 => OA2MSK_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "No mask"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OA2MSK_A::B_0X0
    }
    #[doc = "OA2\\[1\\]
is masked and don't care. Only OA2\\[7:2\\]
are compared."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OA2MSK_A::B_0X1
    }
    #[doc = "OA2\\[2:1\\]
are masked and don't care. Only OA2\\[7:3\\]
are compared."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OA2MSK_A::B_0X2
    }
    #[doc = "OA2\\[3:1\\]
are masked and don't care. Only OA2\\[7:4\\]
are compared."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OA2MSK_A::B_0X3
    }
    #[doc = "OA2\\[4:1\\]
are masked and don't care. Only OA2\\[7:5\\]
are compared."]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == OA2MSK_A::B_0X4
    }
    #[doc = "OA2\\[5:1\\]
are masked and don't care. Only OA2\\[7:6\\]
are compared."]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == OA2MSK_A::B_0X5
    }
    #[doc = "OA2\\[6:1\\]
are masked and don't care. Only OA2\\[7\\]
is compared."]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == OA2MSK_A::B_0X6
    }
    #[doc = "OA2\\[7:1\\]
are masked and don't care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged."]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == OA2MSK_A::B_0X7
    }
}
#[doc = "Field `OA2MSK` writer - Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches."]
pub type OA2MSK_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, OA2MSK_A>;
impl<'a, REG> OA2MSK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No mask"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK_A::B_0X0)
    }
    #[doc = "OA2\\[1\\]
is masked and don't care. Only OA2\\[7:2\\]
are compared."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK_A::B_0X1)
    }
    #[doc = "OA2\\[2:1\\]
are masked and don't care. Only OA2\\[7:3\\]
are compared."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK_A::B_0X2)
    }
    #[doc = "OA2\\[3:1\\]
are masked and don't care. Only OA2\\[7:4\\]
are compared."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK_A::B_0X3)
    }
    #[doc = "OA2\\[4:1\\]
are masked and don't care. Only OA2\\[7:5\\]
are compared."]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK_A::B_0X4)
    }
    #[doc = "OA2\\[5:1\\]
are masked and don't care. Only OA2\\[7:6\\]
are compared."]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK_A::B_0X5)
    }
    #[doc = "OA2\\[6:1\\]
are masked and don't care. Only OA2\\[7\\]
is compared."]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK_A::B_0X6)
    }
    #[doc = "OA2\\[7:1\\]
are masked and don't care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged."]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK_A::B_0X7)
    }
}
#[doc = "Field `OA2EN` reader - Own Address 2 enable"]
pub type OA2EN_R = crate::BitReader<OA2EN_A>;
#[doc = "Own Address 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OA2EN_A {
    #[doc = "0: Own address 2 disabled. The received slave address OA2 is NACKed."]
    B_0X0 = 0,
    #[doc = "1: Own address 2 enabled. The received slave address OA2 is ACKed."]
    B_0X1 = 1,
}
impl From<OA2EN_A> for bool {
    #[inline(always)]
    fn from(variant: OA2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl OA2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OA2EN_A {
        match self.bits {
            false => OA2EN_A::B_0X0,
            true => OA2EN_A::B_0X1,
        }
    }
    #[doc = "Own address 2 disabled. The received slave address OA2 is NACKed."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OA2EN_A::B_0X0
    }
    #[doc = "Own address 2 enabled. The received slave address OA2 is ACKed."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OA2EN_A::B_0X1
    }
}
#[doc = "Field `OA2EN` writer - Own Address 2 enable"]
pub type OA2EN_W<'a, REG> = crate::BitWriter<'a, REG, OA2EN_A>;
impl<'a, REG> OA2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Own address 2 disabled. The received slave address OA2 is NACKed."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OA2EN_A::B_0X0)
    }
    #[doc = "Own address 2 enabled. The received slave address OA2 is ACKed."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OA2EN_A::B_0X1)
    }
}
impl R {
    #[doc = "Bits 1:7 - Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN=0."]
    #[inline(always)]
    pub fn oa2(&self) -> OA2_R {
        OA2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches."]
    #[inline(always)]
    pub fn oa2msk(&self) -> OA2MSK_R {
        OA2MSK_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline(always)]
    pub fn oa2en(&self) -> OA2EN_R {
        OA2EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN=0."]
    #[inline(always)]
    #[must_use]
    pub fn oa2(&mut self) -> OA2_W<I2C_OAR2_SPEC> {
        OA2_W::new(self, 1)
    }
    #[doc = "Bits 8:10 - Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches."]
    #[inline(always)]
    #[must_use]
    pub fn oa2msk(&mut self) -> OA2MSK_W<I2C_OAR2_SPEC> {
        OA2MSK_W::new(self, 8)
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn oa2en(&mut self) -> OA2EN_W<I2C_OAR2_SPEC> {
        OA2EN_W::new(self, 15)
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
#[doc = "Own address register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_oar2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_oar2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_OAR2_SPEC;
impl crate::RegisterSpec for I2C_OAR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_oar2::R`](R) reader structure"]
impl crate::Readable for I2C_OAR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_oar2::W`](W) writer structure"]
impl crate::Writable for I2C_OAR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_OAR2 to value 0"]
impl crate::Resettable for I2C_OAR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
