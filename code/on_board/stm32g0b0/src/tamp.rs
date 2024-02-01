#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tamp_cr1: TAMP_CR1,
    tamp_cr2: TAMP_CR2,
    _reserved2: [u8; 0x04],
    tamp_fltcr: TAMP_FLTCR,
    _reserved3: [u8; 0x1c],
    tamp_ier: TAMP_IER,
    tamp_sr: TAMP_SR,
    tamp_misr: TAMP_MISR,
    _reserved6: [u8; 0x04],
    tamp_scr: TAMP_SCR,
    _reserved7: [u8; 0xc0],
    tamp_bkp0r: TAMP_BKP0R,
    tamp_bkp1r: TAMP_BKP1R,
    tamp_bkp2r: TAMP_BKP2R,
    tamp_bkp3r: TAMP_BKP3R,
    tamp_bkp4r: TAMP_BKP4R,
}
impl RegisterBlock {
    #[doc = "0x00 - TAMP control register 1"]
    #[inline(always)]
    pub const fn tamp_cr1(&self) -> &TAMP_CR1 {
        &self.tamp_cr1
    }
    #[doc = "0x04 - TAMP control register 2"]
    #[inline(always)]
    pub const fn tamp_cr2(&self) -> &TAMP_CR2 {
        &self.tamp_cr2
    }
    #[doc = "0x0c - TAMP filter control register"]
    #[inline(always)]
    pub const fn tamp_fltcr(&self) -> &TAMP_FLTCR {
        &self.tamp_fltcr
    }
    #[doc = "0x2c - TAMP interrupt enable register"]
    #[inline(always)]
    pub const fn tamp_ier(&self) -> &TAMP_IER {
        &self.tamp_ier
    }
    #[doc = "0x30 - TAMP status register"]
    #[inline(always)]
    pub const fn tamp_sr(&self) -> &TAMP_SR {
        &self.tamp_sr
    }
    #[doc = "0x34 - TAMP masked interrupt status register"]
    #[inline(always)]
    pub const fn tamp_misr(&self) -> &TAMP_MISR {
        &self.tamp_misr
    }
    #[doc = "0x3c - TAMP status clear register"]
    #[inline(always)]
    pub const fn tamp_scr(&self) -> &TAMP_SCR {
        &self.tamp_scr
    }
    #[doc = "0x100 - TAMP backup 0 register"]
    #[inline(always)]
    pub const fn tamp_bkp0r(&self) -> &TAMP_BKP0R {
        &self.tamp_bkp0r
    }
    #[doc = "0x104 - TAMP backup 1 register"]
    #[inline(always)]
    pub const fn tamp_bkp1r(&self) -> &TAMP_BKP1R {
        &self.tamp_bkp1r
    }
    #[doc = "0x108 - TAMP backup 2 register"]
    #[inline(always)]
    pub const fn tamp_bkp2r(&self) -> &TAMP_BKP2R {
        &self.tamp_bkp2r
    }
    #[doc = "0x10c - TAMP backup 3 register"]
    #[inline(always)]
    pub const fn tamp_bkp3r(&self) -> &TAMP_BKP3R {
        &self.tamp_bkp3r
    }
    #[doc = "0x110 - TAMP backup 4 register"]
    #[inline(always)]
    pub const fn tamp_bkp4r(&self) -> &TAMP_BKP4R {
        &self.tamp_bkp4r
    }
}
#[doc = "TAMP_CR1 (rw) register accessor: TAMP control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_cr1`]
module"]
pub type TAMP_CR1 = crate::Reg<tamp_cr1::TAMP_CR1_SPEC>;
#[doc = "TAMP control register 1"]
pub mod tamp_cr1;
#[doc = "TAMP_CR2 (rw) register accessor: TAMP control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_cr2`]
module"]
pub type TAMP_CR2 = crate::Reg<tamp_cr2::TAMP_CR2_SPEC>;
#[doc = "TAMP control register 2"]
pub mod tamp_cr2;
#[doc = "TAMP_FLTCR (rw) register accessor: TAMP filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_fltcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_fltcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_fltcr`]
module"]
pub type TAMP_FLTCR = crate::Reg<tamp_fltcr::TAMP_FLTCR_SPEC>;
#[doc = "TAMP filter control register"]
pub mod tamp_fltcr;
#[doc = "TAMP_IER (rw) register accessor: TAMP interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_ier`]
module"]
pub type TAMP_IER = crate::Reg<tamp_ier::TAMP_IER_SPEC>;
#[doc = "TAMP interrupt enable register"]
pub mod tamp_ier;
#[doc = "TAMP_SR (r) register accessor: TAMP status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_sr`]
module"]
pub type TAMP_SR = crate::Reg<tamp_sr::TAMP_SR_SPEC>;
#[doc = "TAMP status register"]
pub mod tamp_sr;
#[doc = "TAMP_MISR (r) register accessor: TAMP masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_misr`]
module"]
pub type TAMP_MISR = crate::Reg<tamp_misr::TAMP_MISR_SPEC>;
#[doc = "TAMP masked interrupt status register"]
pub mod tamp_misr;
#[doc = "TAMP_SCR (w) register accessor: TAMP status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_scr`]
module"]
pub type TAMP_SCR = crate::Reg<tamp_scr::TAMP_SCR_SPEC>;
#[doc = "TAMP status clear register"]
pub mod tamp_scr;
#[doc = "TAMP_BKP0R (rw) register accessor: TAMP backup 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_bkp0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_bkp0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_bkp0r`]
module"]
pub type TAMP_BKP0R = crate::Reg<tamp_bkp0r::TAMP_BKP0R_SPEC>;
#[doc = "TAMP backup 0 register"]
pub mod tamp_bkp0r;
#[doc = "TAMP_BKP1R (rw) register accessor: TAMP backup 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_bkp1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_bkp1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_bkp1r`]
module"]
pub type TAMP_BKP1R = crate::Reg<tamp_bkp1r::TAMP_BKP1R_SPEC>;
#[doc = "TAMP backup 1 register"]
pub mod tamp_bkp1r;
#[doc = "TAMP_BKP2R (rw) register accessor: TAMP backup 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_bkp2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_bkp2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_bkp2r`]
module"]
pub type TAMP_BKP2R = crate::Reg<tamp_bkp2r::TAMP_BKP2R_SPEC>;
#[doc = "TAMP backup 2 register"]
pub mod tamp_bkp2r;
#[doc = "TAMP_BKP3R (rw) register accessor: TAMP backup 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_bkp3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_bkp3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_bkp3r`]
module"]
pub type TAMP_BKP3R = crate::Reg<tamp_bkp3r::TAMP_BKP3R_SPEC>;
#[doc = "TAMP backup 3 register"]
pub mod tamp_bkp3r;
#[doc = "TAMP_BKP4R (rw) register accessor: TAMP backup 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_bkp4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_bkp4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_bkp4r`]
module"]
pub type TAMP_BKP4R = crate::Reg<tamp_bkp4r::TAMP_BKP4R_SPEC>;
#[doc = "TAMP backup 4 register"]
pub mod tamp_bkp4r;
