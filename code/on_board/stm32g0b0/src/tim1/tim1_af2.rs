#[doc = "Register `TIM1_AF2` reader"]
pub type R = crate::R<TIM1_AF2_SPEC>;
#[doc = "Register `TIM1_AF2` writer"]
pub type W = crate::W<TIM1_AF2_SPEC>;
#[doc = "Field `BK2INE` reader - BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timer's BRK2 input. BKIN2 input is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2INE_R = crate::BitReader<BK2INE_A>;
#[doc = "BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timer's BRK2 input. BKIN2 input is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2INE_A {
    #[doc = "0: BKIN2 input disabled"]
    B_0X0 = 0,
    #[doc = "1: BKIN2 input enabled"]
    B_0X1 = 1,
}
impl From<BK2INE_A> for bool {
    #[inline(always)]
    fn from(variant: BK2INE_A) -> Self {
        variant as u8 != 0
    }
}
impl BK2INE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2INE_A {
        match self.bits {
            false => BK2INE_A::B_0X0,
            true => BK2INE_A::B_0X1,
        }
    }
    #[doc = "BKIN2 input disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BK2INE_A::B_0X0
    }
    #[doc = "BKIN2 input enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BK2INE_A::B_0X1
    }
}
#[doc = "Field `BK2INE` writer - BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timer's BRK2 input. BKIN2 input is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2INE_W<'a, REG> = crate::BitWriter<'a, REG, BK2INE_A>;
impl<'a, REG> BK2INE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BKIN2 input disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INE_A::B_0X0)
    }
    #[doc = "BKIN2 input enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INE_A::B_0X1)
    }
}
#[doc = "Field `BK2CMP1E` reader - BRK2 COMP1 enable This bit enables the COMP1 for the timer's BRK2 input. COMP1 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP1E_R = crate::BitReader<BK2CMP1E_A>;
#[doc = "BRK2 COMP1 enable This bit enables the COMP1 for the timer's BRK2 input. COMP1 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP1E_A {
    #[doc = "0: COMP1 input disabled"]
    B_0X0 = 0,
    #[doc = "1: COMP1 input enabled"]
    B_0X1 = 1,
}
impl From<BK2CMP1E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP1E_A) -> Self {
        variant as u8 != 0
    }
}
impl BK2CMP1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP1E_A {
        match self.bits {
            false => BK2CMP1E_A::B_0X0,
            true => BK2CMP1E_A::B_0X1,
        }
    }
    #[doc = "COMP1 input disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BK2CMP1E_A::B_0X0
    }
    #[doc = "COMP1 input enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BK2CMP1E_A::B_0X1
    }
}
#[doc = "Field `BK2CMP1E` writer - BRK2 COMP1 enable This bit enables the COMP1 for the timer's BRK2 input. COMP1 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP1E_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP1E_A>;
impl<'a, REG> BK2CMP1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP1 input disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1E_A::B_0X0)
    }
    #[doc = "COMP1 input enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1E_A::B_0X1)
    }
}
#[doc = "Field `BK2CMP2E` reader - BRK2 COMP2 enable This bit enables the COMP2 for the timer's BRK2 input. COMP2 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP2E_R = crate::BitReader<BK2CMP2E_A>;
#[doc = "BRK2 COMP2 enable This bit enables the COMP2 for the timer's BRK2 input. COMP2 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP2E_A {
    #[doc = "0: COMP2 input disabled"]
    B_0X0 = 0,
    #[doc = "1: COMP2 input enabled"]
    B_0X1 = 1,
}
impl From<BK2CMP2E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP2E_A) -> Self {
        variant as u8 != 0
    }
}
impl BK2CMP2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP2E_A {
        match self.bits {
            false => BK2CMP2E_A::B_0X0,
            true => BK2CMP2E_A::B_0X1,
        }
    }
    #[doc = "COMP2 input disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BK2CMP2E_A::B_0X0
    }
    #[doc = "COMP2 input enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BK2CMP2E_A::B_0X1
    }
}
#[doc = "Field `BK2CMP2E` writer - BRK2 COMP2 enable This bit enables the COMP2 for the timer's BRK2 input. COMP2 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP2E_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP2E_A>;
impl<'a, REG> BK2CMP2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP2 input disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP2E_A::B_0X0)
    }
    #[doc = "COMP2 input enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP2E_A::B_0X1)
    }
}
#[doc = "Field `BK2INP` reader - BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2INP_R = crate::BitReader<BK2INP_A>;
#[doc = "BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2INP_A {
    #[doc = "0: BKIN2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    B_0X0 = 0,
    #[doc = "1: BKIN2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    B_0X1 = 1,
}
impl From<BK2INP_A> for bool {
    #[inline(always)]
    fn from(variant: BK2INP_A) -> Self {
        variant as u8 != 0
    }
}
impl BK2INP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2INP_A {
        match self.bits {
            false => BK2INP_A::B_0X0,
            true => BK2INP_A::B_0X1,
        }
    }
    #[doc = "BKIN2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BK2INP_A::B_0X0
    }
    #[doc = "BKIN2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BK2INP_A::B_0X1
    }
}
#[doc = "Field `BK2INP` writer - BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2INP_W<'a, REG> = crate::BitWriter<'a, REG, BK2INP_A>;
impl<'a, REG> BK2INP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BKIN2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INP_A::B_0X0)
    }
    #[doc = "BKIN2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2INP_A::B_0X1)
    }
}
#[doc = "Field `BK2CMP1P` reader - BRK2 COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP1P_R = crate::BitReader<BK2CMP1P_A>;
#[doc = "BRK2 COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP1P_A {
    #[doc = "0: COMP1 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    B_0X0 = 0,
    #[doc = "1: COMP1 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    B_0X1 = 1,
}
impl From<BK2CMP1P_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP1P_A) -> Self {
        variant as u8 != 0
    }
}
impl BK2CMP1P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP1P_A {
        match self.bits {
            false => BK2CMP1P_A::B_0X0,
            true => BK2CMP1P_A::B_0X1,
        }
    }
    #[doc = "COMP1 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BK2CMP1P_A::B_0X0
    }
    #[doc = "COMP1 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BK2CMP1P_A::B_0X1
    }
}
#[doc = "Field `BK2CMP1P` writer - BRK2 COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP1P_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP1P_A>;
impl<'a, REG> BK2CMP1P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP1 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1P_A::B_0X0)
    }
    #[doc = "COMP1 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP1P_A::B_0X1)
    }
}
#[doc = "Field `BK2CMP2P` reader - BRK2 COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP2P_R = crate::BitReader<BK2CMP2P_A>;
#[doc = "BRK2 COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2CMP2P_A {
    #[doc = "0: COMP2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    B_0X0 = 0,
    #[doc = "1: COMP2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    B_0X1 = 1,
}
impl From<BK2CMP2P_A> for bool {
    #[inline(always)]
    fn from(variant: BK2CMP2P_A) -> Self {
        variant as u8 != 0
    }
}
impl BK2CMP2P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2CMP2P_A {
        match self.bits {
            false => BK2CMP2P_A::B_0X0,
            true => BK2CMP2P_A::B_0X1,
        }
    }
    #[doc = "COMP2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BK2CMP2P_A::B_0X0
    }
    #[doc = "COMP2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BK2CMP2P_A::B_0X1
    }
}
#[doc = "Field `BK2CMP2P` writer - BRK2 COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2CMP2P_W<'a, REG> = crate::BitWriter<'a, REG, BK2CMP2P_A>;
impl<'a, REG> BK2CMP2P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP2P_A::B_0X0)
    }
    #[doc = "COMP2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2CMP2P_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timer's BRK2 input. BKIN2 input is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable This bit enables the COMP1 for the timer's BRK2 input. COMP1 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> BK2CMP1E_R {
        BK2CMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable This bit enables the COMP2 for the timer's BRK2 input. COMP2 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> BK2CMP2E_R {
        BK2CMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> BK2CMP1P_R {
        BK2CMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> BK2CMP2P_R {
        BK2CMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timer's BRK2 input. BKIN2 input is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bk2ine(&mut self) -> BK2INE_W<TIM1_AF2_SPEC> {
        BK2INE_W::new(self, 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable This bit enables the COMP1 for the timer's BRK2 input. COMP1 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp1e(&mut self) -> BK2CMP1E_W<TIM1_AF2_SPEC> {
        BK2CMP1E_W::new(self, 1)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable This bit enables the COMP2 for the timer's BRK2 input. COMP2 output is 'ORed' with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp2e(&mut self) -> BK2CMP2E_W<TIM1_AF2_SPEC> {
        BK2CMP2E_W::new(self, 2)
    }
    #[doc = "Bit 9 - BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bk2inp(&mut self) -> BK2INP_W<TIM1_AF2_SPEC> {
        BK2INP_W::new(self, 9)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp1p(&mut self) -> BK2CMP1P_W<TIM1_AF2_SPEC> {
        BK2CMP1P_W::new(self, 10)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bk2cmp2p(&mut self) -> BK2CMP2P_W<TIM1_AF2_SPEC> {
        BK2CMP2P_W::new(self, 11)
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
#[doc = "DMA address for full transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_af2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_af2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_AF2_SPEC;
impl crate::RegisterSpec for TIM1_AF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_af2::R`](R) reader structure"]
impl crate::Readable for TIM1_AF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tim1_af2::W`](W) writer structure"]
impl crate::Writable for TIM1_AF2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_AF2 to value 0x01"]
impl crate::Resettable for TIM1_AF2_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
