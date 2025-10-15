#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between event DMA.RX.END and task DMA.RX.START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxEndDmaRxStart {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaRxEndDmaRxStart> for bool {
    #[inline(always)]
    fn from(variant: DmaRxEndDmaRxStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_END_DMA_RX_START` reader - Shortcut between event DMA.RX.END and task DMA.RX.START"]
pub type DmaRxEndDmaRxStartR = crate::BitReader<DmaRxEndDmaRxStart>;
impl DmaRxEndDmaRxStartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxEndDmaRxStart {
        match self.bits {
            false => DmaRxEndDmaRxStart::Disabled,
            true => DmaRxEndDmaRxStart::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaRxEndDmaRxStart::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaRxEndDmaRxStart::Enabled
    }
}
#[doc = "Field `DMA_RX_END_DMA_RX_START` writer - Shortcut between event DMA.RX.END and task DMA.RX.START"]
pub type DmaRxEndDmaRxStartW<'a, REG> = crate::BitWriter<'a, REG, DmaRxEndDmaRxStart>;
impl<'a, REG> DmaRxEndDmaRxStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxEndDmaRxStart::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxEndDmaRxStart::Enabled)
    }
}
#[doc = "Shortcut between event DMA.RX.END and task DMA.RX.STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxEndDmaRxStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaRxEndDmaRxStop> for bool {
    #[inline(always)]
    fn from(variant: DmaRxEndDmaRxStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_END_DMA_RX_STOP` reader - Shortcut between event DMA.RX.END and task DMA.RX.STOP"]
pub type DmaRxEndDmaRxStopR = crate::BitReader<DmaRxEndDmaRxStop>;
impl DmaRxEndDmaRxStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxEndDmaRxStop {
        match self.bits {
            false => DmaRxEndDmaRxStop::Disabled,
            true => DmaRxEndDmaRxStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaRxEndDmaRxStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaRxEndDmaRxStop::Enabled
    }
}
#[doc = "Field `DMA_RX_END_DMA_RX_STOP` writer - Shortcut between event DMA.RX.END and task DMA.RX.STOP"]
pub type DmaRxEndDmaRxStopW<'a, REG> = crate::BitWriter<'a, REG, DmaRxEndDmaRxStop>;
impl<'a, REG> DmaRxEndDmaRxStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxEndDmaRxStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxEndDmaRxStop::Enabled)
    }
}
#[doc = "Shortcut between event DMA.TX.END and task DMA.TX.STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaTxEndDmaTxStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaTxEndDmaTxStop> for bool {
    #[inline(always)]
    fn from(variant: DmaTxEndDmaTxStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_TX_END_DMA_TX_STOP` reader - Shortcut between event DMA.TX.END and task DMA.TX.STOP"]
pub type DmaTxEndDmaTxStopR = crate::BitReader<DmaTxEndDmaTxStop>;
impl DmaTxEndDmaTxStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaTxEndDmaTxStop {
        match self.bits {
            false => DmaTxEndDmaTxStop::Disabled,
            true => DmaTxEndDmaTxStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaTxEndDmaTxStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaTxEndDmaTxStop::Enabled
    }
}
#[doc = "Field `DMA_TX_END_DMA_TX_STOP` writer - Shortcut between event DMA.TX.END and task DMA.TX.STOP"]
pub type DmaTxEndDmaTxStopW<'a, REG> = crate::BitWriter<'a, REG, DmaTxEndDmaTxStop>;
impl<'a, REG> DmaTxEndDmaTxStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaTxEndDmaTxStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaTxEndDmaTxStop::Enabled)
    }
}
#[doc = "Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[1\\] Allows daisy-chaining match events.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxMatch0DmaRxEnablematch1 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaRxMatch0DmaRxEnablematch1> for bool {
    #[inline(always)]
    fn from(variant: DmaRxMatch0DmaRxEnablematch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_MATCH0_DMA_RX_ENABLEMATCH1` reader - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[1\\] Allows daisy-chaining match events."]
pub type DmaRxMatch0DmaRxEnablematch1R = crate::BitReader<DmaRxMatch0DmaRxEnablematch1>;
impl DmaRxMatch0DmaRxEnablematch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxMatch0DmaRxEnablematch1 {
        match self.bits {
            false => DmaRxMatch0DmaRxEnablematch1::Disabled,
            true => DmaRxMatch0DmaRxEnablematch1::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaRxMatch0DmaRxEnablematch1::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaRxMatch0DmaRxEnablematch1::Enabled
    }
}
#[doc = "Field `DMA_RX_MATCH0_DMA_RX_ENABLEMATCH1` writer - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[1\\] Allows daisy-chaining match events."]
pub type DmaRxMatch0DmaRxEnablematch1W<'a, REG> =
    crate::BitWriter<'a, REG, DmaRxMatch0DmaRxEnablematch1>;
impl<'a, REG> DmaRxMatch0DmaRxEnablematch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch0DmaRxEnablematch1::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch0DmaRxEnablematch1::Enabled)
    }
}
#[doc = "Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[2\\] Allows daisy-chaining match events.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxMatch1DmaRxEnablematch2 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaRxMatch1DmaRxEnablematch2> for bool {
    #[inline(always)]
    fn from(variant: DmaRxMatch1DmaRxEnablematch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_MATCH1_DMA_RX_ENABLEMATCH2` reader - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[2\\] Allows daisy-chaining match events."]
pub type DmaRxMatch1DmaRxEnablematch2R = crate::BitReader<DmaRxMatch1DmaRxEnablematch2>;
impl DmaRxMatch1DmaRxEnablematch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxMatch1DmaRxEnablematch2 {
        match self.bits {
            false => DmaRxMatch1DmaRxEnablematch2::Disabled,
            true => DmaRxMatch1DmaRxEnablematch2::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaRxMatch1DmaRxEnablematch2::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaRxMatch1DmaRxEnablematch2::Enabled
    }
}
#[doc = "Field `DMA_RX_MATCH1_DMA_RX_ENABLEMATCH2` writer - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[2\\] Allows daisy-chaining match events."]
pub type DmaRxMatch1DmaRxEnablematch2W<'a, REG> =
    crate::BitWriter<'a, REG, DmaRxMatch1DmaRxEnablematch2>;
impl<'a, REG> DmaRxMatch1DmaRxEnablematch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch1DmaRxEnablematch2::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch1DmaRxEnablematch2::Enabled)
    }
}
#[doc = "Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[3\\] Allows daisy-chaining match events.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxMatch2DmaRxEnablematch3 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaRxMatch2DmaRxEnablematch3> for bool {
    #[inline(always)]
    fn from(variant: DmaRxMatch2DmaRxEnablematch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_MATCH2_DMA_RX_ENABLEMATCH3` reader - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[3\\] Allows daisy-chaining match events."]
pub type DmaRxMatch2DmaRxEnablematch3R = crate::BitReader<DmaRxMatch2DmaRxEnablematch3>;
impl DmaRxMatch2DmaRxEnablematch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxMatch2DmaRxEnablematch3 {
        match self.bits {
            false => DmaRxMatch2DmaRxEnablematch3::Disabled,
            true => DmaRxMatch2DmaRxEnablematch3::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaRxMatch2DmaRxEnablematch3::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaRxMatch2DmaRxEnablematch3::Enabled
    }
}
#[doc = "Field `DMA_RX_MATCH2_DMA_RX_ENABLEMATCH3` writer - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[3\\] Allows daisy-chaining match events."]
pub type DmaRxMatch2DmaRxEnablematch3W<'a, REG> =
    crate::BitWriter<'a, REG, DmaRxMatch2DmaRxEnablematch3>;
impl<'a, REG> DmaRxMatch2DmaRxEnablematch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch2DmaRxEnablematch3::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch2DmaRxEnablematch3::Enabled)
    }
}
#[doc = "Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[0\\] Allows daisy-chaining match events.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxMatch3DmaRxEnablematch0 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaRxMatch3DmaRxEnablematch0> for bool {
    #[inline(always)]
    fn from(variant: DmaRxMatch3DmaRxEnablematch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_MATCH3_DMA_RX_ENABLEMATCH0` reader - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[0\\] Allows daisy-chaining match events."]
pub type DmaRxMatch3DmaRxEnablematch0R = crate::BitReader<DmaRxMatch3DmaRxEnablematch0>;
impl DmaRxMatch3DmaRxEnablematch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxMatch3DmaRxEnablematch0 {
        match self.bits {
            false => DmaRxMatch3DmaRxEnablematch0::Disabled,
            true => DmaRxMatch3DmaRxEnablematch0::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaRxMatch3DmaRxEnablematch0::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaRxMatch3DmaRxEnablematch0::Enabled
    }
}
#[doc = "Field `DMA_RX_MATCH3_DMA_RX_ENABLEMATCH0` writer - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[0\\] Allows daisy-chaining match events."]
pub type DmaRxMatch3DmaRxEnablematch0W<'a, REG> =
    crate::BitWriter<'a, REG, DmaRxMatch3DmaRxEnablematch0>;
impl<'a, REG> DmaRxMatch3DmaRxEnablematch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch3DmaRxEnablematch0::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch3DmaRxEnablematch0::Enabled)
    }
}
#[doc = "Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxMatch0DmaRxDisablematch0 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaRxMatch0DmaRxDisablematch0> for bool {
    #[inline(always)]
    fn from(variant: DmaRxMatch0DmaRxDisablematch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_MATCH0_DMA_RX_DISABLEMATCH0` reader - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]"]
pub type DmaRxMatch0DmaRxDisablematch0R = crate::BitReader<DmaRxMatch0DmaRxDisablematch0>;
impl DmaRxMatch0DmaRxDisablematch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxMatch0DmaRxDisablematch0 {
        match self.bits {
            false => DmaRxMatch0DmaRxDisablematch0::Disabled,
            true => DmaRxMatch0DmaRxDisablematch0::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaRxMatch0DmaRxDisablematch0::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaRxMatch0DmaRxDisablematch0::Enabled
    }
}
#[doc = "Field `DMA_RX_MATCH0_DMA_RX_DISABLEMATCH0` writer - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]"]
pub type DmaRxMatch0DmaRxDisablematch0W<'a, REG> =
    crate::BitWriter<'a, REG, DmaRxMatch0DmaRxDisablematch0>;
impl<'a, REG> DmaRxMatch0DmaRxDisablematch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch0DmaRxDisablematch0::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch0DmaRxDisablematch0::Enabled)
    }
}
#[doc = "Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxMatch1DmaRxDisablematch1 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaRxMatch1DmaRxDisablematch1> for bool {
    #[inline(always)]
    fn from(variant: DmaRxMatch1DmaRxDisablematch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_MATCH1_DMA_RX_DISABLEMATCH1` reader - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]"]
pub type DmaRxMatch1DmaRxDisablematch1R = crate::BitReader<DmaRxMatch1DmaRxDisablematch1>;
impl DmaRxMatch1DmaRxDisablematch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxMatch1DmaRxDisablematch1 {
        match self.bits {
            false => DmaRxMatch1DmaRxDisablematch1::Disabled,
            true => DmaRxMatch1DmaRxDisablematch1::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaRxMatch1DmaRxDisablematch1::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaRxMatch1DmaRxDisablematch1::Enabled
    }
}
#[doc = "Field `DMA_RX_MATCH1_DMA_RX_DISABLEMATCH1` writer - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]"]
pub type DmaRxMatch1DmaRxDisablematch1W<'a, REG> =
    crate::BitWriter<'a, REG, DmaRxMatch1DmaRxDisablematch1>;
impl<'a, REG> DmaRxMatch1DmaRxDisablematch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch1DmaRxDisablematch1::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch1DmaRxDisablematch1::Enabled)
    }
}
#[doc = "Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxMatch2DmaRxDisablematch2 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaRxMatch2DmaRxDisablematch2> for bool {
    #[inline(always)]
    fn from(variant: DmaRxMatch2DmaRxDisablematch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_MATCH2_DMA_RX_DISABLEMATCH2` reader - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]"]
pub type DmaRxMatch2DmaRxDisablematch2R = crate::BitReader<DmaRxMatch2DmaRxDisablematch2>;
impl DmaRxMatch2DmaRxDisablematch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxMatch2DmaRxDisablematch2 {
        match self.bits {
            false => DmaRxMatch2DmaRxDisablematch2::Disabled,
            true => DmaRxMatch2DmaRxDisablematch2::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaRxMatch2DmaRxDisablematch2::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaRxMatch2DmaRxDisablematch2::Enabled
    }
}
#[doc = "Field `DMA_RX_MATCH2_DMA_RX_DISABLEMATCH2` writer - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]"]
pub type DmaRxMatch2DmaRxDisablematch2W<'a, REG> =
    crate::BitWriter<'a, REG, DmaRxMatch2DmaRxDisablematch2>;
impl<'a, REG> DmaRxMatch2DmaRxDisablematch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch2DmaRxDisablematch2::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch2DmaRxDisablematch2::Enabled)
    }
}
#[doc = "Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxMatch3DmaRxDisablematch3 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaRxMatch3DmaRxDisablematch3> for bool {
    #[inline(always)]
    fn from(variant: DmaRxMatch3DmaRxDisablematch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_MATCH3_DMA_RX_DISABLEMATCH3` reader - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]"]
pub type DmaRxMatch3DmaRxDisablematch3R = crate::BitReader<DmaRxMatch3DmaRxDisablematch3>;
impl DmaRxMatch3DmaRxDisablematch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxMatch3DmaRxDisablematch3 {
        match self.bits {
            false => DmaRxMatch3DmaRxDisablematch3::Disabled,
            true => DmaRxMatch3DmaRxDisablematch3::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaRxMatch3DmaRxDisablematch3::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaRxMatch3DmaRxDisablematch3::Enabled
    }
}
#[doc = "Field `DMA_RX_MATCH3_DMA_RX_DISABLEMATCH3` writer - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]"]
pub type DmaRxMatch3DmaRxDisablematch3W<'a, REG> =
    crate::BitWriter<'a, REG, DmaRxMatch3DmaRxDisablematch3>;
impl<'a, REG> DmaRxMatch3DmaRxDisablematch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch3DmaRxDisablematch3::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch3DmaRxDisablematch3::Enabled)
    }
}
#[doc = "Shortcut between event FRAMETIMEOUT and task DMA.RX.STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrametimeoutDmaRxStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<FrametimeoutDmaRxStop> for bool {
    #[inline(always)]
    fn from(variant: FrametimeoutDmaRxStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAMETIMEOUT_DMA_RX_STOP` reader - Shortcut between event FRAMETIMEOUT and task DMA.RX.STOP"]
pub type FrametimeoutDmaRxStopR = crate::BitReader<FrametimeoutDmaRxStop>;
impl FrametimeoutDmaRxStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FrametimeoutDmaRxStop {
        match self.bits {
            false => FrametimeoutDmaRxStop::Disabled,
            true => FrametimeoutDmaRxStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FrametimeoutDmaRxStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FrametimeoutDmaRxStop::Enabled
    }
}
#[doc = "Field `FRAMETIMEOUT_DMA_RX_STOP` writer - Shortcut between event FRAMETIMEOUT and task DMA.RX.STOP"]
pub type FrametimeoutDmaRxStopW<'a, REG> = crate::BitWriter<'a, REG, FrametimeoutDmaRxStop>;
impl<'a, REG> FrametimeoutDmaRxStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FrametimeoutDmaRxStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FrametimeoutDmaRxStop::Enabled)
    }
}
impl R {
    #[doc = "Bit 5 - Shortcut between event DMA.RX.END and task DMA.RX.START"]
    #[inline(always)]
    pub fn dma_rx_end_dma_rx_start(&self) -> DmaRxEndDmaRxStartR {
        DmaRxEndDmaRxStartR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Shortcut between event DMA.RX.END and task DMA.RX.STOP"]
    #[inline(always)]
    pub fn dma_rx_end_dma_rx_stop(&self) -> DmaRxEndDmaRxStopR {
        DmaRxEndDmaRxStopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 18 - Shortcut between event DMA.TX.END and task DMA.TX.STOP"]
    #[inline(always)]
    pub fn dma_tx_end_dma_tx_stop(&self) -> DmaTxEndDmaTxStopR {
        DmaTxEndDmaTxStopR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[1\\] Allows daisy-chaining match events."]
    #[inline(always)]
    pub fn dma_rx_match0_dma_rx_enablematch1(&self) -> DmaRxMatch0DmaRxEnablematch1R {
        DmaRxMatch0DmaRxEnablematch1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[2\\] Allows daisy-chaining match events."]
    #[inline(always)]
    pub fn dma_rx_match1_dma_rx_enablematch2(&self) -> DmaRxMatch1DmaRxEnablematch2R {
        DmaRxMatch1DmaRxEnablematch2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[3\\] Allows daisy-chaining match events."]
    #[inline(always)]
    pub fn dma_rx_match2_dma_rx_enablematch3(&self) -> DmaRxMatch2DmaRxEnablematch3R {
        DmaRxMatch2DmaRxEnablematch3R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[0\\] Allows daisy-chaining match events."]
    #[inline(always)]
    pub fn dma_rx_match3_dma_rx_enablematch0(&self) -> DmaRxMatch3DmaRxEnablematch0R {
        DmaRxMatch3DmaRxEnablematch0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub fn dma_rx_match0_dma_rx_disablematch0(&self) -> DmaRxMatch0DmaRxDisablematch0R {
        DmaRxMatch0DmaRxDisablematch0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub fn dma_rx_match1_dma_rx_disablematch1(&self) -> DmaRxMatch1DmaRxDisablematch1R {
        DmaRxMatch1DmaRxDisablematch1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub fn dma_rx_match2_dma_rx_disablematch2(&self) -> DmaRxMatch2DmaRxDisablematch2R {
        DmaRxMatch2DmaRxDisablematch2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub fn dma_rx_match3_dma_rx_disablematch3(&self) -> DmaRxMatch3DmaRxDisablematch3R {
        DmaRxMatch3DmaRxDisablematch3R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Shortcut between event FRAMETIMEOUT and task DMA.RX.STOP"]
    #[inline(always)]
    pub fn frametimeout_dma_rx_stop(&self) -> FrametimeoutDmaRxStopR {
        FrametimeoutDmaRxStopR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Shortcut between event DMA.RX.END and task DMA.RX.START"]
    #[inline(always)]
    pub fn dma_rx_end_dma_rx_start(&mut self) -> DmaRxEndDmaRxStartW<'_, ShortsSpec> {
        DmaRxEndDmaRxStartW::new(self, 5)
    }
    #[doc = "Bit 6 - Shortcut between event DMA.RX.END and task DMA.RX.STOP"]
    #[inline(always)]
    pub fn dma_rx_end_dma_rx_stop(&mut self) -> DmaRxEndDmaRxStopW<'_, ShortsSpec> {
        DmaRxEndDmaRxStopW::new(self, 6)
    }
    #[doc = "Bit 18 - Shortcut between event DMA.TX.END and task DMA.TX.STOP"]
    #[inline(always)]
    pub fn dma_tx_end_dma_tx_stop(&mut self) -> DmaTxEndDmaTxStopW<'_, ShortsSpec> {
        DmaTxEndDmaTxStopW::new(self, 18)
    }
    #[doc = "Bit 21 - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[1\\] Allows daisy-chaining match events."]
    #[inline(always)]
    pub fn dma_rx_match0_dma_rx_enablematch1(
        &mut self,
    ) -> DmaRxMatch0DmaRxEnablematch1W<'_, ShortsSpec> {
        DmaRxMatch0DmaRxEnablematch1W::new(self, 21)
    }
    #[doc = "Bit 22 - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[2\\] Allows daisy-chaining match events."]
    #[inline(always)]
    pub fn dma_rx_match1_dma_rx_enablematch2(
        &mut self,
    ) -> DmaRxMatch1DmaRxEnablematch2W<'_, ShortsSpec> {
        DmaRxMatch1DmaRxEnablematch2W::new(self, 22)
    }
    #[doc = "Bit 23 - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[3\\] Allows daisy-chaining match events."]
    #[inline(always)]
    pub fn dma_rx_match2_dma_rx_enablematch3(
        &mut self,
    ) -> DmaRxMatch2DmaRxEnablematch3W<'_, ShortsSpec> {
        DmaRxMatch2DmaRxEnablematch3W::new(self, 23)
    }
    #[doc = "Bit 24 - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.ENABLEMATCH\\[0\\] Allows daisy-chaining match events."]
    #[inline(always)]
    pub fn dma_rx_match3_dma_rx_enablematch0(
        &mut self,
    ) -> DmaRxMatch3DmaRxEnablematch0W<'_, ShortsSpec> {
        DmaRxMatch3DmaRxEnablematch0W::new(self, 24)
    }
    #[doc = "Bit 25 - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub fn dma_rx_match0_dma_rx_disablematch0(
        &mut self,
    ) -> DmaRxMatch0DmaRxDisablematch0W<'_, ShortsSpec> {
        DmaRxMatch0DmaRxDisablematch0W::new(self, 25)
    }
    #[doc = "Bit 26 - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub fn dma_rx_match1_dma_rx_disablematch1(
        &mut self,
    ) -> DmaRxMatch1DmaRxDisablematch1W<'_, ShortsSpec> {
        DmaRxMatch1DmaRxDisablematch1W::new(self, 26)
    }
    #[doc = "Bit 27 - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub fn dma_rx_match2_dma_rx_disablematch2(
        &mut self,
    ) -> DmaRxMatch2DmaRxDisablematch2W<'_, ShortsSpec> {
        DmaRxMatch2DmaRxDisablematch2W::new(self, 27)
    }
    #[doc = "Bit 28 - Shortcut between event DMA.RX.MATCH\\[n\\] and task DMA.RX.DISABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub fn dma_rx_match3_dma_rx_disablematch3(
        &mut self,
    ) -> DmaRxMatch3DmaRxDisablematch3W<'_, ShortsSpec> {
        DmaRxMatch3DmaRxDisablematch3W::new(self, 28)
    }
    #[doc = "Bit 29 - Shortcut between event FRAMETIMEOUT and task DMA.RX.STOP"]
    #[inline(always)]
    pub fn frametimeout_dma_rx_stop(&mut self) -> FrametimeoutDmaRxStopW<'_, ShortsSpec> {
        FrametimeoutDmaRxStopW::new(self, 29)
    }
}
#[doc = "Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShortsSpec;
impl crate::RegisterSpec for ShortsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shorts::R`](R) reader structure"]
impl crate::Readable for ShortsSpec {}
#[doc = "`write(|w| ..)` method takes [`shorts::W`](W) writer structure"]
impl crate::Writable for ShortsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for ShortsSpec {}
