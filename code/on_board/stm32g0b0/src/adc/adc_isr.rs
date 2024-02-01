#[doc = "Register `ADC_ISR` reader"]
pub type R = crate::R<ADC_ISR_SPEC>;
#[doc = "Register `ADC_ISR` writer"]
pub type W = crate::W<ADC_ISR_SPEC>;
#[doc = "Field `ADRDY` reader - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN=1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
pub type ADRDY_R = crate::BitReader<ADRDY_A>;
#[doc = "ADC ready This bit is set by hardware after the ADC has been enabled (ADEN=1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDY_A {
    #[doc = "0: ADC not yet ready to start conversion (or the flag event was already acknowledged and cleared by software)"]
    B_0X0 = 0,
    #[doc = "1: ADC is ready to start conversion"]
    B_0X1 = 1,
}
impl From<ADRDY_A> for bool {
    #[inline(always)]
    fn from(variant: ADRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl ADRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADRDY_A {
        match self.bits {
            false => ADRDY_A::B_0X0,
            true => ADRDY_A::B_0X1,
        }
    }
    #[doc = "ADC not yet ready to start conversion (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADRDY_A::B_0X0
    }
    #[doc = "ADC is ready to start conversion"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADRDY_A::B_0X1
    }
}
#[doc = "Field `ADRDY` writer - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN=1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
pub type ADRDY_W<'a, REG> = crate::BitWriter<'a, REG, ADRDY_A>;
impl<'a, REG> ADRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC not yet ready to start conversion (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDY_A::B_0X0)
    }
    #[doc = "ADC is ready to start conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDY_A::B_0X1)
    }
}
#[doc = "Field `EOSMP` reader - End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to '1'."]
pub type EOSMP_R = crate::BitReader<EOSMP_A>;
#[doc = "End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to '1'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMP_A {
    #[doc = "0: Not at the end of the sampling phase (or the flag event was already acknowledged and cleared by software)"]
    B_0X0 = 0,
    #[doc = "1: End of sampling phase reached"]
    B_0X1 = 1,
}
impl From<EOSMP_A> for bool {
    #[inline(always)]
    fn from(variant: EOSMP_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOSMP_A {
        match self.bits {
            false => EOSMP_A::B_0X0,
            true => EOSMP_A::B_0X1,
        }
    }
    #[doc = "Not at the end of the sampling phase (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOSMP_A::B_0X0
    }
    #[doc = "End of sampling phase reached"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOSMP_A::B_0X1
    }
}
#[doc = "Field `EOSMP` writer - End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to '1'."]
pub type EOSMP_W<'a, REG> = crate::BitWriter<'a, REG, EOSMP_A>;
impl<'a, REG> EOSMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not at the end of the sampling phase (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMP_A::B_0X0)
    }
    #[doc = "End of sampling phase reached"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMP_A::B_0X1)
    }
}
#[doc = "Field `EOC` reader - End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register."]
pub type EOC_R = crate::BitReader<EOC_A>;
#[doc = "End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOC_A {
    #[doc = "0: Channel conversion not complete (or the flag event was already acknowledged and cleared by software)"]
    B_0X0 = 0,
    #[doc = "1: Channel conversion complete"]
    B_0X1 = 1,
}
impl From<EOC_A> for bool {
    #[inline(always)]
    fn from(variant: EOC_A) -> Self {
        variant as u8 != 0
    }
}
impl EOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOC_A {
        match self.bits {
            false => EOC_A::B_0X0,
            true => EOC_A::B_0X1,
        }
    }
    #[doc = "Channel conversion not complete (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOC_A::B_0X0
    }
    #[doc = "Channel conversion complete"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOC_A::B_0X1
    }
}
#[doc = "Field `EOC` writer - End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register."]
pub type EOC_W<'a, REG> = crate::BitWriter<'a, REG, EOC_A>;
impl<'a, REG> EOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel conversion not complete (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOC_A::B_0X0)
    }
    #[doc = "Channel conversion complete"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOC_A::B_0X1)
    }
}
#[doc = "Field `EOS` reader - End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it."]
pub type EOS_R = crate::BitReader<EOS_A>;
#[doc = "End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOS_A {
    #[doc = "0: Conversion sequence not complete (or the flag event was already acknowledged and cleared by software)"]
    B_0X0 = 0,
    #[doc = "1: Conversion sequence complete"]
    B_0X1 = 1,
}
impl From<EOS_A> for bool {
    #[inline(always)]
    fn from(variant: EOS_A) -> Self {
        variant as u8 != 0
    }
}
impl EOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOS_A {
        match self.bits {
            false => EOS_A::B_0X0,
            true => EOS_A::B_0X1,
        }
    }
    #[doc = "Conversion sequence not complete (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOS_A::B_0X0
    }
    #[doc = "Conversion sequence complete"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOS_A::B_0X1
    }
}
#[doc = "Field `EOS` writer - End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it."]
pub type EOS_W<'a, REG> = crate::BitWriter<'a, REG, EOS_A>;
impl<'a, REG> EOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Conversion sequence not complete (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOS_A::B_0X0)
    }
    #[doc = "Conversion sequence complete"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOS_A::B_0X1)
    }
}
#[doc = "Field `OVR` reader - ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it."]
pub type OVR_R = crate::BitReader<OVR_A>;
#[doc = "ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_A {
    #[doc = "0: No overrun occurred (or the flag event was already acknowledged and cleared by software)"]
    B_0X0 = 0,
    #[doc = "1: Overrun has occurred"]
    B_0X1 = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::B_0X0,
            true => OVR_A::B_0X1,
        }
    }
    #[doc = "No overrun occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OVR_A::B_0X0
    }
    #[doc = "Overrun has occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OVR_A::B_0X1
    }
}
#[doc = "Field `OVR` writer - ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it."]
pub type OVR_W<'a, REG> = crate::BitWriter<'a, REG, OVR_A>;
impl<'a, REG> OVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overrun occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVR_A::B_0X0)
    }
    #[doc = "Overrun has occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVR_A::B_0X1)
    }
}
#[doc = "Field `AWD1` reader - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1."]
pub type AWD1_R = crate::BitReader<AWD1_A>;
#[doc = "Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1_A {
    #[doc = "0: No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    B_0X0 = 0,
    #[doc = "1: Analog watchdog event occurred"]
    B_0X1 = 1,
}
impl From<AWD1_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD1_A {
        match self.bits {
            false => AWD1_A::B_0X0,
            true => AWD1_A::B_0X1,
        }
    }
    #[doc = "No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD1_A::B_0X0
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD1_A::B_0X1
    }
}
#[doc = "Field `AWD1` writer - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1."]
pub type AWD1_W<'a, REG> = crate::BitWriter<'a, REG, AWD1_A>;
impl<'a, REG> AWD1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1_A::B_0X0)
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1_A::B_0X1)
    }
}
#[doc = "Field `AWD2` reader - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it."]
pub type AWD2_R = crate::BitReader<AWD2_A>;
#[doc = "Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD2_A {
    #[doc = "0: No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    B_0X0 = 0,
    #[doc = "1: Analog watchdog event occurred"]
    B_0X1 = 1,
}
impl From<AWD2_A> for bool {
    #[inline(always)]
    fn from(variant: AWD2_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD2_A {
        match self.bits {
            false => AWD2_A::B_0X0,
            true => AWD2_A::B_0X1,
        }
    }
    #[doc = "No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD2_A::B_0X0
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD2_A::B_0X1
    }
}
#[doc = "Field `AWD2` writer - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it."]
pub type AWD2_W<'a, REG> = crate::BitWriter<'a, REG, AWD2_A>;
impl<'a, REG> AWD2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2_A::B_0X0)
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD2_A::B_0X1)
    }
}
#[doc = "Field `AWD3` reader - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1."]
pub type AWD3_R = crate::BitReader<AWD3_A>;
#[doc = "Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3_A {
    #[doc = "0: No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    B_0X0 = 0,
    #[doc = "1: Analog watchdog event occurred"]
    B_0X1 = 1,
}
impl From<AWD3_A> for bool {
    #[inline(always)]
    fn from(variant: AWD3_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD3_A {
        match self.bits {
            false => AWD3_A::B_0X0,
            true => AWD3_A::B_0X1,
        }
    }
    #[doc = "No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWD3_A::B_0X0
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWD3_A::B_0X1
    }
}
#[doc = "Field `AWD3` writer - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1."]
pub type AWD3_W<'a, REG> = crate::BitWriter<'a, REG, AWD3_A>;
impl<'a, REG> AWD3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No analog watchdog event occurred (or the flag event was already acknowledged and cleared by software)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3_A::B_0X0)
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3_A::B_0X1)
    }
}
#[doc = "Field `EOCAL` reader - End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it."]
pub type EOCAL_R = crate::BitReader<EOCAL_A>;
#[doc = "End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCAL_A {
    #[doc = "0: Calibration is not complete"]
    B_0X0 = 0,
    #[doc = "1: Calibration is complete"]
    B_0X1 = 1,
}
impl From<EOCAL_A> for bool {
    #[inline(always)]
    fn from(variant: EOCAL_A) -> Self {
        variant as u8 != 0
    }
}
impl EOCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOCAL_A {
        match self.bits {
            false => EOCAL_A::B_0X0,
            true => EOCAL_A::B_0X1,
        }
    }
    #[doc = "Calibration is not complete"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOCAL_A::B_0X0
    }
    #[doc = "Calibration is complete"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOCAL_A::B_0X1
    }
}
#[doc = "Field `EOCAL` writer - End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it."]
pub type EOCAL_W<'a, REG> = crate::BitWriter<'a, REG, EOCAL_A>;
impl<'a, REG> EOCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration is not complete"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOCAL_A::B_0X0)
    }
    #[doc = "Calibration is complete"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOCAL_A::B_0X1)
    }
}
#[doc = "Field `CCRDY` reader - Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration."]
pub type CCRDY_R = crate::BitReader<CCRDY_A>;
#[doc = "Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCRDY_A {
    #[doc = "0: Channel configuration update not applied."]
    B_0X0 = 0,
    #[doc = "1: Channel configuration update is applied."]
    B_0X1 = 1,
}
impl From<CCRDY_A> for bool {
    #[inline(always)]
    fn from(variant: CCRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl CCRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCRDY_A {
        match self.bits {
            false => CCRDY_A::B_0X0,
            true => CCRDY_A::B_0X1,
        }
    }
    #[doc = "Channel configuration update not applied."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CCRDY_A::B_0X0
    }
    #[doc = "Channel configuration update is applied."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CCRDY_A::B_0X1
    }
}
#[doc = "Field `CCRDY` writer - Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration."]
pub type CCRDY_W<'a, REG> = crate::BitWriter<'a, REG, CCRDY_A>;
impl<'a, REG> CCRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel configuration update not applied."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CCRDY_A::B_0X0)
    }
    #[doc = "Channel configuration update is applied."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CCRDY_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN=1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to '1'."]
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register."]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1."]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it."]
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1."]
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it."]
    #[inline(always)]
    pub fn eocal(&self) -> EOCAL_R {
        EOCAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration."]
    #[inline(always)]
    pub fn ccrdy(&self) -> CCRDY_R {
        CCRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ready This bit is set by hardware after the ADC has been enabled (ADEN=1) and when the ADC reaches a state where it is ready to accept conversion requests. It is cleared by software writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn adrdy(&mut self) -> ADRDY_W<ADC_ISR_SPEC> {
        ADRDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - End of sampling flag This bit is set by hardware during the conversion, at the end of the sampling phase.It is cleared by software by programming it to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn eosmp(&mut self) -> EOSMP_W<ADC_ISR_SPEC> {
        EOSMP_W::new(self, 1)
    }
    #[doc = "Bit 2 - End of conversion flag This bit is set by hardware at the end of each conversion of a channel when a new data result is available in the ADC_DR register. It is cleared by software writing 1 to it or by reading the ADC_DR register."]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<ADC_ISR_SPEC> {
        EOC_W::new(self, 2)
    }
    #[doc = "Bit 3 - End of sequence flag This bit is set by hardware at the end of the conversion of a sequence of channels selected by the CHSEL bits. It is cleared by software writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn eos(&mut self) -> EOS_W<ADC_ISR_SPEC> {
        EOS_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC overrun This bit is set by hardware when an overrun occurs, meaning that a new conversion has complete while the EOC flag was already set. It is cleared by software writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<ADC_ISR_SPEC> {
        OVR_W::new(self, 4)
    }
    #[doc = "Bit 7 - Analog watchdog 1 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_TR1 and ADC_HR1 registers. It is cleared by software by programming it to 1."]
    #[inline(always)]
    #[must_use]
    pub fn awd1(&mut self) -> AWD1_W<ADC_ISR_SPEC> {
        AWD1_W::new(self, 7)
    }
    #[doc = "Bit 8 - Analog watchdog 2 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD2TR and ADC_AWD2TR registers. It is cleared by software programming it it."]
    #[inline(always)]
    #[must_use]
    pub fn awd2(&mut self) -> AWD2_W<ADC_ISR_SPEC> {
        AWD2_W::new(self, 8)
    }
    #[doc = "Bit 9 - Analog watchdog 3 flag This bit is set by hardware when the converted voltage crosses the values programmed in ADC_AWD3TR and ADC_AWD3TR registers. It is cleared by software by programming it to 1."]
    #[inline(always)]
    #[must_use]
    pub fn awd3(&mut self) -> AWD3_W<ADC_ISR_SPEC> {
        AWD3_W::new(self, 9)
    }
    #[doc = "Bit 11 - End Of Calibration flag This bit is set by hardware when calibration is complete. It is cleared by software writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn eocal(&mut self) -> EOCAL_W<ADC_ISR_SPEC> {
        EOCAL_W::new(self, 11)
    }
    #[doc = "Bit 13 - Channel Configuration Ready flag This flag bit is set by hardware when the channel configuration is applied after programming to ADC_CHSELR register or changing CHSELRMOD or SCANDIR. It is cleared by software by programming it to it. Note: When the software configures the channels (by programming ADC_CHSELR or changing CHSELRMOD or SCANDIR), it must wait until the CCRDY flag rises before configuring again or starting conversions, otherwise the new configuration (or the START bit) is ignored. Once the flag is asserted, if the software needs to configure again the channels, it must clear the CCRDY flag before proceeding with a new configuration."]
    #[inline(always)]
    #[must_use]
    pub fn ccrdy(&mut self) -> CCRDY_W<ADC_ISR_SPEC> {
        CCRDY_W::new(self, 13)
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
#[doc = "ADC interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_ISR_SPEC;
impl crate::RegisterSpec for ADC_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_isr::R`](R) reader structure"]
impl crate::Readable for ADC_ISR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc_isr::W`](W) writer structure"]
impl crate::Writable for ADC_ISR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_ISR to value 0"]
impl crate::Resettable for ADC_ISR_SPEC {
    const RESET_VALUE: u32 = 0;
}
