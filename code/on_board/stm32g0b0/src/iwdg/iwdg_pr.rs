#[doc = "Register `IWDG_PR` reader"]
pub type R = crate::R<IWDG_PR_SPEC>;
#[doc = "Register `IWDG_PR` writer"]
pub type W = crate::W<IWDG_PR_SPEC>;
#[doc = "Field `PR` reader - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
pub type PR_R = crate::FieldReader<PR_A>;
#[doc = "Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PR_A {
    #[doc = "0: divider /4"]
    B_0X0 = 0,
    #[doc = "1: divider /8"]
    B_0X1 = 1,
    #[doc = "2: divider /16"]
    B_0X2 = 2,
    #[doc = "3: divider /32"]
    B_0X3 = 3,
    #[doc = "4: divider /64"]
    B_0X4 = 4,
    #[doc = "5: divider /128"]
    B_0X5 = 5,
    #[doc = "6: divider /256"]
    B_0X6 = 6,
    #[doc = "7: divider /256"]
    B_0X7 = 7,
}
impl From<PR_A> for u8 {
    #[inline(always)]
    fn from(variant: PR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PR_A {
    type Ux = u8;
}
impl PR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PR_A {
        match self.bits {
            0 => PR_A::B_0X0,
            1 => PR_A::B_0X1,
            2 => PR_A::B_0X2,
            3 => PR_A::B_0X3,
            4 => PR_A::B_0X4,
            5 => PR_A::B_0X5,
            6 => PR_A::B_0X6,
            7 => PR_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "divider /4"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PR_A::B_0X0
    }
    #[doc = "divider /8"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PR_A::B_0X1
    }
    #[doc = "divider /16"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PR_A::B_0X2
    }
    #[doc = "divider /32"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PR_A::B_0X3
    }
    #[doc = "divider /64"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == PR_A::B_0X4
    }
    #[doc = "divider /128"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == PR_A::B_0X5
    }
    #[doc = "divider /256"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == PR_A::B_0X6
    }
    #[doc = "divider /256"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == PR_A::B_0X7
    }
}
#[doc = "Field `PR` writer - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
pub type PR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, PR_A>;
impl<'a, REG> PR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "divider /4"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::B_0X0)
    }
    #[doc = "divider /8"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::B_0X1)
    }
    #[doc = "divider /16"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::B_0X2)
    }
    #[doc = "divider /32"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::B_0X3)
    }
    #[doc = "divider /64"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::B_0X4)
    }
    #[doc = "divider /128"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::B_0X5)
    }
    #[doc = "divider /256"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::B_0X6)
    }
    #[doc = "divider /256"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::B_0X7)
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<IWDG_PR_SPEC> {
        PR_W::new(self, 0)
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
#[doc = "Prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_pr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_pr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IWDG_PR_SPEC;
impl crate::RegisterSpec for IWDG_PR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_pr::R`](R) reader structure"]
impl crate::Readable for IWDG_PR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iwdg_pr::W`](W) writer structure"]
impl crate::Writable for IWDG_PR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IWDG_PR to value 0"]
impl crate::Resettable for IWDG_PR_SPEC {
    const RESET_VALUE: u32 = 0;
}
