#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x14],
    tasks_acquire: TasksAcquire,
    tasks_release: TasksRelease,
    _reserved2: [u8; 0x0c],
    tasks_dma: TasksDma,
    _reserved3: [u8; 0x44],
    subscribe_acquire: SubscribeAcquire,
    subscribe_release: SubscribeRelease,
    _reserved5: [u8; 0x0c],
    subscribe_dma: SubscribeDma,
    _reserved6: [u8; 0x34],
    events_end: EventsEnd,
    _reserved7: [u8; 0x10],
    events_acquired: EventsAcquired,
    _reserved8: [u8; 0x30],
    events_dma: EventsDma,
    _reserved9: [u8; 0x10],
    publish_end: PublishEnd,
    _reserved10: [u8; 0x10],
    publish_acquired: PublishAcquired,
    _reserved11: [u8; 0x30],
    publish_dma: PublishDma,
    _reserved12: [u8; 0x0c],
    shorts: Shorts,
    _reserved13: [u8; 0x0100],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved15: [u8; 0xf4],
    semstat: Semstat,
    _reserved16: [u8; 0x3c],
    status: Status,
    _reserved17: [u8; 0xbc],
    enable: Enable,
    _reserved18: [u8; 0x50],
    config: Config,
    _reserved19: [u8; 0x04],
    def: Def,
    _reserved20: [u8; 0x60],
    orc: Orc,
    _reserved21: [u8; 0x3c],
    psel: Psel,
    _reserved22: [u8; 0xec],
    dma: Dma,
}
impl RegisterBlock {
    #[doc = "0x14 - Acquire SPI semaphore"]
    #[inline(always)]
    pub const fn tasks_acquire(&self) -> &TasksAcquire {
        &self.tasks_acquire
    }
    #[doc = "0x18 - Release SPI semaphore, enabling the SPI slave to acquire it"]
    #[inline(always)]
    pub const fn tasks_release(&self) -> &TasksRelease {
        &self.tasks_release
    }
    #[doc = "0x28..0x50 - Peripheral tasks."]
    #[inline(always)]
    pub const fn tasks_dma(&self) -> &TasksDma {
        &self.tasks_dma
    }
    #[doc = "0x94 - Subscribe configuration for task ACQUIRE"]
    #[inline(always)]
    pub const fn subscribe_acquire(&self) -> &SubscribeAcquire {
        &self.subscribe_acquire
    }
    #[doc = "0x98 - Subscribe configuration for task RELEASE"]
    #[inline(always)]
    pub const fn subscribe_release(&self) -> &SubscribeRelease {
        &self.subscribe_release
    }
    #[doc = "0xa8..0xd0 - Subscribe configuration for tasks"]
    #[inline(always)]
    pub const fn subscribe_dma(&self) -> &SubscribeDma {
        &self.subscribe_dma
    }
    #[doc = "0x104 - Granted transaction completed"]
    #[inline(always)]
    pub const fn events_end(&self) -> &EventsEnd {
        &self.events_end
    }
    #[doc = "0x118 - Semaphore acquired"]
    #[inline(always)]
    pub const fn events_acquired(&self) -> &EventsAcquired {
        &self.events_acquired
    }
    #[doc = "0x14c..0x174 - Peripheral events."]
    #[inline(always)]
    pub const fn events_dma(&self) -> &EventsDma {
        &self.events_dma
    }
    #[doc = "0x184 - Publish configuration for event END"]
    #[inline(always)]
    pub const fn publish_end(&self) -> &PublishEnd {
        &self.publish_end
    }
    #[doc = "0x198 - Publish configuration for event ACQUIRED"]
    #[inline(always)]
    pub const fn publish_acquired(&self) -> &PublishAcquired {
        &self.publish_acquired
    }
    #[doc = "0x1cc..0x1f4 - Publish configuration for events"]
    #[inline(always)]
    pub const fn publish_dma(&self) -> &PublishDma {
        &self.publish_dma
    }
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(&self) -> &Shorts {
        &self.shorts
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
    #[doc = "0x400 - Semaphore status register"]
    #[inline(always)]
    pub const fn semstat(&self) -> &Semstat {
        &self.semstat
    }
    #[doc = "0x440 - Status from last transaction"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x500 - Enable SPI slave"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x554 - Configuration register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x55c - Default character. Character clocked out in case of an ignored transaction."]
    #[inline(always)]
    pub const fn def(&self) -> &Def {
        &self.def
    }
    #[doc = "0x5c0 - Over-read character"]
    #[inline(always)]
    pub const fn orc(&self) -> &Orc {
        &self.orc
    }
    #[doc = "0x600..0x614 - Unspecified"]
    #[inline(always)]
    pub const fn psel(&self) -> &Psel {
        &self.psel
    }
    #[doc = "0x700..0x75c - Unspecified"]
    #[inline(always)]
    pub const fn dma(&self) -> &Dma {
        &self.dma
    }
}
#[doc = "TASKS_ACQUIRE (w) register accessor: Acquire SPI semaphore\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_acquire::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_acquire`] module"]
#[doc(alias = "TASKS_ACQUIRE")]
pub type TasksAcquire = crate::Reg<tasks_acquire::TasksAcquireSpec>;
#[doc = "Acquire SPI semaphore"]
pub mod tasks_acquire;
#[doc = "TASKS_RELEASE (w) register accessor: Release SPI semaphore, enabling the SPI slave to acquire it\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_release::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_release`] module"]
#[doc(alias = "TASKS_RELEASE")]
pub type TasksRelease = crate::Reg<tasks_release::TasksReleaseSpec>;
#[doc = "Release SPI semaphore, enabling the SPI slave to acquire it"]
pub mod tasks_release;
#[doc = "Peripheral tasks."]
pub use self::tasks_dma::TasksDma;
#[doc = r"Cluster"]
#[doc = "Peripheral tasks."]
pub mod tasks_dma;
#[doc = "SUBSCRIBE_ACQUIRE (rw) register accessor: Subscribe configuration for task ACQUIRE\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_acquire::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_acquire::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_acquire`] module"]
#[doc(alias = "SUBSCRIBE_ACQUIRE")]
pub type SubscribeAcquire = crate::Reg<subscribe_acquire::SubscribeAcquireSpec>;
#[doc = "Subscribe configuration for task ACQUIRE"]
pub mod subscribe_acquire;
#[doc = "SUBSCRIBE_RELEASE (rw) register accessor: Subscribe configuration for task RELEASE\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_release::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_release::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_release`] module"]
#[doc(alias = "SUBSCRIBE_RELEASE")]
pub type SubscribeRelease = crate::Reg<subscribe_release::SubscribeReleaseSpec>;
#[doc = "Subscribe configuration for task RELEASE"]
pub mod subscribe_release;
#[doc = "Subscribe configuration for tasks"]
pub use self::subscribe_dma::SubscribeDma;
#[doc = r"Cluster"]
#[doc = "Subscribe configuration for tasks"]
pub mod subscribe_dma;
#[doc = "EVENTS_END (rw) register accessor: Granted transaction completed\n\nYou can [`read`](crate::Reg::read) this register and get [`events_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_end`] module"]
#[doc(alias = "EVENTS_END")]
pub type EventsEnd = crate::Reg<events_end::EventsEndSpec>;
#[doc = "Granted transaction completed"]
pub mod events_end;
#[doc = "EVENTS_ACQUIRED (rw) register accessor: Semaphore acquired\n\nYou can [`read`](crate::Reg::read) this register and get [`events_acquired::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_acquired::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_acquired`] module"]
#[doc(alias = "EVENTS_ACQUIRED")]
pub type EventsAcquired = crate::Reg<events_acquired::EventsAcquiredSpec>;
#[doc = "Semaphore acquired"]
pub mod events_acquired;
#[doc = "Peripheral events."]
pub use self::events_dma::EventsDma;
#[doc = r"Cluster"]
#[doc = "Peripheral events."]
pub mod events_dma;
#[doc = "PUBLISH_END (rw) register accessor: Publish configuration for event END\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_end`] module"]
#[doc(alias = "PUBLISH_END")]
pub type PublishEnd = crate::Reg<publish_end::PublishEndSpec>;
#[doc = "Publish configuration for event END"]
pub mod publish_end;
#[doc = "PUBLISH_ACQUIRED (rw) register accessor: Publish configuration for event ACQUIRED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_acquired::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_acquired::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_acquired`] module"]
#[doc(alias = "PUBLISH_ACQUIRED")]
pub type PublishAcquired = crate::Reg<publish_acquired::PublishAcquiredSpec>;
#[doc = "Publish configuration for event ACQUIRED"]
pub mod publish_acquired;
#[doc = "Publish configuration for events"]
pub use self::publish_dma::PublishDma;
#[doc = r"Cluster"]
#[doc = "Publish configuration for events"]
pub mod publish_dma;
#[doc = "SHORTS (rw) register accessor: Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`] module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
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
#[doc = "SEMSTAT (r) register accessor: Semaphore status register\n\nYou can [`read`](crate::Reg::read) this register and get [`semstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@semstat`] module"]
#[doc(alias = "SEMSTAT")]
pub type Semstat = crate::Reg<semstat::SemstatSpec>;
#[doc = "Semaphore status register"]
pub mod semstat;
#[doc = "STATUS (rw) register accessor: Status from last transaction\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status from last transaction"]
pub mod status;
#[doc = "ENABLE (rw) register accessor: Enable SPI slave\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable SPI slave"]
pub mod enable;
#[doc = "CONFIG (rw) register accessor: Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "DEF (rw) register accessor: Default character. Character clocked out in case of an ignored transaction.\n\nYou can [`read`](crate::Reg::read) this register and get [`def::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`def::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@def`] module"]
#[doc(alias = "DEF")]
pub type Def = crate::Reg<def::DefSpec>;
#[doc = "Default character. Character clocked out in case of an ignored transaction."]
pub mod def;
#[doc = "ORC (rw) register accessor: Over-read character\n\nYou can [`read`](crate::Reg::read) this register and get [`orc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`orc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@orc`] module"]
#[doc(alias = "ORC")]
pub type Orc = crate::Reg<orc::OrcSpec>;
#[doc = "Over-read character"]
pub mod orc;
#[doc = "Unspecified"]
pub use self::psel::Psel;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "Unspecified"]
pub use self::dma::Dma;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod dma;
