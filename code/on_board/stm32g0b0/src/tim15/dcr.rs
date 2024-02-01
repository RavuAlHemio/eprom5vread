#[doc = "Register `DCR` reader"]
pub type R = crate::R<DCR_SPEC>;
#[doc = "Register `DCR` writer"]
pub type W = crate::W<DCR_SPEC>;
#[doc = "Field `DBA` reader - DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ..."]
pub type DBA_R = crate::FieldReader<DBA_A>;
#[doc = "DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ...\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBA_A {
    #[doc = "0: TIMx_CR1,"]
    B_0X0 = 0,
    #[doc = "1: TIMx_CR2,"]
    B_0X1 = 1,
    #[doc = "2: TIMx_SMCR,"]
    B_0X2 = 2,
}
impl From<DBA_A> for u8 {
    #[inline(always)]
    fn from(variant: DBA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DBA_A {
    type Ux = u8;
}
impl DBA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DBA_A> {
        match self.bits {
            0 => Some(DBA_A::B_0X0),
            1 => Some(DBA_A::B_0X1),
            2 => Some(DBA_A::B_0X2),
            _ => None,
        }
    }
    #[doc = "TIMx_CR1,"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBA_A::B_0X0
    }
    #[doc = "TIMx_CR2,"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBA_A::B_0X1
    }
    #[doc = "TIMx_SMCR,"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DBA_A::B_0X2
    }
}
#[doc = "Field `DBA` writer - DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ..."]
pub type DBA_W<'a, REG> = crate::FieldWriter<'a, REG, 5, DBA_A>;
impl<'a, REG> DBA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIMx_CR1,"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBA_A::B_0X0)
    }
    #[doc = "TIMx_CR2,"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBA_A::B_0X1)
    }
    #[doc = "TIMx_SMCR,"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(DBA_A::B_0X2)
    }
}
#[doc = "Field `DBL` reader - DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address). ..."]
pub type DBL_R = crate::FieldReader<DBL_A>;
#[doc = "DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address). ...\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBL_A {
    #[doc = "0: 1 transfer,"]
    B_0X0 = 0,
    #[doc = "1: 2 transfers,"]
    B_0X1 = 1,
    #[doc = "2: 3 transfers,"]
    B_0X2 = 2,
    #[doc = "17: 18 transfers."]
    B_0X11 = 17,
}
impl From<DBL_A> for u8 {
    #[inline(always)]
    fn from(variant: DBL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DBL_A {
    type Ux = u8;
}
impl DBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DBL_A> {
        match self.bits {
            0 => Some(DBL_A::B_0X0),
            1 => Some(DBL_A::B_0X1),
            2 => Some(DBL_A::B_0X2),
            17 => Some(DBL_A::B_0X11),
            _ => None,
        }
    }
    #[doc = "1 transfer,"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBL_A::B_0X0
    }
    #[doc = "2 transfers,"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBL_A::B_0X1
    }
    #[doc = "3 transfers,"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DBL_A::B_0X2
    }
    #[doc = "18 transfers."]
    #[inline(always)]
    pub fn is_b_0x11(&self) -> bool {
        *self == DBL_A::B_0X11
    }
}
#[doc = "Field `DBL` writer - DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address). ..."]
pub type DBL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, DBL_A>;
impl<'a, REG> DBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 transfer,"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBL_A::B_0X0)
    }
    #[doc = "2 transfers,"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBL_A::B_0X1)
    }
    #[doc = "3 transfers,"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(DBL_A::B_0X2)
    }
    #[doc = "18 transfers."]
    #[inline(always)]
    pub fn b_0x11(self) -> &'a mut crate::W<REG> {
        self.variant(DBL_A::B_0X11)
    }
}
impl R {
    #[doc = "Bits 0:4 - DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ..."]
    #[inline(always)]
    pub fn dba(&self) -> DBA_R {
        DBA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address). ..."]
    #[inline(always)]
    pub fn dbl(&self) -> DBL_R {
        DBL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ..."]
    #[inline(always)]
    #[must_use]
    pub fn dba(&mut self) -> DBA_W<DCR_SPEC> {
        DBA_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address). ..."]
    #[inline(always)]
    #[must_use]
    pub fn dbl(&mut self) -> DBL_W<DCR_SPEC> {
        DBL_W::new(self, 8)
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
#[doc = "DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCR_SPEC;
impl crate::RegisterSpec for DCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr::R`](R) reader structure"]
impl crate::Readable for DCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcr::W`](W) writer structure"]
impl crate::Writable for DCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCR to value 0"]
impl crate::Resettable for DCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
