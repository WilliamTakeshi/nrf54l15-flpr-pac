#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to disable interrupt for event TRIGGERED\\[16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered16 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Triggered16> for bool {
    #[inline(always)]
    fn from(variant: Triggered16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED16` reader - Write '1' to disable interrupt for event TRIGGERED\\[16\\]"]
pub type Triggered16R = crate::BitReader<Triggered16>;
impl Triggered16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered16 {
        match self.bits {
            false => Triggered16::Disabled,
            true => Triggered16::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered16::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered16::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event TRIGGERED\\[16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered16WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Triggered16WO> for bool {
    #[inline(always)]
    fn from(variant: Triggered16WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED16` writer - Write '1' to disable interrupt for event TRIGGERED\\[16\\]"]
pub type Triggered16W<'a, REG> = crate::BitWriter<'a, REG, Triggered16WO>;
impl<'a, REG> Triggered16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered16WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event TRIGGERED\\[17\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered17 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Triggered17> for bool {
    #[inline(always)]
    fn from(variant: Triggered17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED17` reader - Write '1' to disable interrupt for event TRIGGERED\\[17\\]"]
pub type Triggered17R = crate::BitReader<Triggered17>;
impl Triggered17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered17 {
        match self.bits {
            false => Triggered17::Disabled,
            true => Triggered17::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered17::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered17::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event TRIGGERED\\[17\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered17WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Triggered17WO> for bool {
    #[inline(always)]
    fn from(variant: Triggered17WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED17` writer - Write '1' to disable interrupt for event TRIGGERED\\[17\\]"]
pub type Triggered17W<'a, REG> = crate::BitWriter<'a, REG, Triggered17WO>;
impl<'a, REG> Triggered17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered17WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event TRIGGERED\\[18\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered18 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Triggered18> for bool {
    #[inline(always)]
    fn from(variant: Triggered18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED18` reader - Write '1' to disable interrupt for event TRIGGERED\\[18\\]"]
pub type Triggered18R = crate::BitReader<Triggered18>;
impl Triggered18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered18 {
        match self.bits {
            false => Triggered18::Disabled,
            true => Triggered18::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered18::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered18::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event TRIGGERED\\[18\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered18WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Triggered18WO> for bool {
    #[inline(always)]
    fn from(variant: Triggered18WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED18` writer - Write '1' to disable interrupt for event TRIGGERED\\[18\\]"]
pub type Triggered18W<'a, REG> = crate::BitWriter<'a, REG, Triggered18WO>;
impl<'a, REG> Triggered18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered18WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event TRIGGERED\\[19\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered19 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Triggered19> for bool {
    #[inline(always)]
    fn from(variant: Triggered19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED19` reader - Write '1' to disable interrupt for event TRIGGERED\\[19\\]"]
pub type Triggered19R = crate::BitReader<Triggered19>;
impl Triggered19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered19 {
        match self.bits {
            false => Triggered19::Disabled,
            true => Triggered19::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered19::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered19::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event TRIGGERED\\[19\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered19WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Triggered19WO> for bool {
    #[inline(always)]
    fn from(variant: Triggered19WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED19` writer - Write '1' to disable interrupt for event TRIGGERED\\[19\\]"]
pub type Triggered19W<'a, REG> = crate::BitWriter<'a, REG, Triggered19WO>;
impl<'a, REG> Triggered19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered19WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event TRIGGERED\\[20\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered20 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Triggered20> for bool {
    #[inline(always)]
    fn from(variant: Triggered20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED20` reader - Write '1' to disable interrupt for event TRIGGERED\\[20\\]"]
pub type Triggered20R = crate::BitReader<Triggered20>;
impl Triggered20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered20 {
        match self.bits {
            false => Triggered20::Disabled,
            true => Triggered20::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered20::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered20::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event TRIGGERED\\[20\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered20WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Triggered20WO> for bool {
    #[inline(always)]
    fn from(variant: Triggered20WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED20` writer - Write '1' to disable interrupt for event TRIGGERED\\[20\\]"]
pub type Triggered20W<'a, REG> = crate::BitWriter<'a, REG, Triggered20WO>;
impl<'a, REG> Triggered20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered20WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event TRIGGERED\\[21\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered21 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Triggered21> for bool {
    #[inline(always)]
    fn from(variant: Triggered21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED21` reader - Write '1' to disable interrupt for event TRIGGERED\\[21\\]"]
pub type Triggered21R = crate::BitReader<Triggered21>;
impl Triggered21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered21 {
        match self.bits {
            false => Triggered21::Disabled,
            true => Triggered21::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered21::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered21::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event TRIGGERED\\[21\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered21WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Triggered21WO> for bool {
    #[inline(always)]
    fn from(variant: Triggered21WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED21` writer - Write '1' to disable interrupt for event TRIGGERED\\[21\\]"]
pub type Triggered21W<'a, REG> = crate::BitWriter<'a, REG, Triggered21WO>;
impl<'a, REG> Triggered21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered21WO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event TRIGGERED\\[22\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered22 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Triggered22> for bool {
    #[inline(always)]
    fn from(variant: Triggered22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED22` reader - Write '1' to disable interrupt for event TRIGGERED\\[22\\]"]
pub type Triggered22R = crate::BitReader<Triggered22>;
impl Triggered22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered22 {
        match self.bits {
            false => Triggered22::Disabled,
            true => Triggered22::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered22::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered22::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event TRIGGERED\\[22\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered22WO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<Triggered22WO> for bool {
    #[inline(always)]
    fn from(variant: Triggered22WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED22` writer - Write '1' to disable interrupt for event TRIGGERED\\[22\\]"]
pub type Triggered22W<'a, REG> = crate::BitWriter<'a, REG, Triggered22WO>;
impl<'a, REG> Triggered22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered22WO::Clear)
    }
}
impl R {
    #[doc = "Bit 16 - Write '1' to disable interrupt for event TRIGGERED\\[16\\]"]
    #[inline(always)]
    pub fn triggered16(&self) -> Triggered16R {
        Triggered16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for event TRIGGERED\\[17\\]"]
    #[inline(always)]
    pub fn triggered17(&self) -> Triggered17R {
        Triggered17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event TRIGGERED\\[18\\]"]
    #[inline(always)]
    pub fn triggered18(&self) -> Triggered18R {
        Triggered18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event TRIGGERED\\[19\\]"]
    #[inline(always)]
    pub fn triggered19(&self) -> Triggered19R {
        Triggered19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event TRIGGERED\\[20\\]"]
    #[inline(always)]
    pub fn triggered20(&self) -> Triggered20R {
        Triggered20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write '1' to disable interrupt for event TRIGGERED\\[21\\]"]
    #[inline(always)]
    pub fn triggered21(&self) -> Triggered21R {
        Triggered21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Write '1' to disable interrupt for event TRIGGERED\\[22\\]"]
    #[inline(always)]
    pub fn triggered22(&self) -> Triggered22R {
        Triggered22R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Write '1' to disable interrupt for event TRIGGERED\\[16\\]"]
    #[inline(always)]
    pub fn triggered16(&mut self) -> Triggered16W<'_, IntenclrSpec> {
        Triggered16W::new(self, 16)
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for event TRIGGERED\\[17\\]"]
    #[inline(always)]
    pub fn triggered17(&mut self) -> Triggered17W<'_, IntenclrSpec> {
        Triggered17W::new(self, 17)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event TRIGGERED\\[18\\]"]
    #[inline(always)]
    pub fn triggered18(&mut self) -> Triggered18W<'_, IntenclrSpec> {
        Triggered18W::new(self, 18)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event TRIGGERED\\[19\\]"]
    #[inline(always)]
    pub fn triggered19(&mut self) -> Triggered19W<'_, IntenclrSpec> {
        Triggered19W::new(self, 19)
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event TRIGGERED\\[20\\]"]
    #[inline(always)]
    pub fn triggered20(&mut self) -> Triggered20W<'_, IntenclrSpec> {
        Triggered20W::new(self, 20)
    }
    #[doc = "Bit 21 - Write '1' to disable interrupt for event TRIGGERED\\[21\\]"]
    #[inline(always)]
    pub fn triggered21(&mut self) -> Triggered21W<'_, IntenclrSpec> {
        Triggered21W::new(self, 21)
    }
    #[doc = "Bit 22 - Write '1' to disable interrupt for event TRIGGERED\\[22\\]"]
    #[inline(always)]
    pub fn triggered22(&mut self) -> Triggered22W<'_, IntenclrSpec> {
        Triggered22W::new(self, 22)
    }
}
#[doc = "Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
