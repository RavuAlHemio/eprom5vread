#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    spi_cr1: SPI_CR1,
    _reserved1: [u8; 0x02],
    spi_cr2: SPI_CR2,
    _reserved2: [u8; 0x02],
    spi_sr: SPI_SR,
    _reserved3: [u8; 0x02],
    spi_dr: SPI_DR,
    _reserved4: [u8; 0x02],
    spi_crcpr: SPI_CRCPR,
    _reserved5: [u8; 0x02],
    spi_rxcrcr: SPI_RXCRCR,
    _reserved6: [u8; 0x02],
    spi_txcrcr: SPI_TXCRCR,
    _reserved7: [u8; 0x02],
    spi_i2scfgr: SPI_I2SCFGR,
    _reserved8: [u8; 0x02],
    spi_i2spr: SPI_I2SPR,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn spi_cr1(&self) -> &SPI_CR1 {
        &self.spi_cr1
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn spi_cr2(&self) -> &SPI_CR2 {
        &self.spi_cr2
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn spi_sr(&self) -> &SPI_SR {
        &self.spi_sr
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn spi_dr(&self) -> &SPI_DR {
        &self.spi_dr
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn spi_crcpr(&self) -> &SPI_CRCPR {
        &self.spi_crcpr
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn spi_rxcrcr(&self) -> &SPI_RXCRCR {
        &self.spi_rxcrcr
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn spi_txcrcr(&self) -> &SPI_TXCRCR {
        &self.spi_txcrcr
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn spi_i2scfgr(&self) -> &SPI_I2SCFGR {
        &self.spi_i2scfgr
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn spi_i2spr(&self) -> &SPI_I2SPR {
        &self.spi_i2spr
    }
}
#[doc = "SPI_CR1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_cr1`]
module"]
pub type SPI_CR1 = crate::Reg<spi_cr1::SPI_CR1_SPEC>;
#[doc = ""]
pub mod spi_cr1;
#[doc = "SPI_CR2 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_cr2`]
module"]
pub type SPI_CR2 = crate::Reg<spi_cr2::SPI_CR2_SPEC>;
#[doc = ""]
pub mod spi_cr2;
#[doc = "SPI_SR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_sr`]
module"]
pub type SPI_SR = crate::Reg<spi_sr::SPI_SR_SPEC>;
#[doc = ""]
pub mod spi_sr;
#[doc = "SPI_DR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_dr`]
module"]
pub type SPI_DR = crate::Reg<spi_dr::SPI_DR_SPEC>;
#[doc = ""]
pub mod spi_dr;
#[doc = "SPI_CRCPR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_crcpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_crcpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_crcpr`]
module"]
pub type SPI_CRCPR = crate::Reg<spi_crcpr::SPI_CRCPR_SPEC>;
#[doc = ""]
pub mod spi_crcpr;
#[doc = "SPI_RXCRCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_rxcrcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_rxcrcr`]
module"]
pub type SPI_RXCRCR = crate::Reg<spi_rxcrcr::SPI_RXCRCR_SPEC>;
#[doc = ""]
pub mod spi_rxcrcr;
#[doc = "SPI_TXCRCR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_txcrcr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_txcrcr`]
module"]
pub type SPI_TXCRCR = crate::Reg<spi_txcrcr::SPI_TXCRCR_SPEC>;
#[doc = ""]
pub mod spi_txcrcr;
#[doc = "SPI_I2SCFGR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_i2scfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_i2scfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_i2scfgr`]
module"]
pub type SPI_I2SCFGR = crate::Reg<spi_i2scfgr::SPI_I2SCFGR_SPEC>;
#[doc = ""]
pub mod spi_i2scfgr;
#[doc = "SPI_I2SPR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_i2spr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_i2spr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_i2spr`]
module"]
pub type SPI_I2SPR = crate::Reg<spi_i2spr::SPI_I2SPR_SPEC>;
#[doc = ""]
pub mod spi_i2spr;
