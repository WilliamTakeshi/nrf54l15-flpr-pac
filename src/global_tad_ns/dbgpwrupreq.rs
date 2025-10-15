#[doc = "Register `DBGPWRUPREQ` reader"]
pub type R = crate::R<DbgpwrupreqSpec>;
#[doc = "Register `DBGPWRUPREQ` writer"]
pub type W = crate::W<DbgpwrupreqSpec>;
#[doc = "Activate power-up request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Active {
    #[doc = "0: Power-up request not active"]
    NotActive = 0,
    #[doc = "1: Power-up request active"]
    Active = 1,
}
impl From<Active> for bool {
    #[inline(always)]
    fn from(variant: Active) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVE` reader - Activate power-up request"]
pub type ActiveR = crate::BitReader<Active>;
impl ActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Active {
        match self.bits {
            false => Active::NotActive,
            true => Active::Active,
        }
    }
    #[doc = "Power-up request not active"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == Active::NotActive
    }
    #[doc = "Power-up request active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Active::Active
    }
}
#[doc = "Field `ACTIVE` writer - Activate power-up request"]
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG, Active>;
impl<'a, REG> ActiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power-up request not active"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(Active::NotActive)
    }
    #[doc = "Power-up request active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Active::Active)
    }
}
impl R {
    #[doc = "Bit 0 - Activate power-up request"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Activate power-up request"]
    #[inline(always)]
    pub fn active(&mut self) -> ActiveW<'_, DbgpwrupreqSpec> {
        ActiveW::new(self, 0)
    }
}
#[doc = "Debug power-up request\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgpwrupreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgpwrupreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgpwrupreqSpec;
impl crate::RegisterSpec for DbgpwrupreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgpwrupreq::R`](R) reader structure"]
impl crate::Readable for DbgpwrupreqSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgpwrupreq::W`](W) writer structure"]
impl crate::Writable for DbgpwrupreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBGPWRUPREQ to value 0"]
impl crate::Resettable for DbgpwrupreqSpec {}
