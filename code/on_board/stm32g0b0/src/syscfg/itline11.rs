#[doc = "Register `ITLINE11` reader"]
pub type R = crate::R<ITLINE11_SPEC>;
#[doc = "Field `DMAMUX` reader - DMAMUX"]
pub type DMAMUX_R = crate::BitReader;
#[doc = "Field `DMA1_CH4` reader - DMA1_CH4"]
pub type DMA1_CH4_R = crate::BitReader;
#[doc = "Field `DMA1_CH5` reader - DMA1_CH5"]
pub type DMA1_CH5_R = crate::BitReader;
#[doc = "Field `DMA1_CH6` reader - DMA1_CH6"]
pub type DMA1_CH6_R = crate::BitReader;
#[doc = "Field `DMA1_CH7` reader - DMA1_CH7"]
pub type DMA1_CH7_R = crate::BitReader;
#[doc = "Field `DMA2_CH1` reader - DMA2_CH1"]
pub type DMA2_CH1_R = crate::BitReader;
#[doc = "Field `DMA2_CH2` reader - DMA2_CH2"]
pub type DMA2_CH2_R = crate::BitReader;
#[doc = "Field `DMA2_CH3` reader - DMA2_CH3"]
pub type DMA2_CH3_R = crate::BitReader;
#[doc = "Field `DMA2_CH4` reader - DMA2_CH4"]
pub type DMA2_CH4_R = crate::BitReader;
#[doc = "Field `DMA2_CH5` reader - DMA2_CH5"]
pub type DMA2_CH5_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMAMUX"]
    #[inline(always)]
    pub fn dmamux(&self) -> DMAMUX_R {
        DMAMUX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA1_CH4"]
    #[inline(always)]
    pub fn dma1_ch4(&self) -> DMA1_CH4_R {
        DMA1_CH4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA1_CH5"]
    #[inline(always)]
    pub fn dma1_ch5(&self) -> DMA1_CH5_R {
        DMA1_CH5_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA1_CH6"]
    #[inline(always)]
    pub fn dma1_ch6(&self) -> DMA1_CH6_R {
        DMA1_CH6_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA1_CH7"]
    #[inline(always)]
    pub fn dma1_ch7(&self) -> DMA1_CH7_R {
        DMA1_CH7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA2_CH1"]
    #[inline(always)]
    pub fn dma2_ch1(&self) -> DMA2_CH1_R {
        DMA2_CH1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA2_CH2"]
    #[inline(always)]
    pub fn dma2_ch2(&self) -> DMA2_CH2_R {
        DMA2_CH2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA2_CH3"]
    #[inline(always)]
    pub fn dma2_ch3(&self) -> DMA2_CH3_R {
        DMA2_CH3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA2_CH4"]
    #[inline(always)]
    pub fn dma2_ch4(&self) -> DMA2_CH4_R {
        DMA2_CH4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA2_CH5"]
    #[inline(always)]
    pub fn dma2_ch5(&self) -> DMA2_CH5_R {
        DMA2_CH5_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "interrupt line 11 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline11::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE11_SPEC;
impl crate::RegisterSpec for ITLINE11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline11::R`](R) reader structure"]
impl crate::Readable for ITLINE11_SPEC {}
#[doc = "`reset()` method sets ITLINE11 to value 0"]
impl crate::Resettable for ITLINE11_SPEC {
    const RESET_VALUE: u32 = 0;
}
