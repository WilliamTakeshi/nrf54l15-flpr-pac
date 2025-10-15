#[doc = "Register `CHENCLR` reader"]
pub type R = crate::R<ChenclrSpec>;
#[doc = "Register `CHENCLR` writer"]
pub type W = crate::W<ChenclrSpec>;
#[doc = "Channel 0 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch0> for bool {
    #[inline(always)]
    fn from(variant: Ch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` reader - Channel 0 enable clear register. Writing 0 has no effect."]
pub type Ch0R = crate::BitReader<Ch0>;
impl Ch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0 {
        match self.bits {
            false => Ch0::Disabled,
            true => Ch0::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch0::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch0::Enabled
    }
}
#[doc = "Channel 0 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch0WO> for bool {
    #[inline(always)]
    fn from(variant: Ch0WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` writer - Channel 0 enable clear register. Writing 0 has no effect."]
pub type Ch0W<'a, REG> = crate::BitWriter1C<'a, REG, Ch0WO>;
impl<'a, REG> Ch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0WO::Clear)
    }
}
#[doc = "Channel 1 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch1> for bool {
    #[inline(always)]
    fn from(variant: Ch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` reader - Channel 1 enable clear register. Writing 0 has no effect."]
pub type Ch1R = crate::BitReader<Ch1>;
impl Ch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1 {
        match self.bits {
            false => Ch1::Disabled,
            true => Ch1::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch1::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch1::Enabled
    }
}
#[doc = "Channel 1 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch1WO> for bool {
    #[inline(always)]
    fn from(variant: Ch1WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` writer - Channel 1 enable clear register. Writing 0 has no effect."]
pub type Ch1W<'a, REG> = crate::BitWriter1C<'a, REG, Ch1WO>;
impl<'a, REG> Ch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1WO::Clear)
    }
}
#[doc = "Channel 2 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch2> for bool {
    #[inline(always)]
    fn from(variant: Ch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` reader - Channel 2 enable clear register. Writing 0 has no effect."]
pub type Ch2R = crate::BitReader<Ch2>;
impl Ch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2 {
        match self.bits {
            false => Ch2::Disabled,
            true => Ch2::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch2::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch2::Enabled
    }
}
#[doc = "Channel 2 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch2WO> for bool {
    #[inline(always)]
    fn from(variant: Ch2WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` writer - Channel 2 enable clear register. Writing 0 has no effect."]
pub type Ch2W<'a, REG> = crate::BitWriter1C<'a, REG, Ch2WO>;
impl<'a, REG> Ch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2WO::Clear)
    }
}
#[doc = "Channel 3 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch3> for bool {
    #[inline(always)]
    fn from(variant: Ch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` reader - Channel 3 enable clear register. Writing 0 has no effect."]
pub type Ch3R = crate::BitReader<Ch3>;
impl Ch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3 {
        match self.bits {
            false => Ch3::Disabled,
            true => Ch3::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch3::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch3::Enabled
    }
}
#[doc = "Channel 3 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch3WO> for bool {
    #[inline(always)]
    fn from(variant: Ch3WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` writer - Channel 3 enable clear register. Writing 0 has no effect."]
pub type Ch3W<'a, REG> = crate::BitWriter1C<'a, REG, Ch3WO>;
impl<'a, REG> Ch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3WO::Clear)
    }
}
#[doc = "Channel 4 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch4 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch4> for bool {
    #[inline(always)]
    fn from(variant: Ch4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4` reader - Channel 4 enable clear register. Writing 0 has no effect."]
pub type Ch4R = crate::BitReader<Ch4>;
impl Ch4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch4 {
        match self.bits {
            false => Ch4::Disabled,
            true => Ch4::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch4::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch4::Enabled
    }
}
#[doc = "Channel 4 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch4WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch4WO> for bool {
    #[inline(always)]
    fn from(variant: Ch4WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4` writer - Channel 4 enable clear register. Writing 0 has no effect."]
pub type Ch4W<'a, REG> = crate::BitWriter1C<'a, REG, Ch4WO>;
impl<'a, REG> Ch4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4WO::Clear)
    }
}
#[doc = "Channel 5 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch5 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch5> for bool {
    #[inline(always)]
    fn from(variant: Ch5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5` reader - Channel 5 enable clear register. Writing 0 has no effect."]
pub type Ch5R = crate::BitReader<Ch5>;
impl Ch5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch5 {
        match self.bits {
            false => Ch5::Disabled,
            true => Ch5::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch5::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch5::Enabled
    }
}
#[doc = "Channel 5 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch5WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch5WO> for bool {
    #[inline(always)]
    fn from(variant: Ch5WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5` writer - Channel 5 enable clear register. Writing 0 has no effect."]
pub type Ch5W<'a, REG> = crate::BitWriter1C<'a, REG, Ch5WO>;
impl<'a, REG> Ch5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5WO::Clear)
    }
}
#[doc = "Channel 6 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch6 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch6> for bool {
    #[inline(always)]
    fn from(variant: Ch6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6` reader - Channel 6 enable clear register. Writing 0 has no effect."]
pub type Ch6R = crate::BitReader<Ch6>;
impl Ch6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch6 {
        match self.bits {
            false => Ch6::Disabled,
            true => Ch6::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch6::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch6::Enabled
    }
}
#[doc = "Channel 6 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch6WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch6WO> for bool {
    #[inline(always)]
    fn from(variant: Ch6WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6` writer - Channel 6 enable clear register. Writing 0 has no effect."]
pub type Ch6W<'a, REG> = crate::BitWriter1C<'a, REG, Ch6WO>;
impl<'a, REG> Ch6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6WO::Clear)
    }
}
#[doc = "Channel 7 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch7 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch7> for bool {
    #[inline(always)]
    fn from(variant: Ch7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7` reader - Channel 7 enable clear register. Writing 0 has no effect."]
pub type Ch7R = crate::BitReader<Ch7>;
impl Ch7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch7 {
        match self.bits {
            false => Ch7::Disabled,
            true => Ch7::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch7::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch7::Enabled
    }
}
#[doc = "Channel 7 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch7WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch7WO> for bool {
    #[inline(always)]
    fn from(variant: Ch7WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7` writer - Channel 7 enable clear register. Writing 0 has no effect."]
pub type Ch7W<'a, REG> = crate::BitWriter1C<'a, REG, Ch7WO>;
impl<'a, REG> Ch7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7WO::Clear)
    }
}
#[doc = "Channel 8 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch8 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch8> for bool {
    #[inline(always)]
    fn from(variant: Ch8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH8` reader - Channel 8 enable clear register. Writing 0 has no effect."]
pub type Ch8R = crate::BitReader<Ch8>;
impl Ch8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch8 {
        match self.bits {
            false => Ch8::Disabled,
            true => Ch8::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch8::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch8::Enabled
    }
}
#[doc = "Channel 8 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch8WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch8WO> for bool {
    #[inline(always)]
    fn from(variant: Ch8WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH8` writer - Channel 8 enable clear register. Writing 0 has no effect."]
pub type Ch8W<'a, REG> = crate::BitWriter1C<'a, REG, Ch8WO>;
impl<'a, REG> Ch8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch8WO::Clear)
    }
}
#[doc = "Channel 9 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch9 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch9> for bool {
    #[inline(always)]
    fn from(variant: Ch9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH9` reader - Channel 9 enable clear register. Writing 0 has no effect."]
pub type Ch9R = crate::BitReader<Ch9>;
impl Ch9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch9 {
        match self.bits {
            false => Ch9::Disabled,
            true => Ch9::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch9::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch9::Enabled
    }
}
#[doc = "Channel 9 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch9WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch9WO> for bool {
    #[inline(always)]
    fn from(variant: Ch9WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH9` writer - Channel 9 enable clear register. Writing 0 has no effect."]
pub type Ch9W<'a, REG> = crate::BitWriter1C<'a, REG, Ch9WO>;
impl<'a, REG> Ch9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch9WO::Clear)
    }
}
#[doc = "Channel 10 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch10 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch10> for bool {
    #[inline(always)]
    fn from(variant: Ch10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH10` reader - Channel 10 enable clear register. Writing 0 has no effect."]
pub type Ch10R = crate::BitReader<Ch10>;
impl Ch10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch10 {
        match self.bits {
            false => Ch10::Disabled,
            true => Ch10::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch10::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch10::Enabled
    }
}
#[doc = "Channel 10 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch10WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch10WO> for bool {
    #[inline(always)]
    fn from(variant: Ch10WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH10` writer - Channel 10 enable clear register. Writing 0 has no effect."]
pub type Ch10W<'a, REG> = crate::BitWriter1C<'a, REG, Ch10WO>;
impl<'a, REG> Ch10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch10WO::Clear)
    }
}
#[doc = "Channel 11 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch11 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch11> for bool {
    #[inline(always)]
    fn from(variant: Ch11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH11` reader - Channel 11 enable clear register. Writing 0 has no effect."]
pub type Ch11R = crate::BitReader<Ch11>;
impl Ch11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch11 {
        match self.bits {
            false => Ch11::Disabled,
            true => Ch11::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch11::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch11::Enabled
    }
}
#[doc = "Channel 11 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch11WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch11WO> for bool {
    #[inline(always)]
    fn from(variant: Ch11WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH11` writer - Channel 11 enable clear register. Writing 0 has no effect."]
pub type Ch11W<'a, REG> = crate::BitWriter1C<'a, REG, Ch11WO>;
impl<'a, REG> Ch11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch11WO::Clear)
    }
}
#[doc = "Channel 12 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch12 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch12> for bool {
    #[inline(always)]
    fn from(variant: Ch12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH12` reader - Channel 12 enable clear register. Writing 0 has no effect."]
pub type Ch12R = crate::BitReader<Ch12>;
impl Ch12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch12 {
        match self.bits {
            false => Ch12::Disabled,
            true => Ch12::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch12::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch12::Enabled
    }
}
#[doc = "Channel 12 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch12WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch12WO> for bool {
    #[inline(always)]
    fn from(variant: Ch12WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH12` writer - Channel 12 enable clear register. Writing 0 has no effect."]
pub type Ch12W<'a, REG> = crate::BitWriter1C<'a, REG, Ch12WO>;
impl<'a, REG> Ch12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch12WO::Clear)
    }
}
#[doc = "Channel 13 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch13 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch13> for bool {
    #[inline(always)]
    fn from(variant: Ch13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH13` reader - Channel 13 enable clear register. Writing 0 has no effect."]
pub type Ch13R = crate::BitReader<Ch13>;
impl Ch13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch13 {
        match self.bits {
            false => Ch13::Disabled,
            true => Ch13::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch13::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch13::Enabled
    }
}
#[doc = "Channel 13 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch13WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch13WO> for bool {
    #[inline(always)]
    fn from(variant: Ch13WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH13` writer - Channel 13 enable clear register. Writing 0 has no effect."]
pub type Ch13W<'a, REG> = crate::BitWriter1C<'a, REG, Ch13WO>;
impl<'a, REG> Ch13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch13WO::Clear)
    }
}
#[doc = "Channel 14 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch14 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch14> for bool {
    #[inline(always)]
    fn from(variant: Ch14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH14` reader - Channel 14 enable clear register. Writing 0 has no effect."]
pub type Ch14R = crate::BitReader<Ch14>;
impl Ch14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch14 {
        match self.bits {
            false => Ch14::Disabled,
            true => Ch14::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch14::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch14::Enabled
    }
}
#[doc = "Channel 14 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch14WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch14WO> for bool {
    #[inline(always)]
    fn from(variant: Ch14WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH14` writer - Channel 14 enable clear register. Writing 0 has no effect."]
pub type Ch14W<'a, REG> = crate::BitWriter1C<'a, REG, Ch14WO>;
impl<'a, REG> Ch14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch14WO::Clear)
    }
}
#[doc = "Channel 15 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch15 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch15> for bool {
    #[inline(always)]
    fn from(variant: Ch15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH15` reader - Channel 15 enable clear register. Writing 0 has no effect."]
pub type Ch15R = crate::BitReader<Ch15>;
impl Ch15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch15 {
        match self.bits {
            false => Ch15::Disabled,
            true => Ch15::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch15::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch15::Enabled
    }
}
#[doc = "Channel 15 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch15WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch15WO> for bool {
    #[inline(always)]
    fn from(variant: Ch15WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH15` writer - Channel 15 enable clear register. Writing 0 has no effect."]
pub type Ch15W<'a, REG> = crate::BitWriter1C<'a, REG, Ch15WO>;
impl<'a, REG> Ch15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch15WO::Clear)
    }
}
#[doc = "Channel 16 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch16 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch16> for bool {
    #[inline(always)]
    fn from(variant: Ch16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH16` reader - Channel 16 enable clear register. Writing 0 has no effect."]
pub type Ch16R = crate::BitReader<Ch16>;
impl Ch16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch16 {
        match self.bits {
            false => Ch16::Disabled,
            true => Ch16::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch16::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch16::Enabled
    }
}
#[doc = "Channel 16 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch16WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch16WO> for bool {
    #[inline(always)]
    fn from(variant: Ch16WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH16` writer - Channel 16 enable clear register. Writing 0 has no effect."]
pub type Ch16W<'a, REG> = crate::BitWriter1C<'a, REG, Ch16WO>;
impl<'a, REG> Ch16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch16WO::Clear)
    }
}
#[doc = "Channel 17 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch17 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch17> for bool {
    #[inline(always)]
    fn from(variant: Ch17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH17` reader - Channel 17 enable clear register. Writing 0 has no effect."]
pub type Ch17R = crate::BitReader<Ch17>;
impl Ch17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch17 {
        match self.bits {
            false => Ch17::Disabled,
            true => Ch17::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch17::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch17::Enabled
    }
}
#[doc = "Channel 17 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch17WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch17WO> for bool {
    #[inline(always)]
    fn from(variant: Ch17WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH17` writer - Channel 17 enable clear register. Writing 0 has no effect."]
pub type Ch17W<'a, REG> = crate::BitWriter1C<'a, REG, Ch17WO>;
impl<'a, REG> Ch17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch17WO::Clear)
    }
}
#[doc = "Channel 18 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch18 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch18> for bool {
    #[inline(always)]
    fn from(variant: Ch18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH18` reader - Channel 18 enable clear register. Writing 0 has no effect."]
pub type Ch18R = crate::BitReader<Ch18>;
impl Ch18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch18 {
        match self.bits {
            false => Ch18::Disabled,
            true => Ch18::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch18::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch18::Enabled
    }
}
#[doc = "Channel 18 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch18WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch18WO> for bool {
    #[inline(always)]
    fn from(variant: Ch18WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH18` writer - Channel 18 enable clear register. Writing 0 has no effect."]
pub type Ch18W<'a, REG> = crate::BitWriter1C<'a, REG, Ch18WO>;
impl<'a, REG> Ch18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch18WO::Clear)
    }
}
#[doc = "Channel 19 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch19 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch19> for bool {
    #[inline(always)]
    fn from(variant: Ch19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH19` reader - Channel 19 enable clear register. Writing 0 has no effect."]
pub type Ch19R = crate::BitReader<Ch19>;
impl Ch19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch19 {
        match self.bits {
            false => Ch19::Disabled,
            true => Ch19::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch19::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch19::Enabled
    }
}
#[doc = "Channel 19 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch19WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch19WO> for bool {
    #[inline(always)]
    fn from(variant: Ch19WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH19` writer - Channel 19 enable clear register. Writing 0 has no effect."]
pub type Ch19W<'a, REG> = crate::BitWriter1C<'a, REG, Ch19WO>;
impl<'a, REG> Ch19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch19WO::Clear)
    }
}
#[doc = "Channel 20 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch20 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch20> for bool {
    #[inline(always)]
    fn from(variant: Ch20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH20` reader - Channel 20 enable clear register. Writing 0 has no effect."]
pub type Ch20R = crate::BitReader<Ch20>;
impl Ch20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch20 {
        match self.bits {
            false => Ch20::Disabled,
            true => Ch20::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch20::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch20::Enabled
    }
}
#[doc = "Channel 20 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch20WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch20WO> for bool {
    #[inline(always)]
    fn from(variant: Ch20WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH20` writer - Channel 20 enable clear register. Writing 0 has no effect."]
pub type Ch20W<'a, REG> = crate::BitWriter1C<'a, REG, Ch20WO>;
impl<'a, REG> Ch20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch20WO::Clear)
    }
}
#[doc = "Channel 21 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch21 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch21> for bool {
    #[inline(always)]
    fn from(variant: Ch21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH21` reader - Channel 21 enable clear register. Writing 0 has no effect."]
pub type Ch21R = crate::BitReader<Ch21>;
impl Ch21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch21 {
        match self.bits {
            false => Ch21::Disabled,
            true => Ch21::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch21::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch21::Enabled
    }
}
#[doc = "Channel 21 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch21WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch21WO> for bool {
    #[inline(always)]
    fn from(variant: Ch21WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH21` writer - Channel 21 enable clear register. Writing 0 has no effect."]
pub type Ch21W<'a, REG> = crate::BitWriter1C<'a, REG, Ch21WO>;
impl<'a, REG> Ch21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch21WO::Clear)
    }
}
#[doc = "Channel 22 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch22 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch22> for bool {
    #[inline(always)]
    fn from(variant: Ch22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH22` reader - Channel 22 enable clear register. Writing 0 has no effect."]
pub type Ch22R = crate::BitReader<Ch22>;
impl Ch22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch22 {
        match self.bits {
            false => Ch22::Disabled,
            true => Ch22::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch22::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch22::Enabled
    }
}
#[doc = "Channel 22 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch22WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch22WO> for bool {
    #[inline(always)]
    fn from(variant: Ch22WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH22` writer - Channel 22 enable clear register. Writing 0 has no effect."]
pub type Ch22W<'a, REG> = crate::BitWriter1C<'a, REG, Ch22WO>;
impl<'a, REG> Ch22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch22WO::Clear)
    }
}
#[doc = "Channel 23 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch23 {
    #[doc = "0: Read: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Read: Channel enabled"]
    Enabled = 1,
}
impl From<Ch23> for bool {
    #[inline(always)]
    fn from(variant: Ch23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH23` reader - Channel 23 enable clear register. Writing 0 has no effect."]
pub type Ch23R = crate::BitReader<Ch23>;
impl Ch23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch23 {
        match self.bits {
            false => Ch23::Disabled,
            true => Ch23::Enabled,
        }
    }
    #[doc = "Read: Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch23::Disabled
    }
    #[doc = "Read: Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch23::Enabled
    }
}
#[doc = "Channel 23 enable clear register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch23WO {
    #[doc = "1: Write: Disable channel"]
    Clear = 1,
}
impl From<Ch23WO> for bool {
    #[inline(always)]
    fn from(variant: Ch23WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH23` writer - Channel 23 enable clear register. Writing 0 has no effect."]
pub type Ch23W<'a, REG> = crate::BitWriter1C<'a, REG, Ch23WO>;
impl<'a, REG> Ch23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: Disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ch23WO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch8(&self) -> Ch8R {
        Ch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch9(&self) -> Ch9R {
        Ch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch10(&self) -> Ch10R {
        Ch10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch11(&self) -> Ch11R {
        Ch11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 12 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch12(&self) -> Ch12R {
        Ch12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 13 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch13(&self) -> Ch13R {
        Ch13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 14 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch14(&self) -> Ch14R {
        Ch14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 15 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch15(&self) -> Ch15R {
        Ch15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 16 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch16(&self) -> Ch16R {
        Ch16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 17 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch17(&self) -> Ch17R {
        Ch17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 18 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch18(&self) -> Ch18R {
        Ch18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 19 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch19(&self) -> Ch19R {
        Ch19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel 20 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch20(&self) -> Ch20R {
        Ch20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel 21 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch21(&self) -> Ch21R {
        Ch21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel 22 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch22(&self) -> Ch22R {
        Ch22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 23 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch23(&self) -> Ch23R {
        Ch23R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<'_, ChenclrSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, ChenclrSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, ChenclrSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<'_, ChenclrSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch4(&mut self) -> Ch4W<'_, ChenclrSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch5(&mut self) -> Ch5W<'_, ChenclrSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch6(&mut self) -> Ch6W<'_, ChenclrSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch7(&mut self) -> Ch7W<'_, ChenclrSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 8 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch8(&mut self) -> Ch8W<'_, ChenclrSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 9 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch9(&mut self) -> Ch9W<'_, ChenclrSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 10 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch10(&mut self) -> Ch10W<'_, ChenclrSpec> {
        Ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 11 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch11(&mut self) -> Ch11W<'_, ChenclrSpec> {
        Ch11W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 12 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch12(&mut self) -> Ch12W<'_, ChenclrSpec> {
        Ch12W::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 13 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch13(&mut self) -> Ch13W<'_, ChenclrSpec> {
        Ch13W::new(self, 13)
    }
    #[doc = "Bit 14 - Channel 14 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch14(&mut self) -> Ch14W<'_, ChenclrSpec> {
        Ch14W::new(self, 14)
    }
    #[doc = "Bit 15 - Channel 15 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch15(&mut self) -> Ch15W<'_, ChenclrSpec> {
        Ch15W::new(self, 15)
    }
    #[doc = "Bit 16 - Channel 16 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch16(&mut self) -> Ch16W<'_, ChenclrSpec> {
        Ch16W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel 17 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch17(&mut self) -> Ch17W<'_, ChenclrSpec> {
        Ch17W::new(self, 17)
    }
    #[doc = "Bit 18 - Channel 18 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch18(&mut self) -> Ch18W<'_, ChenclrSpec> {
        Ch18W::new(self, 18)
    }
    #[doc = "Bit 19 - Channel 19 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch19(&mut self) -> Ch19W<'_, ChenclrSpec> {
        Ch19W::new(self, 19)
    }
    #[doc = "Bit 20 - Channel 20 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch20(&mut self) -> Ch20W<'_, ChenclrSpec> {
        Ch20W::new(self, 20)
    }
    #[doc = "Bit 21 - Channel 21 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch21(&mut self) -> Ch21W<'_, ChenclrSpec> {
        Ch21W::new(self, 21)
    }
    #[doc = "Bit 22 - Channel 22 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch22(&mut self) -> Ch22W<'_, ChenclrSpec> {
        Ch22W::new(self, 22)
    }
    #[doc = "Bit 23 - Channel 23 enable clear register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ch23(&mut self) -> Ch23W<'_, ChenclrSpec> {
        Ch23W::new(self, 23)
    }
}
#[doc = "Channel enable clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`chenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChenclrSpec;
impl crate::RegisterSpec for ChenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chenclr::R`](R) reader structure"]
impl crate::Readable for ChenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`chenclr::W`](W) writer structure"]
impl crate::Writable for ChenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x00ff_ffff;
}
#[doc = "`reset()` method sets CHENCLR to value 0"]
impl crate::Resettable for ChenclrSpec {}
