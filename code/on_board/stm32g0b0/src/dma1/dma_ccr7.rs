#[doc = "Register `DMA_CCR7` reader"]
pub type R = crate::R<DMA_CCR7_SPEC>;
#[doc = "Register `DMA_CCR7` writer"]
pub type W = crate::W<DMA_CCR7_SPEC>;
#[doc = "Field `EN` reader - channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the DMA_ISR register is cleared (by setting the CTEIFx bit of the DMA_IFCR register). Note: this bit is set and cleared by software."]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the DMA_ISR register is cleared (by setting the CTEIFx bit of the DMA_IFCR register). Note: this bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: disabled"]
    B_0X0 = 0,
    #[doc = "1: enabled"]
    B_0X1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::B_0X0,
            true => EN_A::B_0X1,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EN_A::B_0X0
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EN_A::B_0X1
    }
}
#[doc = "Field `EN` writer - channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the DMA_ISR register is cleared (by setting the CTEIFx bit of the DMA_IFCR register). Note: this bit is set and cleared by software."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN_A>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::B_0X0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::B_0X1)
    }
}
#[doc = "Field `TCIE` reader - transfer complete interrupt enable Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
pub type TCIE_R = crate::BitReader<TCIE_A>;
#[doc = "transfer complete interrupt enable Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE_A {
    #[doc = "0: disabled"]
    B_0X0 = 0,
    #[doc = "1: enabled"]
    B_0X1 = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::B_0X0,
            true => TCIE_A::B_0X1,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIE_A::B_0X0
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIE_A::B_0X1
    }
}
#[doc = "Field `TCIE` writer - transfer complete interrupt enable Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE_A>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE_A::B_0X0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE_A::B_0X1)
    }
}
#[doc = "Field `HTIE` reader - half transfer interrupt enable Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
pub type HTIE_R = crate::BitReader<HTIE_A>;
#[doc = "half transfer interrupt enable Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIE_A {
    #[doc = "0: disabled"]
    B_0X0 = 0,
    #[doc = "1: enabled"]
    B_0X1 = 1,
}
impl From<HTIE_A> for bool {
    #[inline(always)]
    fn from(variant: HTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl HTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HTIE_A {
        match self.bits {
            false => HTIE_A::B_0X0,
            true => HTIE_A::B_0X1,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HTIE_A::B_0X0
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HTIE_A::B_0X1
    }
}
#[doc = "Field `HTIE` writer - half transfer interrupt enable Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
pub type HTIE_W<'a, REG> = crate::BitWriter<'a, REG, HTIE_A>;
impl<'a, REG> HTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HTIE_A::B_0X0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HTIE_A::B_0X1)
    }
}
#[doc = "Field `TEIE` reader - transfer error interrupt enable Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
pub type TEIE_R = crate::BitReader<TEIE_A>;
#[doc = "transfer error interrupt enable Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE_A {
    #[doc = "0: disabled"]
    B_0X0 = 0,
    #[doc = "1: enabled"]
    B_0X1 = 1,
}
impl From<TEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEIE_A {
        match self.bits {
            false => TEIE_A::B_0X0,
            true => TEIE_A::B_0X1,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIE_A::B_0X0
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIE_A::B_0X1
    }
}
#[doc = "Field `TEIE` writer - transfer error interrupt enable Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG, TEIE_A>;
impl<'a, REG> TEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE_A::B_0X0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE_A::B_0X1)
    }
}
#[doc = "Field `DIR` reader - data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. Source attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Destination attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
pub type DIR_R = crate::BitReader<DIR_A>;
#[doc = "data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. Source attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Destination attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: read from peripheral"]
    B_0X0 = 0,
    #[doc = "1: read from memory"]
    B_0X1 = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::B_0X0,
            true => DIR_A::B_0X1,
        }
    }
    #[doc = "read from peripheral"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DIR_A::B_0X0
    }
    #[doc = "read from memory"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DIR_A::B_0X1
    }
}
#[doc = "Field `DIR` writer - data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. Source attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Destination attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG, DIR_A>;
impl<'a, REG> DIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "read from peripheral"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DIR_A::B_0X0)
    }
    #[doc = "read from memory"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DIR_A::B_0X1)
    }
}
#[doc = "Field `CIRC` reader - circular mode Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
pub type CIRC_R = crate::BitReader<CIRC_A>;
#[doc = "circular mode Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIRC_A {
    #[doc = "0: disabled"]
    B_0X0 = 0,
    #[doc = "1: enabled"]
    B_0X1 = 1,
}
impl From<CIRC_A> for bool {
    #[inline(always)]
    fn from(variant: CIRC_A) -> Self {
        variant as u8 != 0
    }
}
impl CIRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CIRC_A {
        match self.bits {
            false => CIRC_A::B_0X0,
            true => CIRC_A::B_0X1,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CIRC_A::B_0X0
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CIRC_A::B_0X1
    }
}
#[doc = "Field `CIRC` writer - circular mode Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
pub type CIRC_W<'a, REG> = crate::BitWriter<'a, REG, CIRC_A>;
impl<'a, REG> CIRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CIRC_A::B_0X0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CIRC_A::B_0X1)
    }
}
#[doc = "Field `PINC` reader - peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this field identifies the memory destination if DIR=1 and the memory source if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR=1 and the peripheral source if DIR=0. Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
pub type PINC_R = crate::BitReader<PINC_A>;
#[doc = "peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this field identifies the memory destination if DIR=1 and the memory source if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR=1 and the peripheral source if DIR=0. Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINC_A {
    #[doc = "0: disabled"]
    B_0X0 = 0,
    #[doc = "1: enabled"]
    B_0X1 = 1,
}
impl From<PINC_A> for bool {
    #[inline(always)]
    fn from(variant: PINC_A) -> Self {
        variant as u8 != 0
    }
}
impl PINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PINC_A {
        match self.bits {
            false => PINC_A::B_0X0,
            true => PINC_A::B_0X1,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PINC_A::B_0X0
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PINC_A::B_0X1
    }
}
#[doc = "Field `PINC` writer - peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this field identifies the memory destination if DIR=1 and the memory source if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR=1 and the peripheral source if DIR=0. Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
pub type PINC_W<'a, REG> = crate::BitWriter<'a, REG, PINC_A>;
impl<'a, REG> PINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PINC_A::B_0X0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PINC_A::B_0X1)
    }
}
#[doc = "Field `MINC` reader - memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR=1 and the memory destination if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR=1 and the peripheral destination if DIR=0. Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
pub type MINC_R = crate::BitReader<MINC_A>;
#[doc = "memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR=1 and the memory destination if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR=1 and the peripheral destination if DIR=0. Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINC_A {
    #[doc = "0: disabled"]
    B_0X0 = 0,
    #[doc = "1: enabled"]
    B_0X1 = 1,
}
impl From<MINC_A> for bool {
    #[inline(always)]
    fn from(variant: MINC_A) -> Self {
        variant as u8 != 0
    }
}
impl MINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MINC_A {
        match self.bits {
            false => MINC_A::B_0X0,
            true => MINC_A::B_0X1,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MINC_A::B_0X0
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MINC_A::B_0X1
    }
}
#[doc = "Field `MINC` writer - memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR=1 and the memory destination if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR=1 and the peripheral destination if DIR=0. Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
pub type MINC_W<'a, REG> = crate::BitWriter<'a, REG, MINC_A>;
impl<'a, REG> MINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MINC_A::B_0X0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MINC_A::B_0X1)
    }
}
#[doc = "Field `PSIZE` reader - peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this field identifies the memory destination if DIR=1 and the memory source if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR=1 and the peripheral source if DIR=0. Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
pub type PSIZE_R = crate::FieldReader<PSIZE_A>;
#[doc = "peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this field identifies the memory destination if DIR=1 and the memory source if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR=1 and the peripheral source if DIR=0. Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSIZE_A {
    #[doc = "0: 8 bits"]
    B_0X0 = 0,
    #[doc = "1: 16 bits"]
    B_0X1 = 1,
    #[doc = "2: 32 bits"]
    B_0X2 = 2,
}
impl From<PSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PSIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PSIZE_A {
    type Ux = u8;
}
impl PSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PSIZE_A> {
        match self.bits {
            0 => Some(PSIZE_A::B_0X0),
            1 => Some(PSIZE_A::B_0X1),
            2 => Some(PSIZE_A::B_0X2),
            _ => None,
        }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PSIZE_A::B_0X0
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PSIZE_A::B_0X1
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PSIZE_A::B_0X2
    }
}
#[doc = "Field `PSIZE` writer - peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this field identifies the memory destination if DIR=1 and the memory source if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR=1 and the peripheral source if DIR=0. Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PSIZE_A>;
impl<'a, REG> PSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE_A::B_0X0)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE_A::B_0X1)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE_A::B_0X2)
    }
}
#[doc = "Field `MSIZE` reader - memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR=1 and the memory destination if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR=1 and the peripheral destination if DIR=0. Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
pub type MSIZE_R = crate::FieldReader<MSIZE_A>;
#[doc = "memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR=1 and the memory destination if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR=1 and the peripheral destination if DIR=0. Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSIZE_A {
    #[doc = "0: 8 bits"]
    B_0X0 = 0,
    #[doc = "1: 16 bits"]
    B_0X1 = 1,
    #[doc = "2: 32 bits"]
    B_0X2 = 2,
}
impl From<MSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: MSIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MSIZE_A {
    type Ux = u8;
}
impl MSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MSIZE_A> {
        match self.bits {
            0 => Some(MSIZE_A::B_0X0),
            1 => Some(MSIZE_A::B_0X1),
            2 => Some(MSIZE_A::B_0X2),
            _ => None,
        }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MSIZE_A::B_0X0
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MSIZE_A::B_0X1
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MSIZE_A::B_0X2
    }
}
#[doc = "Field `MSIZE` writer - memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR=1 and the memory destination if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR=1 and the peripheral destination if DIR=0. Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
pub type MSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MSIZE_A>;
impl<'a, REG> MSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSIZE_A::B_0X0)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSIZE_A::B_0X1)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MSIZE_A::B_0X2)
    }
}
#[doc = "Field `PL` reader - priority level Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
pub type PL_R = crate::FieldReader<PL_A>;
#[doc = "priority level Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PL_A {
    #[doc = "0: low"]
    B_0X0 = 0,
    #[doc = "1: medium"]
    B_0X1 = 1,
    #[doc = "2: high"]
    B_0X2 = 2,
    #[doc = "3: very high"]
    B_0X3 = 3,
}
impl From<PL_A> for u8 {
    #[inline(always)]
    fn from(variant: PL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PL_A {
    type Ux = u8;
}
impl PL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PL_A {
        match self.bits {
            0 => PL_A::B_0X0,
            1 => PL_A::B_0X1,
            2 => PL_A::B_0X2,
            3 => PL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "low"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PL_A::B_0X0
    }
    #[doc = "medium"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PL_A::B_0X1
    }
    #[doc = "high"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PL_A::B_0X2
    }
    #[doc = "very high"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PL_A::B_0X3
    }
}
#[doc = "Field `PL` writer - priority level Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
pub type PL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PL_A>;
impl<'a, REG> PL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "low"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PL_A::B_0X0)
    }
    #[doc = "medium"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PL_A::B_0X1)
    }
    #[doc = "high"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PL_A::B_0X2)
    }
    #[doc = "very high"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PL_A::B_0X3)
    }
}
#[doc = "Field `MEM2MEM` reader - memory-to-memory mode Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
pub type MEM2MEM_R = crate::BitReader<MEM2MEM_A>;
#[doc = "memory-to-memory mode Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEM2MEM_A {
    #[doc = "0: disabled"]
    B_0X0 = 0,
    #[doc = "1: enabled"]
    B_0X1 = 1,
}
impl From<MEM2MEM_A> for bool {
    #[inline(always)]
    fn from(variant: MEM2MEM_A) -> Self {
        variant as u8 != 0
    }
}
impl MEM2MEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MEM2MEM_A {
        match self.bits {
            false => MEM2MEM_A::B_0X0,
            true => MEM2MEM_A::B_0X1,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MEM2MEM_A::B_0X0
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MEM2MEM_A::B_0X1
    }
}
#[doc = "Field `MEM2MEM` writer - memory-to-memory mode Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
pub type MEM2MEM_W<'a, REG> = crate::BitWriter<'a, REG, MEM2MEM_A>;
impl<'a, REG> MEM2MEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MEM2MEM_A::B_0X0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MEM2MEM_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the DMA_ISR register is cleared (by setting the CTEIFx bit of the DMA_IFCR register). Note: this bit is set and cleared by software."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - transfer complete interrupt enable Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - half transfer interrupt enable Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - transfer error interrupt enable Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. Source attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Destination attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - circular mode Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this field identifies the memory destination if DIR=1 and the memory source if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR=1 and the peripheral source if DIR=0. Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR=1 and the memory destination if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR=1 and the peripheral destination if DIR=0. Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this field identifies the memory destination if DIR=1 and the memory source if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR=1 and the peripheral source if DIR=0. Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR=1 and the memory destination if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR=1 and the peripheral destination if DIR=0. Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - priority level Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - memory-to-memory mode Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    pub fn mem2mem(&self) -> MEM2MEM_R {
        MEM2MEM_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the DMA_ISR register is cleared (by setting the CTEIFx bit of the DMA_IFCR register). Note: this bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<DMA_CCR7_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - transfer complete interrupt enable Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<DMA_CCR7_SPEC> {
        TCIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - half transfer interrupt enable Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HTIE_W<DMA_CCR7_SPEC> {
        HTIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - transfer error interrupt enable Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<DMA_CCR7_SPEC> {
        TEIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. Source attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Destination attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<DMA_CCR7_SPEC> {
        DIR_W::new(self, 4)
    }
    #[doc = "Bit 5 - circular mode Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    #[must_use]
    pub fn circ(&mut self) -> CIRC_W<DMA_CCR7_SPEC> {
        CIRC_W::new(self, 5)
    }
    #[doc = "Bit 6 - peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this field identifies the memory destination if DIR=1 and the memory source if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR=1 and the peripheral source if DIR=0. Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    #[must_use]
    pub fn pinc(&mut self) -> PINC_W<DMA_CCR7_SPEC> {
        PINC_W::new(self, 6)
    }
    #[doc = "Bit 7 - memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR=1 and the memory destination if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR=1 and the peripheral destination if DIR=0. Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    #[must_use]
    pub fn minc(&mut self) -> MINC_W<DMA_CCR7_SPEC> {
        MINC_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this field identifies the memory destination if DIR=1 and the memory source if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral destination if DIR=1 and the peripheral source if DIR=0. Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<DMA_CCR7_SPEC> {
        PSIZE_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this field identifies the memory source if DIR=1 and the memory destination if DIR=0. In peripheral-to-peripheral mode, this field identifies the peripheral source if DIR=1 and the peripheral destination if DIR=0. Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    #[must_use]
    pub fn msize(&mut self) -> MSIZE_W<DMA_CCR7_SPEC> {
        MSIZE_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - priority level Note: this field is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<DMA_CCR7_SPEC> {
        PL_W::new(self, 12)
    }
    #[doc = "Bit 14 - memory-to-memory mode Note: this bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN=1)."]
    #[inline(always)]
    #[must_use]
    pub fn mem2mem(&mut self) -> MEM2MEM_W<DMA_CCR7_SPEC> {
        MEM2MEM_W::new(self, 14)
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
#[doc = "DMA channel 7 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ccr7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_ccr7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_CCR7_SPEC;
impl crate::RegisterSpec for DMA_CCR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_ccr7::R`](R) reader structure"]
impl crate::Readable for DMA_CCR7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_ccr7::W`](W) writer structure"]
impl crate::Writable for DMA_CCR7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_CCR7 to value 0"]
impl crate::Resettable for DMA_CCR7_SPEC {
    const RESET_VALUE: u32 = 0;
}
