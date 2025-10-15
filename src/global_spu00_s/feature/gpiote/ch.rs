#[doc = "Register `CH[%s]` reader"]
pub type R = crate::R<ChSpec>;
#[doc = "Register `CH[%s]` writer"]
pub type W = crate::W<ChSpec>;
#[doc = "SECATTR feature\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secattr {
    #[doc = "0: Feature is available for non-secure usage"]
    NonSecure = 0,
    #[doc = "1: Feature is reserved for secure usage"]
    Secure = 1,
}
impl From<Secattr> for bool {
    #[inline(always)]
    fn from(variant: Secattr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECATTR` reader - SECATTR feature"]
pub type SecattrR = crate::BitReader<Secattr>;
impl SecattrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Secattr {
        match self.bits {
            false => Secattr::NonSecure,
            true => Secattr::Secure,
        }
    }
    #[doc = "Feature is available for non-secure usage"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Secattr::NonSecure
    }
    #[doc = "Feature is reserved for secure usage"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Secattr::Secure
    }
}
#[doc = "Field `SECATTR` writer - SECATTR feature"]
pub type SecattrW<'a, REG> = crate::BitWriter<'a, REG, Secattr>;
impl<'a, REG> SecattrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Feature is available for non-secure usage"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Secattr::NonSecure)
    }
    #[doc = "Feature is reserved for secure usage"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Secattr::Secure)
    }
}
#[doc = "LOCK feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: Feature permissions can be updated"]
    Unlocked = 0,
    #[doc = "1: Feature permissions can not be changed until the next reset"]
    Locked = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - LOCK feature"]
pub type LockR = crate::BitReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            false => Lock::Unlocked,
            true => Lock::Locked,
        }
    }
    #[doc = "Feature permissions can be updated"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lock::Unlocked
    }
    #[doc = "Feature permissions can not be changed until the next reset"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lock::Locked
    }
}
#[doc = "Field `LOCK` writer - LOCK feature"]
pub type LockW<'a, REG> = crate::BitWriter1S<'a, REG, Lock>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Feature permissions can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Unlocked)
    }
    #[doc = "Feature permissions can not be changed until the next reset"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Locked)
    }
}
impl R {
    #[doc = "Bit 4 - SECATTR feature"]
    #[inline(always)]
    pub fn secattr(&self) -> SecattrR {
        SecattrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - LOCK feature"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - SECATTR feature"]
    #[inline(always)]
    pub fn secattr(&mut self) -> SecattrW<'_, ChSpec> {
        SecattrW::new(self, 4)
    }
    #[doc = "Bit 8 - LOCK feature"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, ChSpec> {
        LockW::new(self, 8)
    }
}
#[doc = "Description collection: Configuration of features for channel o of GPIOTE\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChSpec;
impl crate::RegisterSpec for ChSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch::R`](R) reader structure"]
impl crate::Readable for ChSpec {}
#[doc = "`write(|w| ..)` method takes [`ch::W`](W) writer structure"]
impl crate::Writable for ChSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0100;
}
#[doc = "`reset()` method sets CH[%s] to value 0x0010_0010"]
impl crate::Resettable for ChSpec {
    const RESET_VALUE: u32 = 0x0010_0010;
}
