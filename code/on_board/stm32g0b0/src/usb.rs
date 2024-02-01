#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    usb_chep0r: USB_CHEP0R,
    usb_chep1r: USB_CHEP1R,
    usb_chep2r: USB_CHEP2R,
    usb_chep3r: USB_CHEP3R,
    usb_chep4r: USB_CHEP4R,
    usb_chep5r: USB_CHEP5R,
    usb_chep6r: USB_CHEP6R,
    usb_chep7r: USB_CHEP7R,
    _reserved8: [u8; 0x20],
    usb_cntr: USB_CNTR,
    usb_istr: USB_ISTR,
    usb_fnr: USB_FNR,
    usb_daddr: USB_DADDR,
    _reserved12: [u8; 0x04],
    usb_lpmcsr: USB_LPMCSR,
    usb_bcdr: USB_BCDR,
}
impl RegisterBlock {
    #[doc = "0x00 - USB endpoint/channel 0 register"]
    #[inline(always)]
    pub const fn usb_chep0r(&self) -> &USB_CHEP0R {
        &self.usb_chep0r
    }
    #[doc = "0x04 - USB endpoint/channel 1 register"]
    #[inline(always)]
    pub const fn usb_chep1r(&self) -> &USB_CHEP1R {
        &self.usb_chep1r
    }
    #[doc = "0x08 - USB endpoint/channel 2 register"]
    #[inline(always)]
    pub const fn usb_chep2r(&self) -> &USB_CHEP2R {
        &self.usb_chep2r
    }
    #[doc = "0x0c - USB endpoint/channel 3 register"]
    #[inline(always)]
    pub const fn usb_chep3r(&self) -> &USB_CHEP3R {
        &self.usb_chep3r
    }
    #[doc = "0x10 - USB endpoint/channel 4 register"]
    #[inline(always)]
    pub const fn usb_chep4r(&self) -> &USB_CHEP4R {
        &self.usb_chep4r
    }
    #[doc = "0x14 - USB endpoint/channel 5 register"]
    #[inline(always)]
    pub const fn usb_chep5r(&self) -> &USB_CHEP5R {
        &self.usb_chep5r
    }
    #[doc = "0x18 - USB endpoint/channel 6 register"]
    #[inline(always)]
    pub const fn usb_chep6r(&self) -> &USB_CHEP6R {
        &self.usb_chep6r
    }
    #[doc = "0x1c - USB endpoint/channel 7 register"]
    #[inline(always)]
    pub const fn usb_chep7r(&self) -> &USB_CHEP7R {
        &self.usb_chep7r
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn usb_cntr(&self) -> &USB_CNTR {
        &self.usb_cntr
    }
    #[doc = "0x44 - USB interrupt status register"]
    #[inline(always)]
    pub const fn usb_istr(&self) -> &USB_ISTR {
        &self.usb_istr
    }
    #[doc = "0x48 - USB frame number register"]
    #[inline(always)]
    pub const fn usb_fnr(&self) -> &USB_FNR {
        &self.usb_fnr
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn usb_daddr(&self) -> &USB_DADDR {
        &self.usb_daddr
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn usb_lpmcsr(&self) -> &USB_LPMCSR {
        &self.usb_lpmcsr
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn usb_bcdr(&self) -> &USB_BCDR {
        &self.usb_bcdr
    }
}
#[doc = "USB_CHEP0R (rw) register accessor: USB endpoint/channel 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_chep0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_chep0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_chep0r`]
module"]
pub type USB_CHEP0R = crate::Reg<usb_chep0r::USB_CHEP0R_SPEC>;
#[doc = "USB endpoint/channel 0 register"]
pub mod usb_chep0r;
#[doc = "USB_CHEP1R (rw) register accessor: USB endpoint/channel 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_chep1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_chep1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_chep1r`]
module"]
pub type USB_CHEP1R = crate::Reg<usb_chep1r::USB_CHEP1R_SPEC>;
#[doc = "USB endpoint/channel 1 register"]
pub mod usb_chep1r;
#[doc = "USB_CHEP2R (rw) register accessor: USB endpoint/channel 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_chep2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_chep2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_chep2r`]
module"]
pub type USB_CHEP2R = crate::Reg<usb_chep2r::USB_CHEP2R_SPEC>;
#[doc = "USB endpoint/channel 2 register"]
pub mod usb_chep2r;
#[doc = "USB_CHEP3R (rw) register accessor: USB endpoint/channel 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_chep3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_chep3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_chep3r`]
module"]
pub type USB_CHEP3R = crate::Reg<usb_chep3r::USB_CHEP3R_SPEC>;
#[doc = "USB endpoint/channel 3 register"]
pub mod usb_chep3r;
#[doc = "USB_CHEP4R (rw) register accessor: USB endpoint/channel 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_chep4r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_chep4r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_chep4r`]
module"]
pub type USB_CHEP4R = crate::Reg<usb_chep4r::USB_CHEP4R_SPEC>;
#[doc = "USB endpoint/channel 4 register"]
pub mod usb_chep4r;
#[doc = "USB_CHEP5R (rw) register accessor: USB endpoint/channel 5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_chep5r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_chep5r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_chep5r`]
module"]
pub type USB_CHEP5R = crate::Reg<usb_chep5r::USB_CHEP5R_SPEC>;
#[doc = "USB endpoint/channel 5 register"]
pub mod usb_chep5r;
#[doc = "USB_CHEP6R (rw) register accessor: USB endpoint/channel 6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_chep6r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_chep6r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_chep6r`]
module"]
pub type USB_CHEP6R = crate::Reg<usb_chep6r::USB_CHEP6R_SPEC>;
#[doc = "USB endpoint/channel 6 register"]
pub mod usb_chep6r;
#[doc = "USB_CHEP7R (rw) register accessor: USB endpoint/channel 7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_chep7r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_chep7r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_chep7r`]
module"]
pub type USB_CHEP7R = crate::Reg<usb_chep7r::USB_CHEP7R_SPEC>;
#[doc = "USB endpoint/channel 7 register"]
pub mod usb_chep7r;
#[doc = "USB_CNTR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_cntr`]
module"]
pub type USB_CNTR = crate::Reg<usb_cntr::USB_CNTR_SPEC>;
#[doc = ""]
pub mod usb_cntr;
#[doc = "USB_ISTR (rw) register accessor: USB interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_istr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_istr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_istr`]
module"]
pub type USB_ISTR = crate::Reg<usb_istr::USB_ISTR_SPEC>;
#[doc = "USB interrupt status register"]
pub mod usb_istr;
#[doc = "USB_FNR (r) register accessor: USB frame number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_fnr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_fnr`]
module"]
pub type USB_FNR = crate::Reg<usb_fnr::USB_FNR_SPEC>;
#[doc = "USB frame number register"]
pub mod usb_fnr;
#[doc = "USB_DADDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_daddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_daddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_daddr`]
module"]
pub type USB_DADDR = crate::Reg<usb_daddr::USB_DADDR_SPEC>;
#[doc = ""]
pub mod usb_daddr;
#[doc = "USB_LPMCSR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_lpmcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_lpmcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_lpmcsr`]
module"]
pub type USB_LPMCSR = crate::Reg<usb_lpmcsr::USB_LPMCSR_SPEC>;
#[doc = ""]
pub mod usb_lpmcsr;
#[doc = "USB_BCDR (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_bcdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_bcdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_bcdr`]
module"]
pub type USB_BCDR = crate::Reg<usb_bcdr::USB_BCDR_SPEC>;
#[doc = ""]
pub mod usb_bcdr;
