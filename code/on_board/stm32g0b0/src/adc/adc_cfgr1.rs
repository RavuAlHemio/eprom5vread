#[doc = "Register `ADC_CFGR1` reader"]
pub type R = crate::R<ADC_CFGR1_SPEC>;
#[doc = "Register `ADC_CFGR1` writer"]
pub type W = crate::W<ADC_CFGR1_SPEC>;
#[doc = "Field `DMAEN` reader - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: DMA disabled"]
    B_0X0 = 0,
    #[doc = "1: DMA enabled"]
    B_0X1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::B_0X0,
            true => DMAEN_A::B_0X1,
        }
    }
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMAEN_A::B_0X0
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMAEN_A::B_0X1
    }
}
#[doc = "Field `DMAEN` writer - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN_A>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN_A::B_0X0)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN_A::B_0X1)
    }
}
#[doc = "Field `DMACFG` reader - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN=1. For more details, refer to page351 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DMACFG_R = crate::BitReader<DMACFG_A>;
#[doc = "Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN=1. For more details, refer to page351 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMACFG_A {
    #[doc = "0: DMA one shot mode selected"]
    B_0X0 = 0,
    #[doc = "1: DMA circular mode selected"]
    B_0X1 = 1,
}
impl From<DMACFG_A> for bool {
    #[inline(always)]
    fn from(variant: DMACFG_A) -> Self {
        variant as u8 != 0
    }
}
impl DMACFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMACFG_A {
        match self.bits {
            false => DMACFG_A::B_0X0,
            true => DMACFG_A::B_0X1,
        }
    }
    #[doc = "DMA one shot mode selected"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMACFG_A::B_0X0
    }
    #[doc = "DMA circular mode selected"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMACFG_A::B_0X1
    }
}
#[doc = "Field `DMACFG` writer - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN=1. For more details, refer to page351 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DMACFG_W<'a, REG> = crate::BitWriter<'a, REG, DMACFG_A>;
impl<'a, REG> DMACFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA one shot mode selected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMACFG_A::B_0X0)
    }
    #[doc = "DMA circular mode selected"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMACFG_A::B_0X1)
    }
}
#[doc = "Field `SCANDIR` reader - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type SCANDIR_R = crate::BitReader<SCANDIR_A>;
#[doc = "Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCANDIR_A {
    #[doc = "0: Upward scan (from CHSEL0 to CHSEL18)"]
    B_0X0 = 0,
    #[doc = "1: Backward scan (from CHSEL18 to CHSEL0)"]
    B_0X1 = 1,
}
impl From<SCANDIR_A> for bool {
    #[inline(always)]
    fn from(variant: SCANDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl SCANDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCANDIR_A {
        match self.bits {
            false => SCANDIR_A::B_0X0,
            true => SCANDIR_A::B_0X1,
        }
    }
    #[doc = "Upward scan (from CHSEL0 to CHSEL18)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SCANDIR_A::B_0X0
    }
    #[doc = "Backward scan (from CHSEL18 to CHSEL0)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SCANDIR_A::B_0X1
    }
}
#[doc = "Field `SCANDIR` writer - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type SCANDIR_W<'a, REG> = crate::BitWriter<'a, REG, SCANDIR_A>;
impl<'a, REG> SCANDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Upward scan (from CHSEL0 to CHSEL18)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SCANDIR_A::B_0X0)
    }
    #[doc = "Backward scan (from CHSEL18 to CHSEL0)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SCANDIR_A::B_0X1)
    }
}
#[doc = "Field `RES` reader - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADEN=0."]
pub type RES_R = crate::FieldReader<RES_A>;
#[doc = "Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADEN=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES_A {
    #[doc = "0: 12 bits"]
    B_0X0 = 0,
    #[doc = "1: 10 bits"]
    B_0X1 = 1,
    #[doc = "2: 8 bits"]
    B_0X2 = 2,
    #[doc = "3: 6 bits"]
    B_0X3 = 3,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RES_A {
    type Ux = u8;
}
impl RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RES_A {
        match self.bits {
            0 => RES_A::B_0X0,
            1 => RES_A::B_0X1,
            2 => RES_A::B_0X2,
            3 => RES_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "12 bits"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RES_A::B_0X0
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RES_A::B_0X1
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == RES_A::B_0X2
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == RES_A::B_0X3
    }
}
#[doc = "Field `RES` writer - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADEN=0."]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RES_A>;
impl<'a, REG> RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12 bits"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RES_A::B_0X0)
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RES_A::B_0X1)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(RES_A::B_0X2)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(RES_A::B_0X3)
    }
}
#[doc = "Field `ALIGN` reader - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on page349 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type ALIGN_R = crate::BitReader<ALIGN_A>;
#[doc = "Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on page349 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIGN_A {
    #[doc = "0: Right alignment"]
    B_0X0 = 0,
    #[doc = "1: Left alignment"]
    B_0X1 = 1,
}
impl From<ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl ALIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALIGN_A {
        match self.bits {
            false => ALIGN_A::B_0X0,
            true => ALIGN_A::B_0X1,
        }
    }
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ALIGN_A::B_0X0
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ALIGN_A::B_0X1
    }
}
#[doc = "Field `ALIGN` writer - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on page349 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type ALIGN_W<'a, REG> = crate::BitWriter<'a, REG, ALIGN_A>;
impl<'a, REG> ALIGN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ALIGN_A::B_0X0)
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ALIGN_A::B_0X1)
    }
}
#[doc = "Field `EXTSEL` reader - External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EXTSEL_R = crate::FieldReader<EXTSEL_A>;
#[doc = "External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL_A {
    #[doc = "0: TRG0"]
    B_0X0 = 0,
    #[doc = "1: TRG1"]
    B_0X1 = 1,
    #[doc = "2: TRG2"]
    B_0X2 = 2,
    #[doc = "3: TRG3"]
    B_0X3 = 3,
    #[doc = "4: TRG4"]
    B_0X4 = 4,
    #[doc = "5: TRG5"]
    B_0X5 = 5,
    #[doc = "6: TRG6"]
    B_0X6 = 6,
    #[doc = "7: TRG7"]
    B_0X7 = 7,
}
impl From<EXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTSEL_A {
    type Ux = u8;
}
impl EXTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTSEL_A {
        match self.bits {
            0 => EXTSEL_A::B_0X0,
            1 => EXTSEL_A::B_0X1,
            2 => EXTSEL_A::B_0X2,
            3 => EXTSEL_A::B_0X3,
            4 => EXTSEL_A::B_0X4,
            5 => EXTSEL_A::B_0X5,
            6 => EXTSEL_A::B_0X6,
            7 => EXTSEL_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "TRG0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EXTSEL_A::B_0X0
    }
    #[doc = "TRG1"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EXTSEL_A::B_0X1
    }
    #[doc = "TRG2"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == EXTSEL_A::B_0X2
    }
    #[doc = "TRG3"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == EXTSEL_A::B_0X3
    }
    #[doc = "TRG4"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == EXTSEL_A::B_0X4
    }
    #[doc = "TRG5"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == EXTSEL_A::B_0X5
    }
    #[doc = "TRG6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == EXTSEL_A::B_0X6
    }
    #[doc = "TRG7"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == EXTSEL_A::B_0X7
    }
}
#[doc = "Field `EXTSEL` writer - External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EXTSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, EXTSEL_A>;
impl<'a, REG> EXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TRG0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0X0)
    }
    #[doc = "TRG1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0X1)
    }
    #[doc = "TRG2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0X2)
    }
    #[doc = "TRG3"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0X3)
    }
    #[doc = "TRG4"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0X4)
    }
    #[doc = "TRG5"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0X5)
    }
    #[doc = "TRG6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0X6)
    }
    #[doc = "TRG7"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTSEL_A::B_0X7)
    }
}
#[doc = "Field `EXTEN` reader - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EXTEN_R = crate::FieldReader<EXTEN_A>;
#[doc = "External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTEN_A {
    #[doc = "0: Hardware trigger detection disabled (conversions can be started by software)"]
    B_0X0 = 0,
    #[doc = "1: Hardware trigger detection on the rising edge"]
    B_0X1 = 1,
    #[doc = "2: Hardware trigger detection on the falling edge"]
    B_0X2 = 2,
    #[doc = "3: Hardware trigger detection on both the rising and falling edges"]
    B_0X3 = 3,
}
impl From<EXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTEN_A {
    type Ux = u8;
}
impl EXTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTEN_A {
        match self.bits {
            0 => EXTEN_A::B_0X0,
            1 => EXTEN_A::B_0X1,
            2 => EXTEN_A::B_0X2,
            3 => EXTEN_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Hardware trigger detection disabled (conversions can be started by software)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EXTEN_A::B_0X0
    }
    #[doc = "Hardware trigger detection on the rising edge"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EXTEN_A::B_0X1
    }
    #[doc = "Hardware trigger detection on the falling edge"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == EXTEN_A::B_0X2
    }
    #[doc = "Hardware trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == EXTEN_A::B_0X3
    }
}
#[doc = "Field `EXTEN` writer - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type EXTEN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EXTEN_A>;
impl<'a, REG> EXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Hardware trigger detection disabled (conversions can be started by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN_A::B_0X0)
    }
    #[doc = "Hardware trigger detection on the rising edge"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN_A::B_0X1)
    }
    #[doc = "Hardware trigger detection on the falling edge"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN_A::B_0X2)
    }
    #[doc = "Hardware trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTEN_A::B_0X3)
    }
}
#[doc = "Field `OVRMOD` reader - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type OVRMOD_R = crate::BitReader<OVRMOD_A>;
#[doc = "Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRMOD_A {
    #[doc = "0: ADC_DR register is preserved with the old data when an overrun is detected."]
    B_0X0 = 0,
    #[doc = "1: ADC_DR register is overwritten with the last conversion result when an overrun is detected."]
    B_0X1 = 1,
}
impl From<OVRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: OVRMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVRMOD_A {
        match self.bits {
            false => OVRMOD_A::B_0X0,
            true => OVRMOD_A::B_0X1,
        }
    }
    #[doc = "ADC_DR register is preserved with the old data when an overrun is detected."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OVRMOD_A::B_0X0
    }
    #[doc = "ADC_DR register is overwritten with the last conversion result when an overrun is detected."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OVRMOD_A::B_0X1
    }
}
#[doc = "Field `OVRMOD` writer - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type OVRMOD_W<'a, REG> = crate::BitWriter<'a, REG, OVRMOD_A>;
impl<'a, REG> OVRMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC_DR register is preserved with the old data when an overrun is detected."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVRMOD_A::B_0X0)
    }
    #[doc = "ADC_DR register is overwritten with the last conversion result when an overrun is detected."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVRMOD_A::B_0X1)
    }
}
#[doc = "Field `CONT` reader - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type CONT_R = crate::BitReader<CONT_A>;
#[doc = "Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT_A {
    #[doc = "0: Single conversion mode"]
    B_0X0 = 0,
    #[doc = "1: Continuous conversion mode"]
    B_0X1 = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
impl CONT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::B_0X0,
            true => CONT_A::B_0X1,
        }
    }
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CONT_A::B_0X0
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CONT_A::B_0X1
    }
}
#[doc = "Field `CONT` writer - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type CONT_W<'a, REG> = crate::BitWriter<'a, REG, CONT_A>;
impl<'a, REG> CONT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CONT_A::B_0X0)
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CONT_A::B_0X1)
    }
}
#[doc = "Field `WAIT` reader - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type WAIT_R = crate::BitReader<WAIT_A>;
#[doc = "Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAIT_A {
    #[doc = "0: Wait conversion mode off"]
    B_0X0 = 0,
    #[doc = "1: Wait conversion mode on"]
    B_0X1 = 1,
}
impl From<WAIT_A> for bool {
    #[inline(always)]
    fn from(variant: WAIT_A) -> Self {
        variant as u8 != 0
    }
}
impl WAIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAIT_A {
        match self.bits {
            false => WAIT_A::B_0X0,
            true => WAIT_A::B_0X1,
        }
    }
    #[doc = "Wait conversion mode off"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WAIT_A::B_0X0
    }
    #[doc = "Wait conversion mode on"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WAIT_A::B_0X1
    }
}
#[doc = "Field `WAIT` writer - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type WAIT_W<'a, REG> = crate::BitWriter<'a, REG, WAIT_A>;
impl<'a, REG> WAIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wait conversion mode off"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WAIT_A::B_0X0)
    }
    #[doc = "Wait conversion mode on"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WAIT_A::B_0X1)
    }
}
#[doc = "Field `AUTOFF` reader - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AUTOFF_R = crate::BitReader<AUTOFF_A>;
#[doc = "Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOFF_A {
    #[doc = "0: Auto-off mode disabled"]
    B_0X0 = 0,
    #[doc = "1: Auto-off mode enabled"]
    B_0X1 = 1,
}
impl From<AUTOFF_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AUTOFF_A {
        match self.bits {
            false => AUTOFF_A::B_0X0,
            true => AUTOFF_A::B_0X1,
        }
    }
    #[doc = "Auto-off mode disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AUTOFF_A::B_0X0
    }
    #[doc = "Auto-off mode enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AUTOFF_A::B_0X1
    }
}
#[doc = "Field `AUTOFF` writer - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AUTOFF_W<'a, REG> = crate::BitWriter<'a, REG, AUTOFF_A>;
impl<'a, REG> AUTOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto-off mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOFF_A::B_0X0)
    }
    #[doc = "Auto-off mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOFF_A::B_0X1)
    }
}
#[doc = "Field `DISCEN` reader - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DISCEN_R = crate::BitReader<DISCEN_A>;
#[doc = "Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCEN_A {
    #[doc = "0: Discontinuous mode disabled"]
    B_0X0 = 0,
    #[doc = "1: Discontinuous mode enabled"]
    B_0X1 = 1,
}
impl From<DISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DISCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DISCEN_A {
        match self.bits {
            false => DISCEN_A::B_0X0,
            true => DISCEN_A::B_0X1,
        }
    }
    #[doc = "Discontinuous mode disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DISCEN_A::B_0X0
    }
    #[doc = "Discontinuous mode enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DISCEN_A::B_0X1
    }
}
#[doc = "Field `DISCEN` writer - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG, DISCEN_A>;
impl<'a, REG> DISCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Discontinuous mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN_A::B_0X0)
    }
    #[doc = "Discontinuous mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DISCEN_A::B_0X1)
    }
}
#[doc = "Field `CHSELRMOD` reader - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSELRMOD_R = crate::BitReader<CHSELRMOD_A>;
#[doc = "Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSELRMOD_A {
    #[doc = "0: Each bit of the ADC_CHSELR register enables an input"]
    B_0X0 = 0,
    #[doc = "1: ADC_CHSELR register is able to sequence up to 8 channels"]
    B_0X1 = 1,
}
impl From<CHSELRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELRMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSELRMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHSELRMOD_A {
        match self.bits {
            false => CHSELRMOD_A::B_0X0,
            true => CHSELRMOD_A::B_0X1,
        }
    }
    #[doc = "Each bit of the ADC_CHSELR register enables an input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHSELRMOD_A::B_0X0
    }
    #[doc = "ADC_CHSELR register is able to sequence up to 8 channels"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHSELRMOD_A::B_0X1
    }
}
#[doc = "Field `CHSELRMOD` writer - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
pub type CHSELRMOD_W<'a, REG> = crate::BitWriter<'a, REG, CHSELRMOD_A>;
impl<'a, REG> CHSELRMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Each bit of the ADC_CHSELR register enables an input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELRMOD_A::B_0X0)
    }
    #[doc = "ADC_CHSELR register is able to sequence up to 8 channels"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHSELRMOD_A::B_0X1)
    }
}
#[doc = "Field `AWD1SGL` reader - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1SGL_R = crate::BitReader<AWD1SGL_A>;
#[doc = "Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1SGL_A {
    #[doc = "0: Analog watchdog 1 enabled on all channels"]
    B_0X0 = 0,
    #[doc = "1: Analog watchdog 1 enabled on a single channel"]
    B_0X1 = 1,
}
impl From<AWD1SGL_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1SGL_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD1SGL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD1SGL_A {
        match self.bits {
            false => AWD1SGL_A::B_0X0,
            true => AWD1SGL_A::B_0X1,
        }
    }
    #[doc = "Analog watchdog 1 enabled on all channels"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD1SGL_A::B_0X0
    }
    #[doc = "Analog watchdog 1 enabled on a single channel"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD1SGL_A::B_0X1
    }
}
#[doc = "Field `AWD1SGL` writer - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1SGL_W<'a, REG> = crate::BitWriter<'a, REG, AWD1SGL_A>;
impl<'a, REG> AWD1SGL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog 1 enabled on all channels"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1SGL_A::B_0X0)
    }
    #[doc = "Analog watchdog 1 enabled on a single channel"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1SGL_A::B_0X1)
    }
}
#[doc = "Field `AWD1EN` reader - Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1EN_R = crate::BitReader<AWD1EN_A>;
#[doc = "Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1EN_A {
    #[doc = "0: Analog watchdog 1 disabled"]
    B_0X0 = 0,
    #[doc = "1: Analog watchdog 1 enabled"]
    B_0X1 = 1,
}
impl From<AWD1EN_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD1EN_A {
        match self.bits {
            false => AWD1EN_A::B_0X0,
            true => AWD1EN_A::B_0X1,
        }
    }
    #[doc = "Analog watchdog 1 disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD1EN_A::B_0X0
    }
    #[doc = "Analog watchdog 1 enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD1EN_A::B_0X1
    }
}
#[doc = "Field `AWD1EN` writer - Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1EN_W<'a, REG> = crate::BitWriter<'a, REG, AWD1EN_A>;
impl<'a, REG> AWD1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog 1 disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1EN_A::B_0X0)
    }
    #[doc = "Analog watchdog 1 enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1EN_A::B_0X1)
    }
}
#[doc = "Field `AWD1CH` reader - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1CH_R = crate::FieldReader<AWD1CH_A>;
#[doc = "Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AWD1CH_A {
    #[doc = "0: ADC analog input Channel 0 monitored by AWD"]
    B_0X0 = 0,
    #[doc = "1: ADC analog input Channel 1 monitored by AWD"]
    B_0X1 = 1,
    #[doc = "17: ADC analog input Channel 17 monitored by AWD"]
    B_0X11 = 17,
    #[doc = "18: ADC analog input Channel 18 monitored by AWD"]
    B_0X12 = 18,
}
impl From<AWD1CH_A> for u8 {
    #[inline(always)]
    fn from(variant: AWD1CH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AWD1CH_A {
    type Ux = u8;
}
impl AWD1CH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AWD1CH_A> {
        match self.bits {
            0 => Some(AWD1CH_A::B_0X0),
            1 => Some(AWD1CH_A::B_0X1),
            17 => Some(AWD1CH_A::B_0X11),
            18 => Some(AWD1CH_A::B_0X12),
            _ => None,
        }
    }
    #[doc = "ADC analog input Channel 0 monitored by AWD"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD1CH_A::B_0X0
    }
    #[doc = "ADC analog input Channel 1 monitored by AWD"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD1CH_A::B_0X1
    }
    #[doc = "ADC analog input Channel 17 monitored by AWD"]
    #[inline(always)]
    pub fn is_b_0x11(&self) -> bool {
        *self == AWD1CH_A::B_0X11
    }
    #[doc = "ADC analog input Channel 18 monitored by AWD"]
    #[inline(always)]
    pub fn is_b_0x12(&self) -> bool {
        *self == AWD1CH_A::B_0X12
    }
}
#[doc = "Field `AWD1CH` writer - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
pub type AWD1CH_W<'a, REG> = crate::FieldWriter<'a, REG, 5, AWD1CH_A>;
impl<'a, REG> AWD1CH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC analog input Channel 0 monitored by AWD"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1CH_A::B_0X0)
    }
    #[doc = "ADC analog input Channel 1 monitored by AWD"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1CH_A::B_0X1)
    }
    #[doc = "ADC analog input Channel 17 monitored by AWD"]
    #[inline(always)]
    pub fn b_0x11(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1CH_A::B_0X11)
    }
    #[doc = "ADC analog input Channel 18 monitored by AWD"]
    #[inline(always)]
    pub fn b_0x12(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1CH_A::B_0X12)
    }
}
impl R {
    #[doc = "Bit 0 - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN=1. For more details, refer to page351 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn scandir(&self) -> SCANDIR_R {
        SCANDIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADEN=0."]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on page349 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 10:11 - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn autoff(&self) -> AUTOFF_R {
        AUTOFF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    pub fn chselrmod(&self) -> CHSELRMOD_R {
        CHSELRMOD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Direct memory access enable This bit is set and cleared by software to enable the generation of DMA requests. This allows the DMA controller to be used to manage automatically the converted data. For more details, refer to . Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<ADC_CFGR1_SPEC> {
        DMAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Direct memory access configuration This bit is set and cleared by software to select between two DMA modes of operation and is effective only when DMAEN=1. For more details, refer to page351 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn dmacfg(&mut self) -> DMACFG_W<ADC_CFGR1_SPEC> {
        DMACFG_W::new(self, 1)
    }
    #[doc = "Bit 2 - Scan sequence direction This bit is set and cleared by software to select the direction in which the channels is scanned in the sequence. It is effective only if CHSELMOD bit is cleared to 0. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn scandir(&mut self) -> SCANDIR_W<ADC_CFGR1_SPEC> {
        SCANDIR_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Data resolution These bits are written by software to select the resolution of the conversion. Note: The software is allowed to write these bits only when ADEN=0."]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<ADC_CFGR1_SPEC> {
        RES_W::new(self, 3)
    }
    #[doc = "Bit 5 - Data alignment This bit is set and cleared by software to select right or left alignment. Refer to Data alignment and resolution (oversampling disabled: OVSE = 0) on page349 Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<ADC_CFGR1_SPEC> {
        ALIGN_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - External trigger selection These bits select the external event used to trigger the start of conversion (refer to External triggers for details): Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<ADC_CFGR1_SPEC> {
        EXTSEL_W::new(self, 6)
    }
    #[doc = "Bits 10:11 - External trigger enable and polarity selection These bits are set and cleared by software to select the external trigger polarity and enable the trigger. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> EXTEN_W<ADC_CFGR1_SPEC> {
        EXTEN_W::new(self, 10)
    }
    #[doc = "Bit 12 - Overrun management mode This bit is set and cleared by software and configure the way data overruns are managed. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn ovrmod(&mut self) -> OVRMOD_W<ADC_CFGR1_SPEC> {
        OVRMOD_W::new(self, 12)
    }
    #[doc = "Bit 13 - Single / continuous conversion mode This bit is set and cleared by software. If it is set, conversion takes place continuously until it is cleared. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<ADC_CFGR1_SPEC> {
        CONT_W::new(self, 13)
    }
    #[doc = "Bit 14 - Wait conversion mode This bit is set and cleared by software to enable/disable wait conversion mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WAIT_W<ADC_CFGR1_SPEC> {
        WAIT_W::new(self, 14)
    }
    #[doc = "Bit 15 - Auto-off mode This bit is set and cleared by software to enable/disable auto-off mode.. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn autoff(&mut self) -> AUTOFF_W<ADC_CFGR1_SPEC> {
        AUTOFF_W::new(self, 15)
    }
    #[doc = "Bit 16 - Discontinuous mode This bit is set and cleared by software to enable/disable discontinuous mode. Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden to set both bits DISCEN=1 and CONT=1. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DISCEN_W<ADC_CFGR1_SPEC> {
        DISCEN_W::new(self, 16)
    }
    #[doc = "Bit 21 - Mode selection of the ADC_CHSELR register This bit is set and cleared by software to control the ADC_CHSELR feature: Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing). If CCRDY is not yet asserted after channel configuration (writing ADC_CHSELR register or changing CHSELRMOD or SCANDIR), the value written to this bit is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn chselrmod(&mut self) -> CHSELRMOD_W<ADC_CFGR1_SPEC> {
        CHSELRMOD_W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable the watchdog on a single channel or on all channels This bit is set and cleared by software to enable the analog watchdog on the channel identified by the AWDCH\\[4:0\\]
bits or on all the channels Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<ADC_CFGR1_SPEC> {
        AWD1SGL_W::new(self, 22)
    }
    #[doc = "Bit 23 - Analog watchdog enable This bit is set and cleared by software. Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd1en(&mut self) -> AWD1EN_W<ADC_CFGR1_SPEC> {
        AWD1EN_W::new(self, 23)
    }
    #[doc = "Bits 26:30 - Analog watchdog channel selection These bits are set and cleared by software. They select the input channel to be guarded by the analog watchdog. ..... Others: Reserved Note: The channel selected by the AWDCH\\[4:0\\]
bits must be also set into the CHSELR register. The software is allowed to write this bit only when ADSTART bit is cleared to 0 (this ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn awd1ch(&mut self) -> AWD1CH_W<ADC_CFGR1_SPEC> {
        AWD1CH_W::new(self, 26)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_CFGR1_SPEC;
impl crate::RegisterSpec for ADC_CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_cfgr1::R`](R) reader structure"]
impl crate::Readable for ADC_CFGR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc_cfgr1::W`](W) writer structure"]
impl crate::Writable for ADC_CFGR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CFGR1 to value 0"]
impl crate::Resettable for ADC_CFGR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
