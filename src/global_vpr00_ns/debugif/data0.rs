#[doc = "Register `DATA0` reader"]
pub type R = crate::R<Data0Spec>;
#[doc = "Register `DATA0` writer"]
pub type W = crate::W<Data0Spec>;
#[doc = "Field `DATA0` reader - Abstract Data 0"]
pub type Data0R = crate::FieldReader<u32>;
#[doc = "Field `DATA0` writer - Abstract Data 0"]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Abstract Data 0"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Abstract Data 0"]
    #[inline(always)]
    pub fn data0(&mut self) -> Data0W<'_, Data0Spec> {
        Data0W::new(self, 0)
    }
}
#[doc = "Abstract Data 0. Read/write data for argument 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data0Spec;
impl crate::RegisterSpec for Data0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0::R`](R) reader structure"]
impl crate::Readable for Data0Spec {}
#[doc = "`write(|w| ..)` method takes [`data0::W`](W) writer structure"]
impl crate::Writable for Data0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA0 to value 0"]
impl crate::Resettable for Data0Spec {}
