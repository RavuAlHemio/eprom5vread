#[doc = "Register `DMA_ISR` reader"]
pub type R = crate::R<DMA_ISR_SPEC>;
#[doc = "Field `GIF1` reader - global interrupt flag for channel 1"]
pub type GIF1_R = crate::BitReader<GIF1_A>;
#[doc = "global interrupt flag for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF1_A {
    #[doc = "0: no TE, HT or TC event"]
    B_0X0 = 0,
    #[doc = "1: a TE, HT or TC event occurred"]
    B_0X1 = 1,
}
impl From<GIF1_A> for bool {
    #[inline(always)]
    fn from(variant: GIF1_A) -> Self {
        variant as u8 != 0
    }
}
impl GIF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GIF1_A {
        match self.bits {
            false => GIF1_A::B_0X0,
            true => GIF1_A::B_0X1,
        }
    }
    #[doc = "no TE, HT or TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF1_A::B_0X0
    }
    #[doc = "a TE, HT or TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF1_A::B_0X1
    }
}
#[doc = "Field `TCIF1` reader - transfer complete (TC) flag for channel 1"]
pub type TCIF1_R = crate::BitReader<TCIF1_A>;
#[doc = "transfer complete (TC) flag for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF1_A {
    #[doc = "0: no TC event"]
    B_0X0 = 0,
    #[doc = "1: a TC event occurred"]
    B_0X1 = 1,
}
impl From<TCIF1_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF1_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIF1_A {
        match self.bits {
            false => TCIF1_A::B_0X0,
            true => TCIF1_A::B_0X1,
        }
    }
    #[doc = "no TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF1_A::B_0X0
    }
    #[doc = "a TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF1_A::B_0X1
    }
}
#[doc = "Field `HTIF1` reader - half transfer (HT) flag for channel 1"]
pub type HTIF1_R = crate::BitReader<HTIF1_A>;
#[doc = "half transfer (HT) flag for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF1_A {
    #[doc = "0: no HT event"]
    B_0X0 = 0,
    #[doc = "1: a HT event occurred"]
    B_0X1 = 1,
}
impl From<HTIF1_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF1_A) -> Self {
        variant as u8 != 0
    }
}
impl HTIF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HTIF1_A {
        match self.bits {
            false => HTIF1_A::B_0X0,
            true => HTIF1_A::B_0X1,
        }
    }
    #[doc = "no HT event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HTIF1_A::B_0X0
    }
    #[doc = "a HT event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HTIF1_A::B_0X1
    }
}
#[doc = "Field `TEIF1` reader - transfer error (TE) flag for channel 1"]
pub type TEIF1_R = crate::BitReader<TEIF1_A>;
#[doc = "transfer error (TE) flag for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF1_A {
    #[doc = "0: no TE event"]
    B_0X0 = 0,
    #[doc = "1: a TE event occurred"]
    B_0X1 = 1,
}
impl From<TEIF1_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF1_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEIF1_A {
        match self.bits {
            false => TEIF1_A::B_0X0,
            true => TEIF1_A::B_0X1,
        }
    }
    #[doc = "no TE event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF1_A::B_0X0
    }
    #[doc = "a TE event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF1_A::B_0X1
    }
}
#[doc = "Field `GIF2` reader - global interrupt flag for channel 2"]
pub type GIF2_R = crate::BitReader<GIF2_A>;
#[doc = "global interrupt flag for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF2_A {
    #[doc = "0: no TE, HT or TC event"]
    B_0X0 = 0,
    #[doc = "1: a TE, HT or TC event occurred"]
    B_0X1 = 1,
}
impl From<GIF2_A> for bool {
    #[inline(always)]
    fn from(variant: GIF2_A) -> Self {
        variant as u8 != 0
    }
}
impl GIF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GIF2_A {
        match self.bits {
            false => GIF2_A::B_0X0,
            true => GIF2_A::B_0X1,
        }
    }
    #[doc = "no TE, HT or TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF2_A::B_0X0
    }
    #[doc = "a TE, HT or TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF2_A::B_0X1
    }
}
#[doc = "Field `TCIF2` reader - transfer complete (TC) flag for channel 2"]
pub type TCIF2_R = crate::BitReader<TCIF2_A>;
#[doc = "transfer complete (TC) flag for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF2_A {
    #[doc = "0: no TC event"]
    B_0X0 = 0,
    #[doc = "1: a TC event occurred"]
    B_0X1 = 1,
}
impl From<TCIF2_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF2_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIF2_A {
        match self.bits {
            false => TCIF2_A::B_0X0,
            true => TCIF2_A::B_0X1,
        }
    }
    #[doc = "no TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF2_A::B_0X0
    }
    #[doc = "a TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF2_A::B_0X1
    }
}
#[doc = "Field `HTIF2` reader - half transfer (HT) flag for channel 2"]
pub type HTIF2_R = crate::BitReader<HTIF2_A>;
#[doc = "half transfer (HT) flag for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF2_A {
    #[doc = "0: no HT event"]
    B_0X0 = 0,
    #[doc = "1: a HT event occurred"]
    B_0X1 = 1,
}
impl From<HTIF2_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF2_A) -> Self {
        variant as u8 != 0
    }
}
impl HTIF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HTIF2_A {
        match self.bits {
            false => HTIF2_A::B_0X0,
            true => HTIF2_A::B_0X1,
        }
    }
    #[doc = "no HT event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HTIF2_A::B_0X0
    }
    #[doc = "a HT event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HTIF2_A::B_0X1
    }
}
#[doc = "Field `TEIF2` reader - transfer error (TE) flag for channel 2"]
pub type TEIF2_R = crate::BitReader<TEIF2_A>;
#[doc = "transfer error (TE) flag for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF2_A {
    #[doc = "0: no TE event"]
    B_0X0 = 0,
    #[doc = "1: a TE event occurred"]
    B_0X1 = 1,
}
impl From<TEIF2_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF2_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEIF2_A {
        match self.bits {
            false => TEIF2_A::B_0X0,
            true => TEIF2_A::B_0X1,
        }
    }
    #[doc = "no TE event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF2_A::B_0X0
    }
    #[doc = "a TE event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF2_A::B_0X1
    }
}
#[doc = "Field `GIF3` reader - global interrupt flag for channel 3"]
pub type GIF3_R = crate::BitReader<GIF3_A>;
#[doc = "global interrupt flag for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF3_A {
    #[doc = "0: no TE, HT or TC event"]
    B_0X0 = 0,
    #[doc = "1: a TE, HT or TC event occurred"]
    B_0X1 = 1,
}
impl From<GIF3_A> for bool {
    #[inline(always)]
    fn from(variant: GIF3_A) -> Self {
        variant as u8 != 0
    }
}
impl GIF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GIF3_A {
        match self.bits {
            false => GIF3_A::B_0X0,
            true => GIF3_A::B_0X1,
        }
    }
    #[doc = "no TE, HT or TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF3_A::B_0X0
    }
    #[doc = "a TE, HT or TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF3_A::B_0X1
    }
}
#[doc = "Field `TCIF3` reader - transfer complete (TC) flag for channel 3"]
pub type TCIF3_R = crate::BitReader<TCIF3_A>;
#[doc = "transfer complete (TC) flag for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF3_A {
    #[doc = "0: no TC event"]
    B_0X0 = 0,
    #[doc = "1: a TC event occurred"]
    B_0X1 = 1,
}
impl From<TCIF3_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF3_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIF3_A {
        match self.bits {
            false => TCIF3_A::B_0X0,
            true => TCIF3_A::B_0X1,
        }
    }
    #[doc = "no TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF3_A::B_0X0
    }
    #[doc = "a TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF3_A::B_0X1
    }
}
#[doc = "Field `HTIF3` reader - half transfer (HT) flag for channel 3"]
pub type HTIF3_R = crate::BitReader<HTIF3_A>;
#[doc = "half transfer (HT) flag for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF3_A {
    #[doc = "0: no HT event"]
    B_0X0 = 0,
    #[doc = "1: a HT event occurred"]
    B_0X1 = 1,
}
impl From<HTIF3_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF3_A) -> Self {
        variant as u8 != 0
    }
}
impl HTIF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HTIF3_A {
        match self.bits {
            false => HTIF3_A::B_0X0,
            true => HTIF3_A::B_0X1,
        }
    }
    #[doc = "no HT event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HTIF3_A::B_0X0
    }
    #[doc = "a HT event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HTIF3_A::B_0X1
    }
}
#[doc = "Field `TEIF3` reader - transfer error (TE) flag for channel 3"]
pub type TEIF3_R = crate::BitReader<TEIF3_A>;
#[doc = "transfer error (TE) flag for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF3_A {
    #[doc = "0: no TE event"]
    B_0X0 = 0,
    #[doc = "1: a TE event occurred"]
    B_0X1 = 1,
}
impl From<TEIF3_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF3_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEIF3_A {
        match self.bits {
            false => TEIF3_A::B_0X0,
            true => TEIF3_A::B_0X1,
        }
    }
    #[doc = "no TE event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF3_A::B_0X0
    }
    #[doc = "a TE event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF3_A::B_0X1
    }
}
#[doc = "Field `GIF4` reader - global interrupt flag for channel 4"]
pub type GIF4_R = crate::BitReader<GIF4_A>;
#[doc = "global interrupt flag for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF4_A {
    #[doc = "0: no TE, HT or TC event"]
    B_0X0 = 0,
    #[doc = "1: a TE, HT or TC event occurred"]
    B_0X1 = 1,
}
impl From<GIF4_A> for bool {
    #[inline(always)]
    fn from(variant: GIF4_A) -> Self {
        variant as u8 != 0
    }
}
impl GIF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GIF4_A {
        match self.bits {
            false => GIF4_A::B_0X0,
            true => GIF4_A::B_0X1,
        }
    }
    #[doc = "no TE, HT or TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF4_A::B_0X0
    }
    #[doc = "a TE, HT or TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF4_A::B_0X1
    }
}
#[doc = "Field `TCIF4` reader - transfer complete (TC) flag for channel 4"]
pub type TCIF4_R = crate::BitReader<TCIF4_A>;
#[doc = "transfer complete (TC) flag for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF4_A {
    #[doc = "0: no TC event"]
    B_0X0 = 0,
    #[doc = "1: a TC event occurred"]
    B_0X1 = 1,
}
impl From<TCIF4_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF4_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIF4_A {
        match self.bits {
            false => TCIF4_A::B_0X0,
            true => TCIF4_A::B_0X1,
        }
    }
    #[doc = "no TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF4_A::B_0X0
    }
    #[doc = "a TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF4_A::B_0X1
    }
}
#[doc = "Field `HTIF4` reader - half transfer (HT) flag for channel 4"]
pub type HTIF4_R = crate::BitReader<HTIF4_A>;
#[doc = "half transfer (HT) flag for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF4_A {
    #[doc = "0: no HT event"]
    B_0X0 = 0,
    #[doc = "1: a HT event occurred"]
    B_0X1 = 1,
}
impl From<HTIF4_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF4_A) -> Self {
        variant as u8 != 0
    }
}
impl HTIF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HTIF4_A {
        match self.bits {
            false => HTIF4_A::B_0X0,
            true => HTIF4_A::B_0X1,
        }
    }
    #[doc = "no HT event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HTIF4_A::B_0X0
    }
    #[doc = "a HT event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HTIF4_A::B_0X1
    }
}
#[doc = "Field `TEIF4` reader - transfer error (TE) flag for channel 4"]
pub type TEIF4_R = crate::BitReader<TEIF4_A>;
#[doc = "transfer error (TE) flag for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF4_A {
    #[doc = "0: no TE event"]
    B_0X0 = 0,
    #[doc = "1: a TE event occurred"]
    B_0X1 = 1,
}
impl From<TEIF4_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF4_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEIF4_A {
        match self.bits {
            false => TEIF4_A::B_0X0,
            true => TEIF4_A::B_0X1,
        }
    }
    #[doc = "no TE event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF4_A::B_0X0
    }
    #[doc = "a TE event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF4_A::B_0X1
    }
}
#[doc = "Field `GIF5` reader - global interrupt flag for channel 5"]
pub type GIF5_R = crate::BitReader<GIF5_A>;
#[doc = "global interrupt flag for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF5_A {
    #[doc = "0: no TE, HT or TC event"]
    B_0X0 = 0,
    #[doc = "1: a TE, HT or TC event occurred"]
    B_0X1 = 1,
}
impl From<GIF5_A> for bool {
    #[inline(always)]
    fn from(variant: GIF5_A) -> Self {
        variant as u8 != 0
    }
}
impl GIF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GIF5_A {
        match self.bits {
            false => GIF5_A::B_0X0,
            true => GIF5_A::B_0X1,
        }
    }
    #[doc = "no TE, HT or TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF5_A::B_0X0
    }
    #[doc = "a TE, HT or TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF5_A::B_0X1
    }
}
#[doc = "Field `TCIF5` reader - transfer complete (TC) flag for channel 5"]
pub type TCIF5_R = crate::BitReader<TCIF5_A>;
#[doc = "transfer complete (TC) flag for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF5_A {
    #[doc = "0: no TC event"]
    B_0X0 = 0,
    #[doc = "1: a TC event occurred"]
    B_0X1 = 1,
}
impl From<TCIF5_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF5_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIF5_A {
        match self.bits {
            false => TCIF5_A::B_0X0,
            true => TCIF5_A::B_0X1,
        }
    }
    #[doc = "no TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF5_A::B_0X0
    }
    #[doc = "a TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF5_A::B_0X1
    }
}
#[doc = "Field `HTIF5` reader - half transfer (HT) flag for channel 5"]
pub type HTIF5_R = crate::BitReader<HTIF5_A>;
#[doc = "half transfer (HT) flag for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF5_A {
    #[doc = "0: no HT event"]
    B_0X0 = 0,
    #[doc = "1: a HT event occurred"]
    B_0X1 = 1,
}
impl From<HTIF5_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF5_A) -> Self {
        variant as u8 != 0
    }
}
impl HTIF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HTIF5_A {
        match self.bits {
            false => HTIF5_A::B_0X0,
            true => HTIF5_A::B_0X1,
        }
    }
    #[doc = "no HT event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HTIF5_A::B_0X0
    }
    #[doc = "a HT event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HTIF5_A::B_0X1
    }
}
#[doc = "Field `TEIF5` reader - transfer error (TE) flag for channel 5"]
pub type TEIF5_R = crate::BitReader<TEIF5_A>;
#[doc = "transfer error (TE) flag for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF5_A {
    #[doc = "0: no TE event"]
    B_0X0 = 0,
    #[doc = "1: a TE event occurred"]
    B_0X1 = 1,
}
impl From<TEIF5_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF5_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEIF5_A {
        match self.bits {
            false => TEIF5_A::B_0X0,
            true => TEIF5_A::B_0X1,
        }
    }
    #[doc = "no TE event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF5_A::B_0X0
    }
    #[doc = "a TE event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF5_A::B_0X1
    }
}
#[doc = "Field `GIF6` reader - global interrupt flag for channel 6"]
pub type GIF6_R = crate::BitReader<GIF6_A>;
#[doc = "global interrupt flag for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF6_A {
    #[doc = "0: no TE, HT or TC event"]
    B_0X0 = 0,
    #[doc = "1: a TE, HT or TC event occurred"]
    B_0X1 = 1,
}
impl From<GIF6_A> for bool {
    #[inline(always)]
    fn from(variant: GIF6_A) -> Self {
        variant as u8 != 0
    }
}
impl GIF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GIF6_A {
        match self.bits {
            false => GIF6_A::B_0X0,
            true => GIF6_A::B_0X1,
        }
    }
    #[doc = "no TE, HT or TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF6_A::B_0X0
    }
    #[doc = "a TE, HT or TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF6_A::B_0X1
    }
}
#[doc = "Field `TCIF6` reader - transfer complete (TC) flag for channel 6"]
pub type TCIF6_R = crate::BitReader<TCIF6_A>;
#[doc = "transfer complete (TC) flag for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF6_A {
    #[doc = "0: no TC event"]
    B_0X0 = 0,
    #[doc = "1: a TC event occurred"]
    B_0X1 = 1,
}
impl From<TCIF6_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF6_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIF6_A {
        match self.bits {
            false => TCIF6_A::B_0X0,
            true => TCIF6_A::B_0X1,
        }
    }
    #[doc = "no TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF6_A::B_0X0
    }
    #[doc = "a TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF6_A::B_0X1
    }
}
#[doc = "Field `HTIF6` reader - half transfer (HT) flag for channel 6"]
pub type HTIF6_R = crate::BitReader<HTIF6_A>;
#[doc = "half transfer (HT) flag for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF6_A {
    #[doc = "0: no HT event"]
    B_0X0 = 0,
    #[doc = "1: a HT event occurred"]
    B_0X1 = 1,
}
impl From<HTIF6_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF6_A) -> Self {
        variant as u8 != 0
    }
}
impl HTIF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HTIF6_A {
        match self.bits {
            false => HTIF6_A::B_0X0,
            true => HTIF6_A::B_0X1,
        }
    }
    #[doc = "no HT event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HTIF6_A::B_0X0
    }
    #[doc = "a HT event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HTIF6_A::B_0X1
    }
}
#[doc = "Field `TEIF6` reader - transfer error (TE) flag for channel 6"]
pub type TEIF6_R = crate::BitReader<TEIF6_A>;
#[doc = "transfer error (TE) flag for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF6_A {
    #[doc = "0: no TE event"]
    B_0X0 = 0,
    #[doc = "1: a TE event occurred"]
    B_0X1 = 1,
}
impl From<TEIF6_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF6_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEIF6_A {
        match self.bits {
            false => TEIF6_A::B_0X0,
            true => TEIF6_A::B_0X1,
        }
    }
    #[doc = "no TE event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF6_A::B_0X0
    }
    #[doc = "a TE event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF6_A::B_0X1
    }
}
#[doc = "Field `GIF7` reader - global interrupt flag for channel 7"]
pub type GIF7_R = crate::BitReader<GIF7_A>;
#[doc = "global interrupt flag for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF7_A {
    #[doc = "0: no TE, HT or TC event"]
    B_0X0 = 0,
    #[doc = "1: a TE, HT or TC event occurred"]
    B_0X1 = 1,
}
impl From<GIF7_A> for bool {
    #[inline(always)]
    fn from(variant: GIF7_A) -> Self {
        variant as u8 != 0
    }
}
impl GIF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GIF7_A {
        match self.bits {
            false => GIF7_A::B_0X0,
            true => GIF7_A::B_0X1,
        }
    }
    #[doc = "no TE, HT or TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF7_A::B_0X0
    }
    #[doc = "a TE, HT or TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF7_A::B_0X1
    }
}
#[doc = "Field `TCIF7` reader - transfer complete (TC) flag for channel 7"]
pub type TCIF7_R = crate::BitReader<TCIF7_A>;
#[doc = "transfer complete (TC) flag for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF7_A {
    #[doc = "0: no TC event"]
    B_0X0 = 0,
    #[doc = "1: a TC event occurred"]
    B_0X1 = 1,
}
impl From<TCIF7_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF7_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIF7_A {
        match self.bits {
            false => TCIF7_A::B_0X0,
            true => TCIF7_A::B_0X1,
        }
    }
    #[doc = "no TC event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF7_A::B_0X0
    }
    #[doc = "a TC event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF7_A::B_0X1
    }
}
#[doc = "Field `HTIF7` reader - half transfer (HT) flag for channel 7"]
pub type HTIF7_R = crate::BitReader<HTIF7_A>;
#[doc = "half transfer (HT) flag for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF7_A {
    #[doc = "0: no HT event"]
    B_0X0 = 0,
    #[doc = "1: a HT event occurred"]
    B_0X1 = 1,
}
impl From<HTIF7_A> for bool {
    #[inline(always)]
    fn from(variant: HTIF7_A) -> Self {
        variant as u8 != 0
    }
}
impl HTIF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HTIF7_A {
        match self.bits {
            false => HTIF7_A::B_0X0,
            true => HTIF7_A::B_0X1,
        }
    }
    #[doc = "no HT event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HTIF7_A::B_0X0
    }
    #[doc = "a HT event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HTIF7_A::B_0X1
    }
}
#[doc = "Field `TEIF7` reader - transfer error (TE) flag for channel 7"]
pub type TEIF7_R = crate::BitReader<TEIF7_A>;
#[doc = "transfer error (TE) flag for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF7_A {
    #[doc = "0: no TE event"]
    B_0X0 = 0,
    #[doc = "1: a TE event occurred"]
    B_0X1 = 1,
}
impl From<TEIF7_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF7_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEIF7_A {
        match self.bits {
            false => TEIF7_A::B_0X0,
            true => TEIF7_A::B_0X1,
        }
    }
    #[doc = "no TE event"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF7_A::B_0X0
    }
    #[doc = "a TE event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF7_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - global interrupt flag for channel 1"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - transfer complete (TC) flag for channel 1"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - half transfer (HT) flag for channel 1"]
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - transfer error (TE) flag for channel 1"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - global interrupt flag for channel 2"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - transfer complete (TC) flag for channel 2"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - half transfer (HT) flag for channel 2"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - transfer error (TE) flag for channel 2"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - global interrupt flag for channel 3"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - transfer complete (TC) flag for channel 3"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - half transfer (HT) flag for channel 3"]
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - transfer error (TE) flag for channel 3"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - global interrupt flag for channel 4"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - transfer complete (TC) flag for channel 4"]
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - half transfer (HT) flag for channel 4"]
    #[inline(always)]
    pub fn htif4(&self) -> HTIF4_R {
        HTIF4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - transfer error (TE) flag for channel 4"]
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - global interrupt flag for channel 5"]
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - transfer complete (TC) flag for channel 5"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - half transfer (HT) flag for channel 5"]
    #[inline(always)]
    pub fn htif5(&self) -> HTIF5_R {
        HTIF5_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - transfer error (TE) flag for channel 5"]
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - global interrupt flag for channel 6"]
    #[inline(always)]
    pub fn gif6(&self) -> GIF6_R {
        GIF6_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - transfer complete (TC) flag for channel 6"]
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - half transfer (HT) flag for channel 6"]
    #[inline(always)]
    pub fn htif6(&self) -> HTIF6_R {
        HTIF6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - transfer error (TE) flag for channel 6"]
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - global interrupt flag for channel 7"]
    #[inline(always)]
    pub fn gif7(&self) -> GIF7_R {
        GIF7_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - transfer complete (TC) flag for channel 7"]
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - half transfer (HT) flag for channel 7"]
    #[inline(always)]
    pub fn htif7(&self) -> HTIF7_R {
        HTIF7_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - transfer error (TE) flag for channel 7"]
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "DMA interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_ISR_SPEC;
impl crate::RegisterSpec for DMA_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_isr::R`](R) reader structure"]
impl crate::Readable for DMA_ISR_SPEC {}
#[doc = "`reset()` method sets DMA_ISR to value 0"]
impl crate::Resettable for DMA_ISR_SPEC {
    const RESET_VALUE: u32 = 0;
}
