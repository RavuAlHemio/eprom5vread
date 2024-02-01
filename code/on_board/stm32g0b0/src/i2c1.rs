#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    i2c_cr1: I2C_CR1,
    i2c_cr2: I2C_CR2,
    i2c_oar1: I2C_OAR1,
    i2c_oar2: I2C_OAR2,
    i2c_timingr: I2C_TIMINGR,
    i2c_timeoutr: I2C_TIMEOUTR,
    i2c_isr: I2C_ISR,
    i2c_icr: I2C_ICR,
    i2c_pecr: I2C_PECR,
    i2c_rxdr: I2C_RXDR,
    i2c_txdr: I2C_TXDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    #[inline(always)]
    pub const fn i2c_cr1(&self) -> &I2C_CR1 {
        &self.i2c_cr1
    }
    #[doc = "0x04 - Control register 2"]
    #[inline(always)]
    pub const fn i2c_cr2(&self) -> &I2C_CR2 {
        &self.i2c_cr2
    }
    #[doc = "0x08 - Own address register 1"]
    #[inline(always)]
    pub const fn i2c_oar1(&self) -> &I2C_OAR1 {
        &self.i2c_oar1
    }
    #[doc = "0x0c - Own address register 2"]
    #[inline(always)]
    pub const fn i2c_oar2(&self) -> &I2C_OAR2 {
        &self.i2c_oar2
    }
    #[doc = "0x10 - Timing register"]
    #[inline(always)]
    pub const fn i2c_timingr(&self) -> &I2C_TIMINGR {
        &self.i2c_timingr
    }
    #[doc = "0x14 - Status register 1"]
    #[inline(always)]
    pub const fn i2c_timeoutr(&self) -> &I2C_TIMEOUTR {
        &self.i2c_timeoutr
    }
    #[doc = "0x18 - Interrupt and Status register"]
    #[inline(always)]
    pub const fn i2c_isr(&self) -> &I2C_ISR {
        &self.i2c_isr
    }
    #[doc = "0x1c - Interrupt clear register"]
    #[inline(always)]
    pub const fn i2c_icr(&self) -> &I2C_ICR {
        &self.i2c_icr
    }
    #[doc = "0x20 - PEC register"]
    #[inline(always)]
    pub const fn i2c_pecr(&self) -> &I2C_PECR {
        &self.i2c_pecr
    }
    #[doc = "0x24 - Receive data register"]
    #[inline(always)]
    pub const fn i2c_rxdr(&self) -> &I2C_RXDR {
        &self.i2c_rxdr
    }
    #[doc = "0x28 - Transmit data register"]
    #[inline(always)]
    pub const fn i2c_txdr(&self) -> &I2C_TXDR {
        &self.i2c_txdr
    }
}
#[doc = "I2C_CR1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_cr1`]
module"]
pub type I2C_CR1 = crate::Reg<i2c_cr1::I2C_CR1_SPEC>;
#[doc = "Control register 1"]
pub mod i2c_cr1;
#[doc = "I2C_CR2 (rw) register accessor: Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_cr2`]
module"]
pub type I2C_CR2 = crate::Reg<i2c_cr2::I2C_CR2_SPEC>;
#[doc = "Control register 2"]
pub mod i2c_cr2;
#[doc = "I2C_OAR1 (rw) register accessor: Own address register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_oar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_oar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_oar1`]
module"]
pub type I2C_OAR1 = crate::Reg<i2c_oar1::I2C_OAR1_SPEC>;
#[doc = "Own address register 1"]
pub mod i2c_oar1;
#[doc = "I2C_OAR2 (rw) register accessor: Own address register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_oar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_oar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_oar2`]
module"]
pub type I2C_OAR2 = crate::Reg<i2c_oar2::I2C_OAR2_SPEC>;
#[doc = "Own address register 2"]
pub mod i2c_oar2;
#[doc = "I2C_TIMINGR (rw) register accessor: Timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_timingr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_timingr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_timingr`]
module"]
pub type I2C_TIMINGR = crate::Reg<i2c_timingr::I2C_TIMINGR_SPEC>;
#[doc = "Timing register"]
pub mod i2c_timingr;
#[doc = "I2C_TIMEOUTR (rw) register accessor: Status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_timeoutr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_timeoutr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_timeoutr`]
module"]
pub type I2C_TIMEOUTR = crate::Reg<i2c_timeoutr::I2C_TIMEOUTR_SPEC>;
#[doc = "Status register 1"]
pub mod i2c_timeoutr;
#[doc = "I2C_ISR (rw) register accessor: Interrupt and Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_isr`]
module"]
pub type I2C_ISR = crate::Reg<i2c_isr::I2C_ISR_SPEC>;
#[doc = "Interrupt and Status register"]
pub mod i2c_isr;
#[doc = "I2C_ICR (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_icr`]
module"]
pub type I2C_ICR = crate::Reg<i2c_icr::I2C_ICR_SPEC>;
#[doc = "Interrupt clear register"]
pub mod i2c_icr;
#[doc = "I2C_PECR (r) register accessor: PEC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_pecr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_pecr`]
module"]
pub type I2C_PECR = crate::Reg<i2c_pecr::I2C_PECR_SPEC>;
#[doc = "PEC register"]
pub mod i2c_pecr;
#[doc = "I2C_RXDR (r) register accessor: Receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_rxdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_rxdr`]
module"]
pub type I2C_RXDR = crate::Reg<i2c_rxdr::I2C_RXDR_SPEC>;
#[doc = "Receive data register"]
pub mod i2c_rxdr;
#[doc = "I2C_TXDR (rw) register accessor: Transmit data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_txdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_txdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_txdr`]
module"]
pub type I2C_TXDR = crate::Reg<i2c_txdr::I2C_TXDR_SPEC>;
#[doc = "Transmit data register"]
pub mod i2c_txdr;
