#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopped {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Stopped> for bool {
    #[inline(always)]
    fn from(variant: Stopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` reader - Enable or disable interrupt for event STOPPED"]
pub type StoppedR = crate::BitReader<Stopped>;
impl StoppedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopped {
        match self.bits {
            false => Stopped::Disabled,
            true => Stopped::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stopped::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stopped::Enabled
    }
}
#[doc = "Field `STOPPED` writer - Enable or disable interrupt for event STOPPED"]
pub type StoppedW<'a, REG> = crate::BitWriter<'a, REG, Stopped>;
impl<'a, REG> StoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stopped::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stopped::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event SEQSTARTED\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqstarted0 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Seqstarted0> for bool {
    #[inline(always)]
    fn from(variant: Seqstarted0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQSTARTED0` reader - Enable or disable interrupt for event SEQSTARTED\\[0\\]"]
pub type Seqstarted0R = crate::BitReader<Seqstarted0>;
impl Seqstarted0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seqstarted0 {
        match self.bits {
            false => Seqstarted0::Disabled,
            true => Seqstarted0::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Seqstarted0::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Seqstarted0::Enabled
    }
}
#[doc = "Field `SEQSTARTED0` writer - Enable or disable interrupt for event SEQSTARTED\\[0\\]"]
pub type Seqstarted0W<'a, REG> = crate::BitWriter<'a, REG, Seqstarted0>;
impl<'a, REG> Seqstarted0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqstarted0::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqstarted0::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event SEQSTARTED\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqstarted1 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Seqstarted1> for bool {
    #[inline(always)]
    fn from(variant: Seqstarted1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQSTARTED1` reader - Enable or disable interrupt for event SEQSTARTED\\[1\\]"]
pub type Seqstarted1R = crate::BitReader<Seqstarted1>;
impl Seqstarted1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seqstarted1 {
        match self.bits {
            false => Seqstarted1::Disabled,
            true => Seqstarted1::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Seqstarted1::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Seqstarted1::Enabled
    }
}
#[doc = "Field `SEQSTARTED1` writer - Enable or disable interrupt for event SEQSTARTED\\[1\\]"]
pub type Seqstarted1W<'a, REG> = crate::BitWriter<'a, REG, Seqstarted1>;
impl<'a, REG> Seqstarted1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqstarted1::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqstarted1::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event SEQEND\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqend0 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Seqend0> for bool {
    #[inline(always)]
    fn from(variant: Seqend0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND0` reader - Enable or disable interrupt for event SEQEND\\[0\\]"]
pub type Seqend0R = crate::BitReader<Seqend0>;
impl Seqend0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seqend0 {
        match self.bits {
            false => Seqend0::Disabled,
            true => Seqend0::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Seqend0::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Seqend0::Enabled
    }
}
#[doc = "Field `SEQEND0` writer - Enable or disable interrupt for event SEQEND\\[0\\]"]
pub type Seqend0W<'a, REG> = crate::BitWriter<'a, REG, Seqend0>;
impl<'a, REG> Seqend0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqend0::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqend0::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event SEQEND\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqend1 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Seqend1> for bool {
    #[inline(always)]
    fn from(variant: Seqend1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND1` reader - Enable or disable interrupt for event SEQEND\\[1\\]"]
pub type Seqend1R = crate::BitReader<Seqend1>;
impl Seqend1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seqend1 {
        match self.bits {
            false => Seqend1::Disabled,
            true => Seqend1::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Seqend1::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Seqend1::Enabled
    }
}
#[doc = "Field `SEQEND1` writer - Enable or disable interrupt for event SEQEND\\[1\\]"]
pub type Seqend1W<'a, REG> = crate::BitWriter<'a, REG, Seqend1>;
impl<'a, REG> Seqend1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqend1::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqend1::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event PWMPERIODEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmperiodend {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Pwmperiodend> for bool {
    #[inline(always)]
    fn from(variant: Pwmperiodend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMPERIODEND` reader - Enable or disable interrupt for event PWMPERIODEND"]
pub type PwmperiodendR = crate::BitReader<Pwmperiodend>;
impl PwmperiodendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmperiodend {
        match self.bits {
            false => Pwmperiodend::Disabled,
            true => Pwmperiodend::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmperiodend::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmperiodend::Enabled
    }
}
#[doc = "Field `PWMPERIODEND` writer - Enable or disable interrupt for event PWMPERIODEND"]
pub type PwmperiodendW<'a, REG> = crate::BitWriter<'a, REG, Pwmperiodend>;
impl<'a, REG> PwmperiodendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmperiodend::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmperiodend::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event LOOPSDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Loopsdone {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Loopsdone> for bool {
    #[inline(always)]
    fn from(variant: Loopsdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPSDONE` reader - Enable or disable interrupt for event LOOPSDONE"]
pub type LoopsdoneR = crate::BitReader<Loopsdone>;
impl LoopsdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Loopsdone {
        match self.bits {
            false => Loopsdone::Disabled,
            true => Loopsdone::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Loopsdone::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Loopsdone::Enabled
    }
}
#[doc = "Field `LOOPSDONE` writer - Enable or disable interrupt for event LOOPSDONE"]
pub type LoopsdoneW<'a, REG> = crate::BitWriter<'a, REG, Loopsdone>;
impl<'a, REG> LoopsdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Loopsdone::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Loopsdone::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RAMUNDERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramunderflow {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Ramunderflow> for bool {
    #[inline(always)]
    fn from(variant: Ramunderflow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMUNDERFLOW` reader - Enable or disable interrupt for event RAMUNDERFLOW"]
pub type RamunderflowR = crate::BitReader<Ramunderflow>;
impl RamunderflowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramunderflow {
        match self.bits {
            false => Ramunderflow::Disabled,
            true => Ramunderflow::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ramunderflow::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ramunderflow::Enabled
    }
}
#[doc = "Field `RAMUNDERFLOW` writer - Enable or disable interrupt for event RAMUNDERFLOW"]
pub type RamunderflowW<'a, REG> = crate::BitWriter<'a, REG, Ramunderflow>;
impl<'a, REG> RamunderflowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ramunderflow::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ramunderflow::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DMASEQ0END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaseq0end {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Dmaseq0end> for bool {
    #[inline(always)]
    fn from(variant: Dmaseq0end) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMASEQ0END` reader - Enable or disable interrupt for event DMASEQ0END"]
pub type Dmaseq0endR = crate::BitReader<Dmaseq0end>;
impl Dmaseq0endR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaseq0end {
        match self.bits {
            false => Dmaseq0end::Disabled,
            true => Dmaseq0end::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmaseq0end::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmaseq0end::Enabled
    }
}
#[doc = "Field `DMASEQ0END` writer - Enable or disable interrupt for event DMASEQ0END"]
pub type Dmaseq0endW<'a, REG> = crate::BitWriter<'a, REG, Dmaseq0end>;
impl<'a, REG> Dmaseq0endW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaseq0end::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaseq0end::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DMASEQ0READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaseq0ready {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Dmaseq0ready> for bool {
    #[inline(always)]
    fn from(variant: Dmaseq0ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMASEQ0READY` reader - Enable or disable interrupt for event DMASEQ0READY"]
pub type Dmaseq0readyR = crate::BitReader<Dmaseq0ready>;
impl Dmaseq0readyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaseq0ready {
        match self.bits {
            false => Dmaseq0ready::Disabled,
            true => Dmaseq0ready::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmaseq0ready::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmaseq0ready::Enabled
    }
}
#[doc = "Field `DMASEQ0READY` writer - Enable or disable interrupt for event DMASEQ0READY"]
pub type Dmaseq0readyW<'a, REG> = crate::BitWriter<'a, REG, Dmaseq0ready>;
impl<'a, REG> Dmaseq0readyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaseq0ready::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaseq0ready::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DMASEQ0BUSERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaseq0buserror {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Dmaseq0buserror> for bool {
    #[inline(always)]
    fn from(variant: Dmaseq0buserror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMASEQ0BUSERROR` reader - Enable or disable interrupt for event DMASEQ0BUSERROR"]
pub type Dmaseq0buserrorR = crate::BitReader<Dmaseq0buserror>;
impl Dmaseq0buserrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaseq0buserror {
        match self.bits {
            false => Dmaseq0buserror::Disabled,
            true => Dmaseq0buserror::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmaseq0buserror::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmaseq0buserror::Enabled
    }
}
#[doc = "Field `DMASEQ0BUSERROR` writer - Enable or disable interrupt for event DMASEQ0BUSERROR"]
pub type Dmaseq0buserrorW<'a, REG> = crate::BitWriter<'a, REG, Dmaseq0buserror>;
impl<'a, REG> Dmaseq0buserrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaseq0buserror::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaseq0buserror::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DMASEQ1END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaseq1end {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Dmaseq1end> for bool {
    #[inline(always)]
    fn from(variant: Dmaseq1end) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMASEQ1END` reader - Enable or disable interrupt for event DMASEQ1END"]
pub type Dmaseq1endR = crate::BitReader<Dmaseq1end>;
impl Dmaseq1endR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaseq1end {
        match self.bits {
            false => Dmaseq1end::Disabled,
            true => Dmaseq1end::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmaseq1end::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmaseq1end::Enabled
    }
}
#[doc = "Field `DMASEQ1END` writer - Enable or disable interrupt for event DMASEQ1END"]
pub type Dmaseq1endW<'a, REG> = crate::BitWriter<'a, REG, Dmaseq1end>;
impl<'a, REG> Dmaseq1endW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaseq1end::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaseq1end::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DMASEQ1READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaseq1ready {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Dmaseq1ready> for bool {
    #[inline(always)]
    fn from(variant: Dmaseq1ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMASEQ1READY` reader - Enable or disable interrupt for event DMASEQ1READY"]
pub type Dmaseq1readyR = crate::BitReader<Dmaseq1ready>;
impl Dmaseq1readyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaseq1ready {
        match self.bits {
            false => Dmaseq1ready::Disabled,
            true => Dmaseq1ready::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmaseq1ready::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmaseq1ready::Enabled
    }
}
#[doc = "Field `DMASEQ1READY` writer - Enable or disable interrupt for event DMASEQ1READY"]
pub type Dmaseq1readyW<'a, REG> = crate::BitWriter<'a, REG, Dmaseq1ready>;
impl<'a, REG> Dmaseq1readyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaseq1ready::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaseq1ready::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event DMASEQ1BUSERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaseq1buserror {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Dmaseq1buserror> for bool {
    #[inline(always)]
    fn from(variant: Dmaseq1buserror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMASEQ1BUSERROR` reader - Enable or disable interrupt for event DMASEQ1BUSERROR"]
pub type Dmaseq1buserrorR = crate::BitReader<Dmaseq1buserror>;
impl Dmaseq1buserrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaseq1buserror {
        match self.bits {
            false => Dmaseq1buserror::Disabled,
            true => Dmaseq1buserror::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmaseq1buserror::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmaseq1buserror::Enabled
    }
}
#[doc = "Field `DMASEQ1BUSERROR` writer - Enable or disable interrupt for event DMASEQ1BUSERROR"]
pub type Dmaseq1buserrorW<'a, REG> = crate::BitWriter<'a, REG, Dmaseq1buserror>;
impl<'a, REG> Dmaseq1buserrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaseq1buserror::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaseq1buserror::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event COMPAREMATCH\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparematch0 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Comparematch0> for bool {
    #[inline(always)]
    fn from(variant: Comparematch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREMATCH0` reader - Enable or disable interrupt for event COMPAREMATCH\\[0\\]"]
pub type Comparematch0R = crate::BitReader<Comparematch0>;
impl Comparematch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comparematch0 {
        match self.bits {
            false => Comparematch0::Disabled,
            true => Comparematch0::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Comparematch0::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Comparematch0::Enabled
    }
}
#[doc = "Field `COMPAREMATCH0` writer - Enable or disable interrupt for event COMPAREMATCH\\[0\\]"]
pub type Comparematch0W<'a, REG> = crate::BitWriter<'a, REG, Comparematch0>;
impl<'a, REG> Comparematch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Comparematch0::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Comparematch0::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event COMPAREMATCH\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparematch1 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Comparematch1> for bool {
    #[inline(always)]
    fn from(variant: Comparematch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREMATCH1` reader - Enable or disable interrupt for event COMPAREMATCH\\[1\\]"]
pub type Comparematch1R = crate::BitReader<Comparematch1>;
impl Comparematch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comparematch1 {
        match self.bits {
            false => Comparematch1::Disabled,
            true => Comparematch1::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Comparematch1::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Comparematch1::Enabled
    }
}
#[doc = "Field `COMPAREMATCH1` writer - Enable or disable interrupt for event COMPAREMATCH\\[1\\]"]
pub type Comparematch1W<'a, REG> = crate::BitWriter<'a, REG, Comparematch1>;
impl<'a, REG> Comparematch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Comparematch1::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Comparematch1::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event COMPAREMATCH\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparematch2 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Comparematch2> for bool {
    #[inline(always)]
    fn from(variant: Comparematch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREMATCH2` reader - Enable or disable interrupt for event COMPAREMATCH\\[2\\]"]
pub type Comparematch2R = crate::BitReader<Comparematch2>;
impl Comparematch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comparematch2 {
        match self.bits {
            false => Comparematch2::Disabled,
            true => Comparematch2::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Comparematch2::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Comparematch2::Enabled
    }
}
#[doc = "Field `COMPAREMATCH2` writer - Enable or disable interrupt for event COMPAREMATCH\\[2\\]"]
pub type Comparematch2W<'a, REG> = crate::BitWriter<'a, REG, Comparematch2>;
impl<'a, REG> Comparematch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Comparematch2::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Comparematch2::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event COMPAREMATCH\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparematch3 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Comparematch3> for bool {
    #[inline(always)]
    fn from(variant: Comparematch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREMATCH3` reader - Enable or disable interrupt for event COMPAREMATCH\\[3\\]"]
pub type Comparematch3R = crate::BitReader<Comparematch3>;
impl Comparematch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comparematch3 {
        match self.bits {
            false => Comparematch3::Disabled,
            true => Comparematch3::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Comparematch3::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Comparematch3::Enabled
    }
}
#[doc = "Field `COMPAREMATCH3` writer - Enable or disable interrupt for event COMPAREMATCH\\[3\\]"]
pub type Comparematch3W<'a, REG> = crate::BitWriter<'a, REG, Comparematch3>;
impl<'a, REG> Comparematch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Comparematch3::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Comparematch3::Enabled)
    }
}
impl R {
    #[doc = "Bit 1 - Enable or disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> StoppedR {
        StoppedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event SEQSTARTED\\[0\\]"]
    #[inline(always)]
    pub fn seqstarted0(&self) -> Seqstarted0R {
        Seqstarted0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event SEQSTARTED\\[1\\]"]
    #[inline(always)]
    pub fn seqstarted1(&self) -> Seqstarted1R {
        Seqstarted1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event SEQEND\\[0\\]"]
    #[inline(always)]
    pub fn seqend0(&self) -> Seqend0R {
        Seqend0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event SEQEND\\[1\\]"]
    #[inline(always)]
    pub fn seqend1(&self) -> Seqend1R {
        Seqend1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event PWMPERIODEND"]
    #[inline(always)]
    pub fn pwmperiodend(&self) -> PwmperiodendR {
        PwmperiodendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event LOOPSDONE"]
    #[inline(always)]
    pub fn loopsdone(&self) -> LoopsdoneR {
        LoopsdoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event RAMUNDERFLOW"]
    #[inline(always)]
    pub fn ramunderflow(&self) -> RamunderflowR {
        RamunderflowR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event DMASEQ0END"]
    #[inline(always)]
    pub fn dmaseq0end(&self) -> Dmaseq0endR {
        Dmaseq0endR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event DMASEQ0READY"]
    #[inline(always)]
    pub fn dmaseq0ready(&self) -> Dmaseq0readyR {
        Dmaseq0readyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for event DMASEQ0BUSERROR"]
    #[inline(always)]
    pub fn dmaseq0buserror(&self) -> Dmaseq0buserrorR {
        Dmaseq0buserrorR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable or disable interrupt for event DMASEQ1END"]
    #[inline(always)]
    pub fn dmaseq1end(&self) -> Dmaseq1endR {
        Dmaseq1endR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for event DMASEQ1READY"]
    #[inline(always)]
    pub fn dmaseq1ready(&self) -> Dmaseq1readyR {
        Dmaseq1readyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event DMASEQ1BUSERROR"]
    #[inline(always)]
    pub fn dmaseq1buserror(&self) -> Dmaseq1buserrorR {
        Dmaseq1buserrorR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable or disable interrupt for event COMPAREMATCH\\[0\\]"]
    #[inline(always)]
    pub fn comparematch0(&self) -> Comparematch0R {
        Comparematch0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable or disable interrupt for event COMPAREMATCH\\[1\\]"]
    #[inline(always)]
    pub fn comparematch1(&self) -> Comparematch1R {
        Comparematch1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable or disable interrupt for event COMPAREMATCH\\[2\\]"]
    #[inline(always)]
    pub fn comparematch2(&self) -> Comparematch2R {
        Comparematch2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable or disable interrupt for event COMPAREMATCH\\[3\\]"]
    #[inline(always)]
    pub fn comparematch3(&self) -> Comparematch3R {
        Comparematch3R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable or disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&mut self) -> StoppedW<'_, IntenSpec> {
        StoppedW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event SEQSTARTED\\[0\\]"]
    #[inline(always)]
    pub fn seqstarted0(&mut self) -> Seqstarted0W<'_, IntenSpec> {
        Seqstarted0W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event SEQSTARTED\\[1\\]"]
    #[inline(always)]
    pub fn seqstarted1(&mut self) -> Seqstarted1W<'_, IntenSpec> {
        Seqstarted1W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event SEQEND\\[0\\]"]
    #[inline(always)]
    pub fn seqend0(&mut self) -> Seqend0W<'_, IntenSpec> {
        Seqend0W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event SEQEND\\[1\\]"]
    #[inline(always)]
    pub fn seqend1(&mut self) -> Seqend1W<'_, IntenSpec> {
        Seqend1W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event PWMPERIODEND"]
    #[inline(always)]
    pub fn pwmperiodend(&mut self) -> PwmperiodendW<'_, IntenSpec> {
        PwmperiodendW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event LOOPSDONE"]
    #[inline(always)]
    pub fn loopsdone(&mut self) -> LoopsdoneW<'_, IntenSpec> {
        LoopsdoneW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event RAMUNDERFLOW"]
    #[inline(always)]
    pub fn ramunderflow(&mut self) -> RamunderflowW<'_, IntenSpec> {
        RamunderflowW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event DMASEQ0END"]
    #[inline(always)]
    pub fn dmaseq0end(&mut self) -> Dmaseq0endW<'_, IntenSpec> {
        Dmaseq0endW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event DMASEQ0READY"]
    #[inline(always)]
    pub fn dmaseq0ready(&mut self) -> Dmaseq0readyW<'_, IntenSpec> {
        Dmaseq0readyW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for event DMASEQ0BUSERROR"]
    #[inline(always)]
    pub fn dmaseq0buserror(&mut self) -> Dmaseq0buserrorW<'_, IntenSpec> {
        Dmaseq0buserrorW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable or disable interrupt for event DMASEQ1END"]
    #[inline(always)]
    pub fn dmaseq1end(&mut self) -> Dmaseq1endW<'_, IntenSpec> {
        Dmaseq1endW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for event DMASEQ1READY"]
    #[inline(always)]
    pub fn dmaseq1ready(&mut self) -> Dmaseq1readyW<'_, IntenSpec> {
        Dmaseq1readyW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event DMASEQ1BUSERROR"]
    #[inline(always)]
    pub fn dmaseq1buserror(&mut self) -> Dmaseq1buserrorW<'_, IntenSpec> {
        Dmaseq1buserrorW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable or disable interrupt for event COMPAREMATCH\\[0\\]"]
    #[inline(always)]
    pub fn comparematch0(&mut self) -> Comparematch0W<'_, IntenSpec> {
        Comparematch0W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable or disable interrupt for event COMPAREMATCH\\[1\\]"]
    #[inline(always)]
    pub fn comparematch1(&mut self) -> Comparematch1W<'_, IntenSpec> {
        Comparematch1W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable or disable interrupt for event COMPAREMATCH\\[2\\]"]
    #[inline(always)]
    pub fn comparematch2(&mut self) -> Comparematch2W<'_, IntenSpec> {
        Comparematch2W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable or disable interrupt for event COMPAREMATCH\\[3\\]"]
    #[inline(always)]
    pub fn comparematch3(&mut self) -> Comparematch3W<'_, IntenSpec> {
        Comparematch3W::new(self, 18)
    }
}
#[doc = "Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
