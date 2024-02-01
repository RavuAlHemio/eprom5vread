#[doc = "Register `GTPR` reader"]
pub type R = crate::R<GTPR_SPEC>;
#[doc = "Register `GTPR` writer"]
pub type W = crate::W<GTPR_SPEC>;
#[doc = "Field `PSC` reader - Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]=Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... 00100000: Divides the source clock by 32 (IrDA mode) ... 11111111: Divides the source clock by 255 (IrDA mode) This bitfield can only be written when the USART is disabled (UE=0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0' when the Smartcard and IrDA modes are not supported. Refer to ."]
pub type PSC_R = crate::FieldReader<PSC_A>;
#[doc = "Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]=Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... 00100000: Divides the source clock by 32 (IrDA mode) ... 11111111: Divides the source clock by 255 (IrDA mode) This bitfield can only be written when the USART is disabled (UE=0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0' when the Smartcard and IrDA modes are not supported. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSC_A {
    #[doc = "0: Reserved - do not program this value"]
    B_0X0 = 0,
    #[doc = "1: Divides the source clock by 1 (IrDA mode) / by 2 (Smarcard mode)"]
    B_0X1 = 1,
    #[doc = "2: Divides the source clock by 2 (IrDA mode) / by 4 (Smartcard mode)"]
    B_0X2 = 2,
    #[doc = "3: Divides the source clock by 3 (IrDA mode) / by 6 (Smartcard mode)"]
    B_0X3 = 3,
    #[doc = "31: Divides the source clock by 31 (IrDA mode) / by 62 (Smartcard mode)"]
    B_0X1F = 31,
}
impl From<PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: PSC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PSC_A {
    type Ux = u8;
}
impl PSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PSC_A> {
        match self.bits {
            0 => Some(PSC_A::B_0X0),
            1 => Some(PSC_A::B_0X1),
            2 => Some(PSC_A::B_0X2),
            3 => Some(PSC_A::B_0X3),
            31 => Some(PSC_A::B_0X1F),
            _ => None,
        }
    }
    #[doc = "Reserved - do not program this value"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PSC_A::B_0X0
    }
    #[doc = "Divides the source clock by 1 (IrDA mode) / by 2 (Smarcard mode)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PSC_A::B_0X1
    }
    #[doc = "Divides the source clock by 2 (IrDA mode) / by 4 (Smartcard mode)"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PSC_A::B_0X2
    }
    #[doc = "Divides the source clock by 3 (IrDA mode) / by 6 (Smartcard mode)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PSC_A::B_0X3
    }
    #[doc = "Divides the source clock by 31 (IrDA mode) / by 62 (Smartcard mode)"]
    #[inline(always)]
    pub fn is_b_0x1f(&self) -> bool {
        *self == PSC_A::B_0X1F
    }
}
#[doc = "Field `PSC` writer - Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]=Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... 00100000: Divides the source clock by 32 (IrDA mode) ... 11111111: Divides the source clock by 255 (IrDA mode) This bitfield can only be written when the USART is disabled (UE=0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0' when the Smartcard and IrDA modes are not supported. Refer to ."]
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8, PSC_A>;
impl<'a, REG> PSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved - do not program this value"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::B_0X0)
    }
    #[doc = "Divides the source clock by 1 (IrDA mode) / by 2 (Smarcard mode)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::B_0X1)
    }
    #[doc = "Divides the source clock by 2 (IrDA mode) / by 4 (Smartcard mode)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::B_0X2)
    }
    #[doc = "Divides the source clock by 3 (IrDA mode) / by 6 (Smartcard mode)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::B_0X3)
    }
    #[doc = "Divides the source clock by 31 (IrDA mode) / by 62 (Smartcard mode)"]
    #[inline(always)]
    pub fn b_0x1f(self) -> &'a mut crate::W<REG> {
        self.variant(PSC_A::B_0X1F)
    }
}
#[doc = "Field `GT` reader - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE=0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type GT_R = crate::FieldReader;
#[doc = "Field `GT` writer - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE=0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type GT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]=Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... 00100000: Divides the source clock by 32 (IrDA mode) ... 11111111: Divides the source clock by 255 (IrDA mode) This bitfield can only be written when the USART is disabled (UE=0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0' when the Smartcard and IrDA modes are not supported. Refer to ."]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE=0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]=Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... 00100000: Divides the source clock by 32 (IrDA mode) ... 11111111: Divides the source clock by 255 (IrDA mode) This bitfield can only be written when the USART is disabled (UE=0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0' when the Smartcard and IrDA modes are not supported. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<GTPR_SPEC> {
        PSC_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE=0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn gt(&mut self) -> GT_W<GTPR_SPEC> {
        GT_W::new(self, 8)
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
#[doc = "Guard time and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTPR_SPEC;
impl crate::RegisterSpec for GTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtpr::R`](R) reader structure"]
impl crate::Readable for GTPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gtpr::W`](W) writer structure"]
impl crate::Writable for GTPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTPR to value 0"]
impl crate::Resettable for GTPR_SPEC {
    const RESET_VALUE: u32 = 0;
}
