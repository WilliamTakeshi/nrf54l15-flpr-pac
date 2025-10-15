#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x30],
    tasks_constlat: TasksConstlat,
    tasks_lowpwr: TasksLowpwr,
    _reserved2: [u8; 0x78],
    subscribe_constlat: SubscribeConstlat,
    subscribe_lowpwr: SubscribeLowpwr,
    _reserved4: [u8; 0x78],
    events_pofwarn: EventsPofwarn,
    events_sleepenter: EventsSleepenter,
    events_sleepexit: EventsSleepexit,
    _reserved7: [u8; 0x74],
    publish_pofwarn: PublishPofwarn,
    publish_sleepenter: PublishSleepenter,
    publish_sleepexit: PublishSleepexit,
    _reserved10: [u8; 0x0144],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved13: [u8; 0x01f4],
    gpregret: [Gpregret; 2],
    _reserved14: [u8; 0x08],
    constlatstat: Constlatstat,
}
impl RegisterBlock {
    #[doc = "0x30 - Enable Constant Latency mode"]
    #[inline(always)]
    pub const fn tasks_constlat(&self) -> &TasksConstlat {
        &self.tasks_constlat
    }
    #[doc = "0x34 - Enable Low-power mode (variable latency)"]
    #[inline(always)]
    pub const fn tasks_lowpwr(&self) -> &TasksLowpwr {
        &self.tasks_lowpwr
    }
    #[doc = "0xb0 - Subscribe configuration for task CONSTLAT"]
    #[inline(always)]
    pub const fn subscribe_constlat(&self) -> &SubscribeConstlat {
        &self.subscribe_constlat
    }
    #[doc = "0xb4 - Subscribe configuration for task LOWPWR"]
    #[inline(always)]
    pub const fn subscribe_lowpwr(&self) -> &SubscribeLowpwr {
        &self.subscribe_lowpwr
    }
    #[doc = "0x130 - Power failure warning"]
    #[inline(always)]
    pub const fn events_pofwarn(&self) -> &EventsPofwarn {
        &self.events_pofwarn
    }
    #[doc = "0x134 - CPU entered WFI/WFE sleep"]
    #[inline(always)]
    pub const fn events_sleepenter(&self) -> &EventsSleepenter {
        &self.events_sleepenter
    }
    #[doc = "0x138 - CPU exited WFI/WFE sleep"]
    #[inline(always)]
    pub const fn events_sleepexit(&self) -> &EventsSleepexit {
        &self.events_sleepexit
    }
    #[doc = "0x1b0 - Publish configuration for event POFWARN"]
    #[inline(always)]
    pub const fn publish_pofwarn(&self) -> &PublishPofwarn {
        &self.publish_pofwarn
    }
    #[doc = "0x1b4 - Publish configuration for event SLEEPENTER"]
    #[inline(always)]
    pub const fn publish_sleepenter(&self) -> &PublishSleepenter {
        &self.publish_sleepenter
    }
    #[doc = "0x1b8 - Publish configuration for event SLEEPEXIT"]
    #[inline(always)]
    pub const fn publish_sleepexit(&self) -> &PublishSleepexit {
        &self.publish_sleepexit
    }
    #[doc = "0x300 - Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x304 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x308 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x500..0x508 - Description collection: General purpose retention register"]
    #[inline(always)]
    pub const fn gpregret(&self, n: usize) -> &Gpregret {
        &self.gpregret[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0x508 - Description collection: General purpose retention register"]
    #[inline(always)]
    pub fn gpregret_iter(&self) -> impl Iterator<Item = &Gpregret> {
        self.gpregret.iter()
    }
    #[doc = "0x510 - Status of constant latency"]
    #[inline(always)]
    pub const fn constlatstat(&self) -> &Constlatstat {
        &self.constlatstat
    }
}
#[doc = "TASKS_CONSTLAT (w) register accessor: Enable Constant Latency mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_constlat::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_constlat`] module"]
#[doc(alias = "TASKS_CONSTLAT")]
pub type TasksConstlat = crate::Reg<tasks_constlat::TasksConstlatSpec>;
#[doc = "Enable Constant Latency mode"]
pub mod tasks_constlat;
#[doc = "TASKS_LOWPWR (w) register accessor: Enable Low-power mode (variable latency)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_lowpwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_lowpwr`] module"]
#[doc(alias = "TASKS_LOWPWR")]
pub type TasksLowpwr = crate::Reg<tasks_lowpwr::TasksLowpwrSpec>;
#[doc = "Enable Low-power mode (variable latency)"]
pub mod tasks_lowpwr;
#[doc = "SUBSCRIBE_CONSTLAT (rw) register accessor: Subscribe configuration for task CONSTLAT\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_constlat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_constlat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_constlat`] module"]
#[doc(alias = "SUBSCRIBE_CONSTLAT")]
pub type SubscribeConstlat = crate::Reg<subscribe_constlat::SubscribeConstlatSpec>;
#[doc = "Subscribe configuration for task CONSTLAT"]
pub mod subscribe_constlat;
#[doc = "SUBSCRIBE_LOWPWR (rw) register accessor: Subscribe configuration for task LOWPWR\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_lowpwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_lowpwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_lowpwr`] module"]
#[doc(alias = "SUBSCRIBE_LOWPWR")]
pub type SubscribeLowpwr = crate::Reg<subscribe_lowpwr::SubscribeLowpwrSpec>;
#[doc = "Subscribe configuration for task LOWPWR"]
pub mod subscribe_lowpwr;
#[doc = "EVENTS_POFWARN (rw) register accessor: Power failure warning\n\nYou can [`read`](crate::Reg::read) this register and get [`events_pofwarn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_pofwarn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_pofwarn`] module"]
#[doc(alias = "EVENTS_POFWARN")]
pub type EventsPofwarn = crate::Reg<events_pofwarn::EventsPofwarnSpec>;
#[doc = "Power failure warning"]
pub mod events_pofwarn;
#[doc = "EVENTS_SLEEPENTER (rw) register accessor: CPU entered WFI/WFE sleep\n\nYou can [`read`](crate::Reg::read) this register and get [`events_sleepenter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_sleepenter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_sleepenter`] module"]
#[doc(alias = "EVENTS_SLEEPENTER")]
pub type EventsSleepenter = crate::Reg<events_sleepenter::EventsSleepenterSpec>;
#[doc = "CPU entered WFI/WFE sleep"]
pub mod events_sleepenter;
#[doc = "EVENTS_SLEEPEXIT (rw) register accessor: CPU exited WFI/WFE sleep\n\nYou can [`read`](crate::Reg::read) this register and get [`events_sleepexit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_sleepexit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_sleepexit`] module"]
#[doc(alias = "EVENTS_SLEEPEXIT")]
pub type EventsSleepexit = crate::Reg<events_sleepexit::EventsSleepexitSpec>;
#[doc = "CPU exited WFI/WFE sleep"]
pub mod events_sleepexit;
#[doc = "PUBLISH_POFWARN (rw) register accessor: Publish configuration for event POFWARN\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_pofwarn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_pofwarn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_pofwarn`] module"]
#[doc(alias = "PUBLISH_POFWARN")]
pub type PublishPofwarn = crate::Reg<publish_pofwarn::PublishPofwarnSpec>;
#[doc = "Publish configuration for event POFWARN"]
pub mod publish_pofwarn;
#[doc = "PUBLISH_SLEEPENTER (rw) register accessor: Publish configuration for event SLEEPENTER\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_sleepenter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_sleepenter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_sleepenter`] module"]
#[doc(alias = "PUBLISH_SLEEPENTER")]
pub type PublishSleepenter = crate::Reg<publish_sleepenter::PublishSleepenterSpec>;
#[doc = "Publish configuration for event SLEEPENTER"]
pub mod publish_sleepenter;
#[doc = "PUBLISH_SLEEPEXIT (rw) register accessor: Publish configuration for event SLEEPEXIT\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_sleepexit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_sleepexit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_sleepexit`] module"]
#[doc(alias = "PUBLISH_SLEEPEXIT")]
pub type PublishSleepexit = crate::Reg<publish_sleepexit::PublishSleepexitSpec>;
#[doc = "Publish configuration for event SLEEPEXIT"]
pub mod publish_sleepexit;
#[doc = "INTEN (rw) register accessor: Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`] module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "GPREGRET (rw) register accessor: Description collection: General purpose retention register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpregret::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpregret::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpregret`] module"]
#[doc(alias = "GPREGRET")]
pub type Gpregret = crate::Reg<gpregret::GpregretSpec>;
#[doc = "Description collection: General purpose retention register"]
pub mod gpregret;
#[doc = "CONSTLATSTAT (r) register accessor: Status of constant latency\n\nYou can [`read`](crate::Reg::read) this register and get [`constlatstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@constlatstat`] module"]
#[doc(alias = "CONSTLATSTAT")]
pub type Constlatstat = crate::Reg<constlatstat::ConstlatstatSpec>;
#[doc = "Status of constant latency"]
pub mod constlatstat;
