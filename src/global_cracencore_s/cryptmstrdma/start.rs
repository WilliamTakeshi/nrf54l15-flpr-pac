#[doc = "Register `START` reader"]
pub type R = crate::R<StartSpec>;
#[doc = "Register `START` writer"]
pub type W = crate::W<StartSpec>;
#[doc = "Field `STARTFETCH` writer - "]
pub type StartfetchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTPUSH` writer - "]
pub type StartpushW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn startfetch(&mut self) -> StartfetchW<'_, StartSpec> {
        StartfetchW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn startpush(&mut self) -> StartpushW<'_, StartSpec> {
        StartpushW::new(self, 1)
    }
}
#[doc = "Start\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartSpec;
impl crate::RegisterSpec for StartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`start::R`](R) reader structure"]
impl crate::Readable for StartSpec {}
#[doc = "`write(|w| ..)` method takes [`start::W`](W) writer structure"]
impl crate::Writable for StartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for StartSpec {}
