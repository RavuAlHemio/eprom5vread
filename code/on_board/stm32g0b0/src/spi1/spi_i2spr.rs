#[doc = "Register `SPI_I2SPR` reader"]
pub type R = crate::R<SPI_I2SPR_SPEC>;
#[doc = "Register `SPI_I2SPR` writer"]
pub type W = crate::W<SPI_I2SPR_SPEC>;
#[doc = "Field `I2SDIV` reader - I2S linear prescaler I2SDIV \\[7:0\\]
= 0 or I2SDIV \\[7:0\\]
= 1 are forbidden values. Refer to . Note: These bits should be configured when the I2S is disabled. They are used only when the I2S is in master mode. They are not used in SPI mode."]
pub type I2SDIV_R = crate::FieldReader;
#[doc = "Field `I2SDIV` writer - I2S linear prescaler I2SDIV \\[7:0\\]
= 0 or I2SDIV \\[7:0\\]
= 1 are forbidden values. Refer to . Note: These bits should be configured when the I2S is disabled. They are used only when the I2S is in master mode. They are not used in SPI mode."]
pub type I2SDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ODD` reader - Odd factor for the prescaler Refer to . Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode."]
pub type ODD_R = crate::BitReader<ODD_A>;
#[doc = "Odd factor for the prescaler Refer to . Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODD_A {
    #[doc = "0: Real divider value is = I2SDIV *2"]
    B_0X0 = 0,
    #[doc = "1: Real divider value is = (I2SDIV * 2)+1"]
    B_0X1 = 1,
}
impl From<ODD_A> for bool {
    #[inline(always)]
    fn from(variant: ODD_A) -> Self {
        variant as u8 != 0
    }
}
impl ODD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ODD_A {
        match self.bits {
            false => ODD_A::B_0X0,
            true => ODD_A::B_0X1,
        }
    }
    #[doc = "Real divider value is = I2SDIV *2"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ODD_A::B_0X0
    }
    #[doc = "Real divider value is = (I2SDIV * 2)+1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ODD_A::B_0X1
    }
}
#[doc = "Field `ODD` writer - Odd factor for the prescaler Refer to . Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode."]
pub type ODD_W<'a, REG> = crate::BitWriter<'a, REG, ODD_A>;
impl<'a, REG> ODD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Real divider value is = I2SDIV *2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ODD_A::B_0X0)
    }
    #[doc = "Real divider value is = (I2SDIV * 2)+1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ODD_A::B_0X1)
    }
}
#[doc = "Field `MCKOE` reader - Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode."]
pub type MCKOE_R = crate::BitReader<MCKOE_A>;
#[doc = "Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCKOE_A {
    #[doc = "0: Master clock output is disabled"]
    B_0X0 = 0,
    #[doc = "1: Master clock output is enabled"]
    B_0X1 = 1,
}
impl From<MCKOE_A> for bool {
    #[inline(always)]
    fn from(variant: MCKOE_A) -> Self {
        variant as u8 != 0
    }
}
impl MCKOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCKOE_A {
        match self.bits {
            false => MCKOE_A::B_0X0,
            true => MCKOE_A::B_0X1,
        }
    }
    #[doc = "Master clock output is disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCKOE_A::B_0X0
    }
    #[doc = "Master clock output is enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCKOE_A::B_0X1
    }
}
#[doc = "Field `MCKOE` writer - Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode."]
pub type MCKOE_W<'a, REG> = crate::BitWriter<'a, REG, MCKOE_A>;
impl<'a, REG> MCKOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master clock output is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MCKOE_A::B_0X0)
    }
    #[doc = "Master clock output is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MCKOE_A::B_0X1)
    }
}
impl R {
    #[doc = "Bits 0:7 - I2S linear prescaler I2SDIV \\[7:0\\]
= 0 or I2SDIV \\[7:0\\]
= 1 are forbidden values. Refer to . Note: These bits should be configured when the I2S is disabled. They are used only when the I2S is in master mode. They are not used in SPI mode."]
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler Refer to . Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode."]
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode."]
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2S linear prescaler I2SDIV \\[7:0\\]
= 0 or I2SDIV \\[7:0\\]
= 1 are forbidden values. Refer to . Note: These bits should be configured when the I2S is disabled. They are used only when the I2S is in master mode. They are not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2sdiv(&mut self) -> I2SDIV_W<SPI_I2SPR_SPEC> {
        I2SDIV_W::new(self, 0)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler Refer to . Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn odd(&mut self) -> ODD_W<SPI_I2SPR_SPEC> {
        ODD_W::new(self, 8)
    }
    #[doc = "Bit 9 - Master clock output enable Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode. It is not used in SPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn mckoe(&mut self) -> MCKOE_W<SPI_I2SPR_SPEC> {
        MCKOE_W::new(self, 9)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_i2spr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_i2spr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_I2SPR_SPEC;
impl crate::RegisterSpec for SPI_I2SPR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spi_i2spr::R`](R) reader structure"]
impl crate::Readable for SPI_I2SPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_i2spr::W`](W) writer structure"]
impl crate::Writable for SPI_I2SPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SPI_I2SPR to value 0x02"]
impl crate::Resettable for SPI_I2SPR_SPEC {
    const RESET_VALUE: u16 = 0x02;
}
