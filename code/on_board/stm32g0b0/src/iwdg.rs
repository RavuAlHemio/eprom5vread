#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    iwdg_kr: IWDG_KR,
    iwdg_pr: IWDG_PR,
    iwdg_rlr: IWDG_RLR,
    iwdg_sr: IWDG_SR,
    iwdg_winr: IWDG_WINR,
}
impl RegisterBlock {
    #[doc = "0x00 - Key register"]
    #[inline(always)]
    pub const fn iwdg_kr(&self) -> &IWDG_KR {
        &self.iwdg_kr
    }
    #[doc = "0x04 - Prescaler register"]
    #[inline(always)]
    pub const fn iwdg_pr(&self) -> &IWDG_PR {
        &self.iwdg_pr
    }
    #[doc = "0x08 - Reload register"]
    #[inline(always)]
    pub const fn iwdg_rlr(&self) -> &IWDG_RLR {
        &self.iwdg_rlr
    }
    #[doc = "0x0c - Status register"]
    #[inline(always)]
    pub const fn iwdg_sr(&self) -> &IWDG_SR {
        &self.iwdg_sr
    }
    #[doc = "0x10 - Window register"]
    #[inline(always)]
    pub const fn iwdg_winr(&self) -> &IWDG_WINR {
        &self.iwdg_winr
    }
}
#[doc = "IWDG_KR (w) register accessor: Key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_kr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_kr`]
module"]
pub type IWDG_KR = crate::Reg<iwdg_kr::IWDG_KR_SPEC>;
#[doc = "Key register"]
pub mod iwdg_kr;
#[doc = "IWDG_PR (rw) register accessor: Prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_pr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_pr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_pr`]
module"]
pub type IWDG_PR = crate::Reg<iwdg_pr::IWDG_PR_SPEC>;
#[doc = "Prescaler register"]
pub mod iwdg_pr;
#[doc = "IWDG_RLR (rw) register accessor: Reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_rlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_rlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_rlr`]
module"]
pub type IWDG_RLR = crate::Reg<iwdg_rlr::IWDG_RLR_SPEC>;
#[doc = "Reload register"]
pub mod iwdg_rlr;
#[doc = "IWDG_SR (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_sr`]
module"]
pub type IWDG_SR = crate::Reg<iwdg_sr::IWDG_SR_SPEC>;
#[doc = "Status register"]
pub mod iwdg_sr;
#[doc = "IWDG_WINR (rw) register accessor: Window register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_winr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_winr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_winr`]
module"]
pub type IWDG_WINR = crate::Reg<iwdg_winr::IWDG_WINR_SPEC>;
#[doc = "Window register"]
pub mod iwdg_winr;
