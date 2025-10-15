#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x30],
    bias: Bias,
    _reserved1: [u8; 0x04],
    ctrl: Ctrl,
}
impl RegisterBlock {
    #[doc = "0x30 - Bias control"]
    #[inline(always)]
    pub const fn bias(&self) -> &Bias {
        &self.bias
    }
    #[doc = "0x38 - Input sampling and buffering control (used by the VPR coprocessor for emulating a QSPI peripheral)"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
}
#[doc = "BIAS (rw) register accessor: Bias control\n\nYou can [`read`](crate::Reg::read) this register and get [`bias::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bias::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bias`] module"]
#[doc(alias = "BIAS")]
pub type Bias = crate::Reg<bias::BiasSpec>;
#[doc = "Bias control"]
pub mod bias;
#[doc = "CTRL (rw) register accessor: Input sampling and buffering control (used by the VPR coprocessor for emulating a QSPI peripheral)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Input sampling and buffering control (used by the VPR coprocessor for emulating a QSPI peripheral)"]
pub mod ctrl;
