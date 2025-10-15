#[doc = "Register `DATA1` reader"]
pub type R = crate::R<Data1Spec>;
#[doc = "Register `DATA1` writer"]
pub type W = crate::W<Data1Spec>;
#[doc = "Field `DATA1` reader - Abstract Data 1"]
pub type Data1R = crate::FieldReader<u32>;
#[doc = "Field `DATA1` writer - Abstract Data 1"]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Abstract Data 1"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Abstract Data 1"]
    #[inline(always)]
    pub fn data1(&mut self) -> Data1W<'_, Data1Spec> {
        Data1W::new(self, 0)
    }
}
#[doc = "Abstract Data 1. Read/write data for argument 1\n\nYou can [`read`](crate::Reg::read) this register and get [`data1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data1Spec;
impl crate::RegisterSpec for Data1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data1::R`](R) reader structure"]
impl crate::Readable for Data1Spec {}
#[doc = "`write(|w| ..)` method takes [`data1::W`](W) writer structure"]
impl crate::Writable for Data1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA1 to value 0"]
impl crate::Resettable for Data1Spec {}
