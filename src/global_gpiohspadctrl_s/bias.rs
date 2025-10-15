#[doc = "Register `BIAS` reader"]
pub type R = crate::R<BiasSpec>;
#[doc = "Register `BIAS` writer"]
pub type W = crate::W<BiasSpec>;
#[doc = "Field `HSBIAS` reader - Slew setting for high-speed pad (higher value is faster)"]
pub type HsbiasR = crate::FieldReader;
#[doc = "Field `HSBIAS` writer - Slew setting for high-speed pad (higher value is faster)"]
pub type HsbiasW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REPLICABIAS` reader - Slew setting for replica clock (used by the VPR coprocessor for emulating a QSPI peripheral)"]
pub type ReplicabiasR = crate::BitReader;
#[doc = "Field `REPLICABIAS` writer - Slew setting for replica clock (used by the VPR coprocessor for emulating a QSPI peripheral)"]
pub type ReplicabiasW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Slew setting for high-speed pad (higher value is faster)"]
    #[inline(always)]
    pub fn hsbias(&self) -> HsbiasR {
        HsbiasR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Slew setting for replica clock (used by the VPR coprocessor for emulating a QSPI peripheral)"]
    #[inline(always)]
    pub fn replicabias(&self) -> ReplicabiasR {
        ReplicabiasR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Slew setting for high-speed pad (higher value is faster)"]
    #[inline(always)]
    pub fn hsbias(&mut self) -> HsbiasW<'_, BiasSpec> {
        HsbiasW::new(self, 0)
    }
    #[doc = "Bit 2 - Slew setting for replica clock (used by the VPR coprocessor for emulating a QSPI peripheral)"]
    #[inline(always)]
    pub fn replicabias(&mut self) -> ReplicabiasW<'_, BiasSpec> {
        ReplicabiasW::new(self, 2)
    }
}
#[doc = "Bias control\n\nYou can [`read`](crate::Reg::read) this register and get [`bias::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bias::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BiasSpec;
impl crate::RegisterSpec for BiasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bias::R`](R) reader structure"]
impl crate::Readable for BiasSpec {}
#[doc = "`write(|w| ..)` method takes [`bias::W`](W) writer structure"]
impl crate::Writable for BiasSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BIAS to value 0"]
impl crate::Resettable for BiasSpec {}
