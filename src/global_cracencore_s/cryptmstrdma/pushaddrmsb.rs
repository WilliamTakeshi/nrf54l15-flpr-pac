#[doc = "Register `PUSHADDRMSB` reader"]
pub type R = crate::R<PushaddrmsbSpec>;
#[doc = "Register `PUSHADDRMSB` writer"]
pub type W = crate::W<PushaddrmsbSpec>;
#[doc = "Field `PUSHADDRMSB` reader - "]
pub type PushaddrmsbR = crate::FieldReader<u32>;
#[doc = "Field `PUSHADDRMSB` writer - "]
pub type PushaddrmsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pushaddrmsb(&self) -> PushaddrmsbR {
        PushaddrmsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pushaddrmsb(&mut self) -> PushaddrmsbW<'_, PushaddrmsbSpec> {
        PushaddrmsbW::new(self, 0)
    }
}
#[doc = "Push Address Most Significant Bit\n\nYou can [`read`](crate::Reg::read) this register and get [`pushaddrmsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pushaddrmsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PushaddrmsbSpec;
impl crate::RegisterSpec for PushaddrmsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pushaddrmsb::R`](R) reader structure"]
impl crate::Readable for PushaddrmsbSpec {}
#[doc = "`write(|w| ..)` method takes [`pushaddrmsb::W`](W) writer structure"]
impl crate::Writable for PushaddrmsbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PUSHADDRMSB to value 0"]
impl crate::Resettable for PushaddrmsbSpec {}
