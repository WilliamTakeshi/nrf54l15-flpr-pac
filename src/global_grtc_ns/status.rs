#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "STATUS")]
pub struct Status {
    lftimer: Lftimer,
    pwm: Pwm,
    clkout: Clkout,
}
impl Status {
    #[doc = "0x00 - Low frequency timer status."]
    #[inline(always)]
    pub const fn lftimer(&self) -> &Lftimer {
        &self.lftimer
    }
    #[doc = "0x04 - PWM status."]
    #[inline(always)]
    pub const fn pwm(&self) -> &Pwm {
        &self.pwm
    }
    #[doc = "0x08 - CLKOUT configuration status."]
    #[inline(always)]
    pub const fn clkout(&self) -> &Clkout {
        &self.clkout
    }
}
#[doc = "LFTIMER (rw) register accessor: Low frequency timer status.\n\nYou can [`read`](crate::Reg::read) this register and get [`lftimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lftimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lftimer`] module"]
#[doc(alias = "LFTIMER")]
pub type Lftimer = crate::Reg<lftimer::LftimerSpec>;
#[doc = "Low frequency timer status."]
pub mod lftimer;
#[doc = "PWM (rw) register accessor: PWM status.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm`] module"]
#[doc(alias = "PWM")]
pub type Pwm = crate::Reg<pwm::PwmSpec>;
#[doc = "PWM status."]
pub mod pwm;
#[doc = "CLKOUT (rw) register accessor: CLKOUT configuration status.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkout`] module"]
#[doc(alias = "CLKOUT")]
pub type Clkout = crate::Reg<clkout::ClkoutSpec>;
#[doc = "CLKOUT configuration status."]
pub mod clkout;
