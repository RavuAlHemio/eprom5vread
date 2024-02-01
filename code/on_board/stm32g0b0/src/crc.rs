#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    crc_dr: CRC_DR,
    crc_idr: CRC_IDR,
    crc_cr: CRC_CR,
    _reserved3: [u8; 0x04],
    crc_init: CRC_INIT,
    crc_pol: CRC_POL,
}
impl RegisterBlock {
    #[doc = "0x00 - Data register"]
    #[inline(always)]
    pub const fn crc_dr(&self) -> &CRC_DR {
        &self.crc_dr
    }
    #[doc = "0x04 - Independent data register"]
    #[inline(always)]
    pub const fn crc_idr(&self) -> &CRC_IDR {
        &self.crc_idr
    }
    #[doc = "0x08 - Control register"]
    #[inline(always)]
    pub const fn crc_cr(&self) -> &CRC_CR {
        &self.crc_cr
    }
    #[doc = "0x10 - Initial CRC value"]
    #[inline(always)]
    pub const fn crc_init(&self) -> &CRC_INIT {
        &self.crc_init
    }
    #[doc = "0x14 - polynomial"]
    #[inline(always)]
    pub const fn crc_pol(&self) -> &CRC_POL {
        &self.crc_pol
    }
}
#[doc = "CRC_DR (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_dr`]
module"]
pub type CRC_DR = crate::Reg<crc_dr::CRC_DR_SPEC>;
#[doc = "Data register"]
pub mod crc_dr;
#[doc = "CRC_IDR (rw) register accessor: Independent data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_idr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_idr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_idr`]
module"]
pub type CRC_IDR = crate::Reg<crc_idr::CRC_IDR_SPEC>;
#[doc = "Independent data register"]
pub mod crc_idr;
#[doc = "CRC_CR (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_cr`]
module"]
pub type CRC_CR = crate::Reg<crc_cr::CRC_CR_SPEC>;
#[doc = "Control register"]
pub mod crc_cr;
#[doc = "CRC_INIT (rw) register accessor: Initial CRC value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_init::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_init::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_init`]
module"]
pub type CRC_INIT = crate::Reg<crc_init::CRC_INIT_SPEC>;
#[doc = "Initial CRC value"]
pub mod crc_init;
#[doc = "CRC_POL (rw) register accessor: polynomial\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_pol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_pol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_pol`]
module"]
pub type CRC_POL = crate::Reg<crc_pol::CRC_POL_SPEC>;
#[doc = "polynomial"]
pub mod crc_pol;
