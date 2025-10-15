#[doc = "Register `A6` reader"]
pub type R = crate::R<A6Spec>;
#[doc = "Register `A6` writer"]
pub type W = crate::W<A6Spec>;
#[doc = "Field `A6` reader - Slope of 7th piece wise linear function"]
pub type A6R = crate::FieldReader<u16>;
#[doc = "Field `A6` writer - Slope of 7th piece wise linear function"]
pub type A6W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Slope of 7th piece wise linear function"]
    #[inline(always)]
    pub fn a6(&self) -> A6R {
        A6R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slope of 7th piece wise linear function"]
    #[inline(always)]
    pub fn a6(&mut self) -> A6W<'_, A6Spec> {
        A6W::new(self, 0)
    }
}
#[doc = "Slope of 7th piece wise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`a6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A6Spec;
impl crate::RegisterSpec for A6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a6::R`](R) reader structure"]
impl crate::Readable for A6Spec {}
#[doc = "`write(|w| ..)` method takes [`a6::W`](W) writer structure"]
impl crate::Writable for A6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets A6 to value 0x05b7"]
impl crate::Resettable for A6Spec {
    const RESET_VALUE: u32 = 0x05b7;
}
