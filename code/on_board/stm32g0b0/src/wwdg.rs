#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    wwdg_cr: WWDG_CR,
    wwdg_cfr: WWDG_CFR,
    wwdg_sr: WWDG_SR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn wwdg_cr(&self) -> &WWDG_CR {
        &self.wwdg_cr
    }
    #[doc = "0x04 - Configuration register"]
    #[inline(always)]
    pub const fn wwdg_cfr(&self) -> &WWDG_CFR {
        &self.wwdg_cfr
    }
    #[doc = "0x08 - Status register"]
    #[inline(always)]
    pub const fn wwdg_sr(&self) -> &WWDG_SR {
        &self.wwdg_sr
    }
}
#[doc = "WWDG_CR (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wwdg_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdg_cr`]
module"]
pub type WWDG_CR = crate::Reg<wwdg_cr::WWDG_CR_SPEC>;
#[doc = "Control register"]
pub mod wwdg_cr;
#[doc = "WWDG_CFR (rw) register accessor: Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_cfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wwdg_cfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdg_cfr`]
module"]
pub type WWDG_CFR = crate::Reg<wwdg_cfr::WWDG_CFR_SPEC>;
#[doc = "Configuration register"]
pub mod wwdg_cfr;
#[doc = "WWDG_SR (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wwdg_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdg_sr`]
module"]
pub type WWDG_SR = crate::Reg<wwdg_sr::WWDG_SR_SPEC>;
#[doc = "Status register"]
pub mod wwdg_sr;
