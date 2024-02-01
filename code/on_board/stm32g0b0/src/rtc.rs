#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    rtc_tr: RTC_TR,
    rtc_dr: RTC_DR,
    rtc_ssr: RTC_SSR,
    rtc_icsr: RTC_ICSR,
    rtc_prer: RTC_PRER,
    rtc_wutr: RTC_WUTR,
    rtc_cr: RTC_CR,
    _reserved7: [u8; 0x08],
    rtc_wpr: RTC_WPR,
    rtc_calr: RTC_CALR,
    rtc_shiftr: RTC_SHIFTR,
    rtc_tstr: RTC_TSTR,
    rtc_tsdr: RTC_TSDR,
    rtc_tsssr: RTC_TSSSR,
    _reserved13: [u8; 0x04],
    rtc_alrmar: RTC_ALRMAR,
    rtc_alrmassr: RTC_ALRMASSR,
    rtc_alrmbr: RTC_ALRMBR,
    rtc_alrmbssr: RTC_ALRMBSSR,
    rtc_sr: RTC_SR,
    rtc_misr: RTC_MISR,
    _reserved19: [u8; 0x04],
    rtc_scr: RTC_SCR,
}
impl RegisterBlock {
    #[doc = "0x00 - RTC time register"]
    #[inline(always)]
    pub const fn rtc_tr(&self) -> &RTC_TR {
        &self.rtc_tr
    }
    #[doc = "0x04 - RTC date register"]
    #[inline(always)]
    pub const fn rtc_dr(&self) -> &RTC_DR {
        &self.rtc_dr
    }
    #[doc = "0x08 - RTC sub second register"]
    #[inline(always)]
    pub const fn rtc_ssr(&self) -> &RTC_SSR {
        &self.rtc_ssr
    }
    #[doc = "0x0c - RTC initialization control and status register"]
    #[inline(always)]
    pub const fn rtc_icsr(&self) -> &RTC_ICSR {
        &self.rtc_icsr
    }
    #[doc = "0x10 - RTC prescaler register"]
    #[inline(always)]
    pub const fn rtc_prer(&self) -> &RTC_PRER {
        &self.rtc_prer
    }
    #[doc = "0x14 - RTC wakeup timer register"]
    #[inline(always)]
    pub const fn rtc_wutr(&self) -> &RTC_WUTR {
        &self.rtc_wutr
    }
    #[doc = "0x18 - control register"]
    #[inline(always)]
    pub const fn rtc_cr(&self) -> &RTC_CR {
        &self.rtc_cr
    }
    #[doc = "0x24 - write protection register"]
    #[inline(always)]
    pub const fn rtc_wpr(&self) -> &RTC_WPR {
        &self.rtc_wpr
    }
    #[doc = "0x28 - RTC calibration register"]
    #[inline(always)]
    pub const fn rtc_calr(&self) -> &RTC_CALR {
        &self.rtc_calr
    }
    #[doc = "0x2c - RTC shift control register"]
    #[inline(always)]
    pub const fn rtc_shiftr(&self) -> &RTC_SHIFTR {
        &self.rtc_shiftr
    }
    #[doc = "0x30 - RTC timestamp time register"]
    #[inline(always)]
    pub const fn rtc_tstr(&self) -> &RTC_TSTR {
        &self.rtc_tstr
    }
    #[doc = "0x34 - RTC timestamp date register"]
    #[inline(always)]
    pub const fn rtc_tsdr(&self) -> &RTC_TSDR {
        &self.rtc_tsdr
    }
    #[doc = "0x38 - RTC timestamp sub second register"]
    #[inline(always)]
    pub const fn rtc_tsssr(&self) -> &RTC_TSSSR {
        &self.rtc_tsssr
    }
    #[doc = "0x40 - RTC alarm A register"]
    #[inline(always)]
    pub const fn rtc_alrmar(&self) -> &RTC_ALRMAR {
        &self.rtc_alrmar
    }
    #[doc = "0x44 - RTC alarm A sub second register"]
    #[inline(always)]
    pub const fn rtc_alrmassr(&self) -> &RTC_ALRMASSR {
        &self.rtc_alrmassr
    }
    #[doc = "0x48 - RTC alarm B register"]
    #[inline(always)]
    pub const fn rtc_alrmbr(&self) -> &RTC_ALRMBR {
        &self.rtc_alrmbr
    }
    #[doc = "0x4c - RTC alarm B sub second register"]
    #[inline(always)]
    pub const fn rtc_alrmbssr(&self) -> &RTC_ALRMBSSR {
        &self.rtc_alrmbssr
    }
    #[doc = "0x50 - RTC status register"]
    #[inline(always)]
    pub const fn rtc_sr(&self) -> &RTC_SR {
        &self.rtc_sr
    }
    #[doc = "0x54 - RTC masked interrupt status register"]
    #[inline(always)]
    pub const fn rtc_misr(&self) -> &RTC_MISR {
        &self.rtc_misr
    }
    #[doc = "0x5c - RTC status clear register"]
    #[inline(always)]
    pub const fn rtc_scr(&self) -> &RTC_SCR {
        &self.rtc_scr
    }
}
#[doc = "RTC_TR (rw) register accessor: RTC time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_tr`]
module"]
pub type RTC_TR = crate::Reg<rtc_tr::RTC_TR_SPEC>;
#[doc = "RTC time register"]
pub mod rtc_tr;
#[doc = "RTC_DR (rw) register accessor: RTC date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_dr`]
module"]
pub type RTC_DR = crate::Reg<rtc_dr::RTC_DR_SPEC>;
#[doc = "RTC date register"]
pub mod rtc_dr;
#[doc = "RTC_SSR (r) register accessor: RTC sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_ssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_ssr`]
module"]
pub type RTC_SSR = crate::Reg<rtc_ssr::RTC_SSR_SPEC>;
#[doc = "RTC sub second register"]
pub mod rtc_ssr;
#[doc = "RTC_ICSR (rw) register accessor: RTC initialization control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_icsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_icsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_icsr`]
module"]
pub type RTC_ICSR = crate::Reg<rtc_icsr::RTC_ICSR_SPEC>;
#[doc = "RTC initialization control and status register"]
pub mod rtc_icsr;
#[doc = "RTC_PRER (rw) register accessor: RTC prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_prer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_prer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_prer`]
module"]
pub type RTC_PRER = crate::Reg<rtc_prer::RTC_PRER_SPEC>;
#[doc = "RTC prescaler register"]
pub mod rtc_prer;
#[doc = "RTC_WUTR (rw) register accessor: RTC wakeup timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_wutr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_wutr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_wutr`]
module"]
pub type RTC_WUTR = crate::Reg<rtc_wutr::RTC_WUTR_SPEC>;
#[doc = "RTC wakeup timer register"]
pub mod rtc_wutr;
#[doc = "RTC_CR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_cr`]
module"]
pub type RTC_CR = crate::Reg<rtc_cr::RTC_CR_SPEC>;
#[doc = "control register"]
pub mod rtc_cr;
#[doc = "RTC_WPR (w) register accessor: write protection register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_wpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_wpr`]
module"]
pub type RTC_WPR = crate::Reg<rtc_wpr::RTC_WPR_SPEC>;
#[doc = "write protection register"]
pub mod rtc_wpr;
#[doc = "RTC_CALR (rw) register accessor: RTC calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_calr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_calr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_calr`]
module"]
pub type RTC_CALR = crate::Reg<rtc_calr::RTC_CALR_SPEC>;
#[doc = "RTC calibration register"]
pub mod rtc_calr;
#[doc = "RTC_SHIFTR (w) register accessor: RTC shift control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_shiftr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_shiftr`]
module"]
pub type RTC_SHIFTR = crate::Reg<rtc_shiftr::RTC_SHIFTR_SPEC>;
#[doc = "RTC shift control register"]
pub mod rtc_shiftr;
#[doc = "RTC_TSTR (r) register accessor: RTC timestamp time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_tstr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_tstr`]
module"]
pub type RTC_TSTR = crate::Reg<rtc_tstr::RTC_TSTR_SPEC>;
#[doc = "RTC timestamp time register"]
pub mod rtc_tstr;
#[doc = "RTC_TSDR (r) register accessor: RTC timestamp date register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_tsdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_tsdr`]
module"]
pub type RTC_TSDR = crate::Reg<rtc_tsdr::RTC_TSDR_SPEC>;
#[doc = "RTC timestamp date register"]
pub mod rtc_tsdr;
#[doc = "RTC_TSSSR (r) register accessor: RTC timestamp sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_tsssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_tsssr`]
module"]
pub type RTC_TSSSR = crate::Reg<rtc_tsssr::RTC_TSSSR_SPEC>;
#[doc = "RTC timestamp sub second register"]
pub mod rtc_tsssr;
#[doc = "RTC_ALRMAR (rw) register accessor: RTC alarm A register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_alrmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_alrmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_alrmar`]
module"]
pub type RTC_ALRMAR = crate::Reg<rtc_alrmar::RTC_ALRMAR_SPEC>;
#[doc = "RTC alarm A register"]
pub mod rtc_alrmar;
#[doc = "RTC_ALRMASSR (rw) register accessor: RTC alarm A sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_alrmassr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_alrmassr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_alrmassr`]
module"]
pub type RTC_ALRMASSR = crate::Reg<rtc_alrmassr::RTC_ALRMASSR_SPEC>;
#[doc = "RTC alarm A sub second register"]
pub mod rtc_alrmassr;
#[doc = "RTC_ALRMBR (rw) register accessor: RTC alarm B register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_alrmbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_alrmbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_alrmbr`]
module"]
pub type RTC_ALRMBR = crate::Reg<rtc_alrmbr::RTC_ALRMBR_SPEC>;
#[doc = "RTC alarm B register"]
pub mod rtc_alrmbr;
#[doc = "RTC_ALRMBSSR (rw) register accessor: RTC alarm B sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_alrmbssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_alrmbssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_alrmbssr`]
module"]
pub type RTC_ALRMBSSR = crate::Reg<rtc_alrmbssr::RTC_ALRMBSSR_SPEC>;
#[doc = "RTC alarm B sub second register"]
pub mod rtc_alrmbssr;
#[doc = "RTC_SR (r) register accessor: RTC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_sr`]
module"]
pub type RTC_SR = crate::Reg<rtc_sr::RTC_SR_SPEC>;
#[doc = "RTC status register"]
pub mod rtc_sr;
#[doc = "RTC_MISR (r) register accessor: RTC masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_misr`]
module"]
pub type RTC_MISR = crate::Reg<rtc_misr::RTC_MISR_SPEC>;
#[doc = "RTC masked interrupt status register"]
pub mod rtc_misr;
#[doc = "RTC_SCR (w) register accessor: RTC status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_scr`]
module"]
pub type RTC_SCR = crate::Reg<rtc_scr::RTC_SCR_SPEC>;
#[doc = "RTC status clear register"]
pub mod rtc_scr;
