#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    dmamux_c0cr: DMAMUX_C0CR,
    dmamux_c1cr: DMAMUX_C1CR,
    dmamux_c2cr: DMAMUX_C2CR,
    dmamux_c3cr: DMAMUX_C3CR,
    dmamux_c4cr: DMAMUX_C4CR,
    dmamux_c5cr: DMAMUX_C5CR,
    dmamux_c6cr: DMAMUX_C6CR,
    _reserved7: [u8; 0x64],
    dmamux_csr: DMAMUX_CSR,
    dmamux_cfr: DMAMUX_CFR,
    _reserved9: [u8; 0x78],
    dmamux_rg0cr: DMAMUX_RG0CR,
    dmamux_rg1cr: DMAMUX_RG1CR,
    dmamux_rg2cr: DMAMUX_RG2CR,
    dmamux_rg3cr: DMAMUX_RG3CR,
    _reserved13: [u8; 0x30],
    dmamux_rgsr: DMAMUX_RGSR,
    dmamux_rgcfr: DMAMUX_RGCFR,
}
impl RegisterBlock {
    #[doc = "0x00 - DMAMUX request line multiplexer channel x configuration register"]
    #[inline(always)]
    pub const fn dmamux_c0cr(&self) -> &DMAMUX_C0CR {
        &self.dmamux_c0cr
    }
    #[doc = "0x04 - DMAMUX request line multiplexer channel x configuration register"]
    #[inline(always)]
    pub const fn dmamux_c1cr(&self) -> &DMAMUX_C1CR {
        &self.dmamux_c1cr
    }
    #[doc = "0x08 - DMAMUX request line multiplexer channel x configuration register"]
    #[inline(always)]
    pub const fn dmamux_c2cr(&self) -> &DMAMUX_C2CR {
        &self.dmamux_c2cr
    }
    #[doc = "0x0c - DMAMUX request line multiplexer channel x configuration register"]
    #[inline(always)]
    pub const fn dmamux_c3cr(&self) -> &DMAMUX_C3CR {
        &self.dmamux_c3cr
    }
    #[doc = "0x10 - DMAMUX request line multiplexer channel x configuration register"]
    #[inline(always)]
    pub const fn dmamux_c4cr(&self) -> &DMAMUX_C4CR {
        &self.dmamux_c4cr
    }
    #[doc = "0x14 - DMAMUX request line multiplexer channel x configuration register"]
    #[inline(always)]
    pub const fn dmamux_c5cr(&self) -> &DMAMUX_C5CR {
        &self.dmamux_c5cr
    }
    #[doc = "0x18 - DMAMUX request line multiplexer channel x configuration register"]
    #[inline(always)]
    pub const fn dmamux_c6cr(&self) -> &DMAMUX_C6CR {
        &self.dmamux_c6cr
    }
    #[doc = "0x80 - DMAMUX request line multiplexer interrupt channel status register"]
    #[inline(always)]
    pub const fn dmamux_csr(&self) -> &DMAMUX_CSR {
        &self.dmamux_csr
    }
    #[doc = "0x84 - DMAMUX request line multiplexer interrupt clear flag register"]
    #[inline(always)]
    pub const fn dmamux_cfr(&self) -> &DMAMUX_CFR {
        &self.dmamux_cfr
    }
    #[doc = "0x100 - DMAMUX request generator channel x configuration register"]
    #[inline(always)]
    pub const fn dmamux_rg0cr(&self) -> &DMAMUX_RG0CR {
        &self.dmamux_rg0cr
    }
    #[doc = "0x104 - DMAMUX request generator channel x configuration register"]
    #[inline(always)]
    pub const fn dmamux_rg1cr(&self) -> &DMAMUX_RG1CR {
        &self.dmamux_rg1cr
    }
    #[doc = "0x108 - DMAMUX request generator channel x configuration register"]
    #[inline(always)]
    pub const fn dmamux_rg2cr(&self) -> &DMAMUX_RG2CR {
        &self.dmamux_rg2cr
    }
    #[doc = "0x10c - DMAMUX request generator channel x configuration register"]
    #[inline(always)]
    pub const fn dmamux_rg3cr(&self) -> &DMAMUX_RG3CR {
        &self.dmamux_rg3cr
    }
    #[doc = "0x140 - DMAMUX request generator interrupt status register"]
    #[inline(always)]
    pub const fn dmamux_rgsr(&self) -> &DMAMUX_RGSR {
        &self.dmamux_rgsr
    }
    #[doc = "0x144 - DMAMUX request generator interrupt clear flag register"]
    #[inline(always)]
    pub const fn dmamux_rgcfr(&self) -> &DMAMUX_RGCFR {
        &self.dmamux_rgcfr
    }
}
#[doc = "DMAMUX_C0CR (rw) register accessor: DMAMUX request line multiplexer channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c0cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c0cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c0cr`]
module"]
pub type DMAMUX_C0CR = crate::Reg<dmamux_c0cr::DMAMUX_C0CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c0cr;
#[doc = "DMAMUX_C1CR (rw) register accessor: DMAMUX request line multiplexer channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c1cr`]
module"]
pub type DMAMUX_C1CR = crate::Reg<dmamux_c1cr::DMAMUX_C1CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c1cr;
#[doc = "DMAMUX_C2CR (rw) register accessor: DMAMUX request line multiplexer channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c2cr`]
module"]
pub type DMAMUX_C2CR = crate::Reg<dmamux_c2cr::DMAMUX_C2CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c2cr;
#[doc = "DMAMUX_C3CR (rw) register accessor: DMAMUX request line multiplexer channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c3cr`]
module"]
pub type DMAMUX_C3CR = crate::Reg<dmamux_c3cr::DMAMUX_C3CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c3cr;
#[doc = "DMAMUX_C4CR (rw) register accessor: DMAMUX request line multiplexer channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c4cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c4cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c4cr`]
module"]
pub type DMAMUX_C4CR = crate::Reg<dmamux_c4cr::DMAMUX_C4CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c4cr;
#[doc = "DMAMUX_C5CR (rw) register accessor: DMAMUX request line multiplexer channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c5cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c5cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c5cr`]
module"]
pub type DMAMUX_C5CR = crate::Reg<dmamux_c5cr::DMAMUX_C5CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c5cr;
#[doc = "DMAMUX_C6CR (rw) register accessor: DMAMUX request line multiplexer channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_c6cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_c6cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_c6cr`]
module"]
pub type DMAMUX_C6CR = crate::Reg<dmamux_c6cr::DMAMUX_C6CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod dmamux_c6cr;
#[doc = "DMAMUX_CSR (r) register accessor: DMAMUX request line multiplexer interrupt channel status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_csr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_csr`]
module"]
pub type DMAMUX_CSR = crate::Reg<dmamux_csr::DMAMUX_CSR_SPEC>;
#[doc = "DMAMUX request line multiplexer interrupt channel status register"]
pub mod dmamux_csr;
#[doc = "DMAMUX_CFR (w) register accessor: DMAMUX request line multiplexer interrupt clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_cfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_cfr`]
module"]
pub type DMAMUX_CFR = crate::Reg<dmamux_cfr::DMAMUX_CFR_SPEC>;
#[doc = "DMAMUX request line multiplexer interrupt clear flag register"]
pub mod dmamux_cfr;
#[doc = "DMAMUX_RG0CR (rw) register accessor: DMAMUX request generator channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg0cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg0cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rg0cr`]
module"]
pub type DMAMUX_RG0CR = crate::Reg<dmamux_rg0cr::DMAMUX_RG0CR_SPEC>;
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod dmamux_rg0cr;
#[doc = "DMAMUX_RG1CR (rw) register accessor: DMAMUX request generator channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rg1cr`]
module"]
pub type DMAMUX_RG1CR = crate::Reg<dmamux_rg1cr::DMAMUX_RG1CR_SPEC>;
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod dmamux_rg1cr;
#[doc = "DMAMUX_RG2CR (rw) register accessor: DMAMUX request generator channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rg2cr`]
module"]
pub type DMAMUX_RG2CR = crate::Reg<dmamux_rg2cr::DMAMUX_RG2CR_SPEC>;
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod dmamux_rg2cr;
#[doc = "DMAMUX_RG3CR (rw) register accessor: DMAMUX request generator channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rg3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rg3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rg3cr`]
module"]
pub type DMAMUX_RG3CR = crate::Reg<dmamux_rg3cr::DMAMUX_RG3CR_SPEC>;
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod dmamux_rg3cr;
#[doc = "DMAMUX_RGSR (r) register accessor: DMAMUX request generator interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_rgsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rgsr`]
module"]
pub type DMAMUX_RGSR = crate::Reg<dmamux_rgsr::DMAMUX_RGSR_SPEC>;
#[doc = "DMAMUX request generator interrupt status register"]
pub mod dmamux_rgsr;
#[doc = "DMAMUX_RGCFR (w) register accessor: DMAMUX request generator interrupt clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamux_rgcfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamux_rgcfr`]
module"]
pub type DMAMUX_RGCFR = crate::Reg<dmamux_rgcfr::DMAMUX_RGCFR_SPEC>;
#[doc = "DMAMUX request generator interrupt clear flag register"]
pub mod dmamux_rgcfr;
