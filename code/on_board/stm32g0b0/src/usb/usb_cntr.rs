#[doc = "Register `USB_CNTR` reader"]
pub type R = crate::R<USB_CNTR_SPEC>;
#[doc = "Register `USB_CNTR` writer"]
pub type W = crate::W<USB_CNTR_SPEC>;
#[doc = "Field `USBRST` reader - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software."]
pub type USBRST_R = crate::BitReader<USBRST_A>;
#[doc = "USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBRST_A {
    #[doc = "0: No effect"]
    B_0X0_DEVICE_MODE = 0,
    #[doc = "1: USB core is under reset"]
    B_0X1_DEVICE_MODE = 1,
}
impl From<USBRST_A> for bool {
    #[inline(always)]
    fn from(variant: USBRST_A) -> Self {
        variant as u8 != 0
    }
}
impl USBRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBRST_A {
        match self.bits {
            false => USBRST_A::B_0X0_DEVICE_MODE,
            true => USBRST_A::B_0X1_DEVICE_MODE,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0_device_mode(&self) -> bool {
        *self == USBRST_A::B_0X0_DEVICE_MODE
    }
    #[doc = "USB core is under reset"]
    #[inline(always)]
    pub fn is_b_0x1_device_mode(&self) -> bool {
        *self == USBRST_A::B_0X1_DEVICE_MODE
    }
}
#[doc = "Field `USBRST` writer - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software."]
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG, USBRST_A>;
impl<'a, REG> USBRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0_device_mode(self) -> &'a mut crate::W<REG> {
        self.variant(USBRST_A::B_0X0_DEVICE_MODE)
    }
    #[doc = "USB core is under reset"]
    #[inline(always)]
    pub fn b_0x1_device_mode(self) -> &'a mut crate::W<REG> {
        self.variant(USBRST_A::B_0X1_DEVICE_MODE)
    }
}
#[doc = "Field `PDWN` reader - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used."]
pub type PDWN_R = crate::BitReader<PDWN_A>;
#[doc = "Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDWN_A {
    #[doc = "0: Exit power down."]
    B_0X0 = 0,
    #[doc = "1: Enter power down mode."]
    B_0X1 = 1,
}
impl From<PDWN_A> for bool {
    #[inline(always)]
    fn from(variant: PDWN_A) -> Self {
        variant as u8 != 0
    }
}
impl PDWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDWN_A {
        match self.bits {
            false => PDWN_A::B_0X0,
            true => PDWN_A::B_0X1,
        }
    }
    #[doc = "Exit power down."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PDWN_A::B_0X0
    }
    #[doc = "Enter power down mode."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PDWN_A::B_0X1
    }
}
#[doc = "Field `PDWN` writer - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used."]
pub type PDWN_W<'a, REG> = crate::BitWriter<'a, REG, PDWN_A>;
impl<'a, REG> PDWN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Exit power down."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PDWN_A::B_0X0)
    }
    #[doc = "Enter power down mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PDWN_A::B_0X1)
    }
}
#[doc = "Field `SUSPRDY` reader - Suspend state effective This bit is set by hardware as soon as the suspend state entered through the SUSPEN control gets internally effective. In this state USB activity is suspended, USB clock is gated, transceiver is set in low power mode by disabling the differential receiver. Only asynchronous wakeup logic and single ended receiver is kept alive to detect remote wakeup or resume events. Software must poll this bit to confirm it to be set before any STOP mode entry. This bit is cleared by hardware simultaneously to the WAKEUP flag being set."]
pub type SUSPRDY_R = crate::BitReader<SUSPRDY_A>;
#[doc = "Suspend state effective This bit is set by hardware as soon as the suspend state entered through the SUSPEN control gets internally effective. In this state USB activity is suspended, USB clock is gated, transceiver is set in low power mode by disabling the differential receiver. Only asynchronous wakeup logic and single ended receiver is kept alive to detect remote wakeup or resume events. Software must poll this bit to confirm it to be set before any STOP mode entry. This bit is cleared by hardware simultaneously to the WAKEUP flag being set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPRDY_A {
    #[doc = "0: Normal operation"]
    B_0X0 = 0,
    #[doc = "1: Suspend state"]
    B_0X1 = 1,
}
impl From<SUSPRDY_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl SUSPRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUSPRDY_A {
        match self.bits {
            false => SUSPRDY_A::B_0X0,
            true => SUSPRDY_A::B_0X1,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SUSPRDY_A::B_0X0
    }
    #[doc = "Suspend state"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SUSPRDY_A::B_0X1
    }
}
#[doc = "Field `SUSPEN` reader - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY=1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set."]
pub type SUSPEN_R = crate::BitReader<SUSPEN_A>;
#[doc = "Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY=1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPEN_A {
    #[doc = "0: No effect."]
    B_0X0_DEVICE_MODE = 0,
    #[doc = "1: Enter L1/L2 suspend"]
    B_0X1_DEVICE_MODE = 1,
}
impl From<SUSPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SUSPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUSPEN_A {
        match self.bits {
            false => SUSPEN_A::B_0X0_DEVICE_MODE,
            true => SUSPEN_A::B_0X1_DEVICE_MODE,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_b_0x0_device_mode(&self) -> bool {
        *self == SUSPEN_A::B_0X0_DEVICE_MODE
    }
    #[doc = "Enter L1/L2 suspend"]
    #[inline(always)]
    pub fn is_b_0x1_device_mode(&self) -> bool {
        *self == SUSPEN_A::B_0X1_DEVICE_MODE
    }
}
#[doc = "Field `SUSPEN` writer - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY=1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set."]
pub type SUSPEN_W<'a, REG> = crate::BitWriter<'a, REG, SUSPEN_A>;
impl<'a, REG> SUSPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn b_0x0_device_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPEN_A::B_0X0_DEVICE_MODE)
    }
    #[doc = "Enter L1/L2 suspend"]
    #[inline(always)]
    pub fn b_0x1_device_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPEN_A::B_0X1_DEVICE_MODE)
    }
}
#[doc = "Field `L2RES` reader - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1ms and no more than 15ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt."]
pub type L2RES_R = crate::BitReader<L2RES_A>;
#[doc = "L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1ms and no more than 15ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2RES_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Send L2 resume signaling to device"]
    B_0X1 = 1,
}
impl From<L2RES_A> for bool {
    #[inline(always)]
    fn from(variant: L2RES_A) -> Self {
        variant as u8 != 0
    }
}
impl L2RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L2RES_A {
        match self.bits {
            false => L2RES_A::B_0X0,
            true => L2RES_A::B_0X1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == L2RES_A::B_0X0
    }
    #[doc = "Send L2 resume signaling to device"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == L2RES_A::B_0X1
    }
}
#[doc = "Field `L2RES` writer - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1ms and no more than 15ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt."]
pub type L2RES_W<'a, REG> = crate::BitWriter<'a, REG, L2RES_A>;
impl<'a, REG> L2RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(L2RES_A::B_0X0)
    }
    #[doc = "Send L2 resume signaling to device"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(L2RES_A::B_0X1)
    }
}
#[doc = "Field `L1RES` reader - L1 remote wakeup / resume driver Device mode"]
pub type L1RES_R = crate::BitReader<L1RES_A>;
#[doc = "L1 remote wakeup / resume driver Device mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1RES_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Send 50 remote-wakeup signaling to host"]
    B_0X1 = 1,
}
impl From<L1RES_A> for bool {
    #[inline(always)]
    fn from(variant: L1RES_A) -> Self {
        variant as u8 != 0
    }
}
impl L1RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L1RES_A {
        match self.bits {
            false => L1RES_A::B_0X0,
            true => L1RES_A::B_0X1,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == L1RES_A::B_0X0
    }
    #[doc = "Send 50 remote-wakeup signaling to host"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == L1RES_A::B_0X1
    }
}
#[doc = "Field `L1RES` writer - L1 remote wakeup / resume driver Device mode"]
pub type L1RES_W<'a, REG> = crate::BitWriter<'a, REG, L1RES_A>;
impl<'a, REG> L1RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(L1RES_A::B_0X0)
    }
    #[doc = "Send 50 remote-wakeup signaling to host"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(L1RES_A::B_0X1)
    }
}
#[doc = "Field `L1REQM` reader - LPM L1 state request interrupt mask"]
pub type L1REQM_R = crate::BitReader<L1REQM_A>;
#[doc = "LPM L1 state request interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1REQM_A {
    #[doc = "0: LPM L1 state request (L1REQ) interrupt disabled."]
    B_0X0 = 0,
    #[doc = "1: L1REQ interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B_0X1 = 1,
}
impl From<L1REQM_A> for bool {
    #[inline(always)]
    fn from(variant: L1REQM_A) -> Self {
        variant as u8 != 0
    }
}
impl L1REQM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L1REQM_A {
        match self.bits {
            false => L1REQM_A::B_0X0,
            true => L1REQM_A::B_0X1,
        }
    }
    #[doc = "LPM L1 state request (L1REQ) interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == L1REQM_A::B_0X0
    }
    #[doc = "L1REQ interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == L1REQM_A::B_0X1
    }
}
#[doc = "Field `L1REQM` writer - LPM L1 state request interrupt mask"]
pub type L1REQM_W<'a, REG> = crate::BitWriter<'a, REG, L1REQM_A>;
impl<'a, REG> L1REQM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPM L1 state request (L1REQ) interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(L1REQM_A::B_0X0)
    }
    #[doc = "L1REQ interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(L1REQM_A::B_0X1)
    }
}
#[doc = "Field `ESOFM` reader - Expected start of frame interrupt mask"]
pub type ESOFM_R = crate::BitReader<ESOFM_A>;
#[doc = "Expected start of frame interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESOFM_A {
    #[doc = "0: Expected start of frame (ESOF) interrupt disabled."]
    B_0X0 = 0,
    #[doc = "1: ESOF interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B_0X1 = 1,
}
impl From<ESOFM_A> for bool {
    #[inline(always)]
    fn from(variant: ESOFM_A) -> Self {
        variant as u8 != 0
    }
}
impl ESOFM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ESOFM_A {
        match self.bits {
            false => ESOFM_A::B_0X0,
            true => ESOFM_A::B_0X1,
        }
    }
    #[doc = "Expected start of frame (ESOF) interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ESOFM_A::B_0X0
    }
    #[doc = "ESOF interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ESOFM_A::B_0X1
    }
}
#[doc = "Field `ESOFM` writer - Expected start of frame interrupt mask"]
pub type ESOFM_W<'a, REG> = crate::BitWriter<'a, REG, ESOFM_A>;
impl<'a, REG> ESOFM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Expected start of frame (ESOF) interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ESOFM_A::B_0X0)
    }
    #[doc = "ESOF interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ESOFM_A::B_0X1)
    }
}
#[doc = "Field `SOFM` reader - Start of frame interrupt mask"]
pub type SOFM_R = crate::BitReader<SOFM_A>;
#[doc = "Start of frame interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFM_A {
    #[doc = "0: SOF interrupt disabled."]
    B_0X0 = 0,
    #[doc = "1: SOF interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B_0X1 = 1,
}
impl From<SOFM_A> for bool {
    #[inline(always)]
    fn from(variant: SOFM_A) -> Self {
        variant as u8 != 0
    }
}
impl SOFM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOFM_A {
        match self.bits {
            false => SOFM_A::B_0X0,
            true => SOFM_A::B_0X1,
        }
    }
    #[doc = "SOF interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SOFM_A::B_0X0
    }
    #[doc = "SOF interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SOFM_A::B_0X1
    }
}
#[doc = "Field `SOFM` writer - Start of frame interrupt mask"]
pub type SOFM_W<'a, REG> = crate::BitWriter<'a, REG, SOFM_A>;
impl<'a, REG> SOFM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SOF interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SOFM_A::B_0X0)
    }
    #[doc = "SOF interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SOFM_A::B_0X1)
    }
}
#[doc = "Field `RST_DCONM` reader - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask"]
pub type RST_DCONM_R = crate::BitReader<RST_DCONM_A>;
#[doc = "USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_DCONM_A {
    #[doc = "0: RESET interrupt disabled."]
    B_0X0 = 0,
    #[doc = "1: RESET interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B_0X1 = 1,
}
impl From<RST_DCONM_A> for bool {
    #[inline(always)]
    fn from(variant: RST_DCONM_A) -> Self {
        variant as u8 != 0
    }
}
impl RST_DCONM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RST_DCONM_A {
        match self.bits {
            false => RST_DCONM_A::B_0X0,
            true => RST_DCONM_A::B_0X1,
        }
    }
    #[doc = "RESET interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RST_DCONM_A::B_0X0
    }
    #[doc = "RESET interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RST_DCONM_A::B_0X1
    }
}
#[doc = "Field `RST_DCONM` writer - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask"]
pub type RST_DCONM_W<'a, REG> = crate::BitWriter<'a, REG, RST_DCONM_A>;
impl<'a, REG> RST_DCONM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RESET interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RST_DCONM_A::B_0X0)
    }
    #[doc = "RESET interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RST_DCONM_A::B_0X1)
    }
}
#[doc = "Field `SUSPM` reader - Suspend mode interrupt mask"]
pub type SUSPM_R = crate::BitReader<SUSPM_A>;
#[doc = "Suspend mode interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPM_A {
    #[doc = "0: Suspend mode request (SUSP) interrupt disabled."]
    B_0X0 = 0,
    #[doc = "1: SUSP interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B_0X1 = 1,
}
impl From<SUSPM_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPM_A) -> Self {
        variant as u8 != 0
    }
}
impl SUSPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUSPM_A {
        match self.bits {
            false => SUSPM_A::B_0X0,
            true => SUSPM_A::B_0X1,
        }
    }
    #[doc = "Suspend mode request (SUSP) interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SUSPM_A::B_0X0
    }
    #[doc = "SUSP interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SUSPM_A::B_0X1
    }
}
#[doc = "Field `SUSPM` writer - Suspend mode interrupt mask"]
pub type SUSPM_W<'a, REG> = crate::BitWriter<'a, REG, SUSPM_A>;
impl<'a, REG> SUSPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Suspend mode request (SUSP) interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPM_A::B_0X0)
    }
    #[doc = "SUSP interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPM_A::B_0X1)
    }
}
#[doc = "Field `WKUPM` reader - Wakeup interrupt mask"]
pub type WKUPM_R = crate::BitReader<WKUPM_A>;
#[doc = "Wakeup interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WKUPM_A {
    #[doc = "0: WKUP interrupt disabled."]
    B_0X0 = 0,
    #[doc = "1: WKUP interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B_0X1 = 1,
}
impl From<WKUPM_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPM_A) -> Self {
        variant as u8 != 0
    }
}
impl WKUPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WKUPM_A {
        match self.bits {
            false => WKUPM_A::B_0X0,
            true => WKUPM_A::B_0X1,
        }
    }
    #[doc = "WKUP interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WKUPM_A::B_0X0
    }
    #[doc = "WKUP interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WKUPM_A::B_0X1
    }
}
#[doc = "Field `WKUPM` writer - Wakeup interrupt mask"]
pub type WKUPM_W<'a, REG> = crate::BitWriter<'a, REG, WKUPM_A>;
impl<'a, REG> WKUPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WKUP interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPM_A::B_0X0)
    }
    #[doc = "WKUP interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WKUPM_A::B_0X1)
    }
}
#[doc = "Field `ERRM` reader - Error interrupt mask"]
pub type ERRM_R = crate::BitReader<ERRM_A>;
#[doc = "Error interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRM_A {
    #[doc = "0: ERR interrupt disabled."]
    B_0X0 = 0,
    #[doc = "1: ERR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B_0X1 = 1,
}
impl From<ERRM_A> for bool {
    #[inline(always)]
    fn from(variant: ERRM_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRM_A {
        match self.bits {
            false => ERRM_A::B_0X0,
            true => ERRM_A::B_0X1,
        }
    }
    #[doc = "ERR interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ERRM_A::B_0X0
    }
    #[doc = "ERR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ERRM_A::B_0X1
    }
}
#[doc = "Field `ERRM` writer - Error interrupt mask"]
pub type ERRM_W<'a, REG> = crate::BitWriter<'a, REG, ERRM_A>;
impl<'a, REG> ERRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ERR interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ERRM_A::B_0X0)
    }
    #[doc = "ERR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ERRM_A::B_0X1)
    }
}
#[doc = "Field `PMAOVRM` reader - Packet memory area over / underrun interrupt mask"]
pub type PMAOVRM_R = crate::BitReader<PMAOVRM_A>;
#[doc = "Packet memory area over / underrun interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMAOVRM_A {
    #[doc = "0: PMAOVR interrupt disabled."]
    B_0X0 = 0,
    #[doc = "1: PMAOVR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B_0X1 = 1,
}
impl From<PMAOVRM_A> for bool {
    #[inline(always)]
    fn from(variant: PMAOVRM_A) -> Self {
        variant as u8 != 0
    }
}
impl PMAOVRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PMAOVRM_A {
        match self.bits {
            false => PMAOVRM_A::B_0X0,
            true => PMAOVRM_A::B_0X1,
        }
    }
    #[doc = "PMAOVR interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PMAOVRM_A::B_0X0
    }
    #[doc = "PMAOVR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PMAOVRM_A::B_0X1
    }
}
#[doc = "Field `PMAOVRM` writer - Packet memory area over / underrun interrupt mask"]
pub type PMAOVRM_W<'a, REG> = crate::BitWriter<'a, REG, PMAOVRM_A>;
impl<'a, REG> PMAOVRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PMAOVR interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PMAOVRM_A::B_0X0)
    }
    #[doc = "PMAOVR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PMAOVRM_A::B_0X1)
    }
}
#[doc = "Field `CTRM` reader - Correct transfer interrupt mask"]
pub type CTRM_R = crate::BitReader<CTRM_A>;
#[doc = "Correct transfer interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTRM_A {
    #[doc = "0: Correct transfer (CTR) interrupt disabled."]
    B_0X0 = 0,
    #[doc = "1: CTR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    B_0X1 = 1,
}
impl From<CTRM_A> for bool {
    #[inline(always)]
    fn from(variant: CTRM_A) -> Self {
        variant as u8 != 0
    }
}
impl CTRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTRM_A {
        match self.bits {
            false => CTRM_A::B_0X0,
            true => CTRM_A::B_0X1,
        }
    }
    #[doc = "Correct transfer (CTR) interrupt disabled."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTRM_A::B_0X0
    }
    #[doc = "CTR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTRM_A::B_0X1
    }
}
#[doc = "Field `CTRM` writer - Correct transfer interrupt mask"]
pub type CTRM_W<'a, REG> = crate::BitWriter<'a, REG, CTRM_A>;
impl<'a, REG> CTRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Correct transfer (CTR) interrupt disabled."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CTRM_A::B_0X0)
    }
    #[doc = "CTR interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CTRM_A::B_0X1)
    }
}
#[doc = "Field `THR512M` reader - 512 byte threshold interrupt mask"]
pub type THR512M_R = crate::BitReader<THR512M_A>;
#[doc = "512 byte threshold interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THR512M_A {
    #[doc = "0: 512 byte threshold interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: 512 byte threshold interrupt enabled"]
    B_0X1 = 1,
}
impl From<THR512M_A> for bool {
    #[inline(always)]
    fn from(variant: THR512M_A) -> Self {
        variant as u8 != 0
    }
}
impl THR512M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> THR512M_A {
        match self.bits {
            false => THR512M_A::B_0X0,
            true => THR512M_A::B_0X1,
        }
    }
    #[doc = "512 byte threshold interrupt disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == THR512M_A::B_0X0
    }
    #[doc = "512 byte threshold interrupt enabled"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == THR512M_A::B_0X1
    }
}
#[doc = "Field `THR512M` writer - 512 byte threshold interrupt mask"]
pub type THR512M_W<'a, REG> = crate::BitWriter<'a, REG, THR512M_A>;
impl<'a, REG> THR512M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "512 byte threshold interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(THR512M_A::B_0X0)
    }
    #[doc = "512 byte threshold interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(THR512M_A::B_0X1)
    }
}
#[doc = "Field `HOST` reader - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit."]
pub type HOST_R = crate::BitReader<HOST_A>;
#[doc = "HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOST_A {
    #[doc = "0: USB Device function"]
    B_0X0 = 0,
    #[doc = "1: USB host function"]
    B_0X1 = 1,
}
impl From<HOST_A> for bool {
    #[inline(always)]
    fn from(variant: HOST_A) -> Self {
        variant as u8 != 0
    }
}
impl HOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HOST_A {
        match self.bits {
            false => HOST_A::B_0X0,
            true => HOST_A::B_0X1,
        }
    }
    #[doc = "USB Device function"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HOST_A::B_0X0
    }
    #[doc = "USB host function"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HOST_A::B_0X1
    }
}
#[doc = "Field `HOST` writer - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit."]
pub type HOST_W<'a, REG> = crate::BitWriter<'a, REG, HOST_A>;
impl<'a, REG> HOST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB Device function"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HOST_A::B_0X0)
    }
    #[doc = "USB host function"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HOST_A::B_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software."]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used."]
    #[inline(always)]
    pub fn pdwn(&self) -> PDWN_R {
        PDWN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Suspend state effective This bit is set by hardware as soon as the suspend state entered through the SUSPEN control gets internally effective. In this state USB activity is suspended, USB clock is gated, transceiver is set in low power mode by disabling the differential receiver. Only asynchronous wakeup logic and single ended receiver is kept alive to detect remote wakeup or resume events. Software must poll this bit to confirm it to be set before any STOP mode entry. This bit is cleared by hardware simultaneously to the WAKEUP flag being set."]
    #[inline(always)]
    pub fn susprdy(&self) -> SUSPRDY_R {
        SUSPRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY=1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set."]
    #[inline(always)]
    pub fn suspen(&self) -> SUSPEN_R {
        SUSPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1ms and no more than 15ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt."]
    #[inline(always)]
    pub fn l2res(&self) -> L2RES_R {
        L2RES_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - L1 remote wakeup / resume driver Device mode"]
    #[inline(always)]
    pub fn l1res(&self) -> L1RES_R {
        L1RES_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - LPM L1 state request interrupt mask"]
    #[inline(always)]
    pub fn l1reqm(&self) -> L1REQM_R {
        L1REQM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline(always)]
    pub fn esofm(&self) -> ESOFM_R {
        ESOFM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask"]
    #[inline(always)]
    pub fn rst_dconm(&self) -> RST_DCONM_R {
        RST_DCONM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    pub fn suspm(&self) -> SUSPM_R {
        SUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    pub fn wkupm(&self) -> WKUPM_R {
        WKUPM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    pub fn errm(&self) -> ERRM_R {
        ERRM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline(always)]
    pub fn pmaovrm(&self) -> PMAOVRM_R {
        PMAOVRM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline(always)]
    pub fn ctrm(&self) -> CTRM_R {
        CTRM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 512 byte threshold interrupt mask"]
    #[inline(always)]
    pub fn thr512m(&self) -> THR512M_R {
        THR512M_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit."]
    #[inline(always)]
    pub fn host(&self) -> HOST_R {
        HOST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Reset Software can set this bit to reset the USB core, exactly as it happens when receiving a RESET signaling on the USB.The USB peripheral, in response to a RESET, resets its internal protocol state machine. Reception and transmission are disabled until the RST_DCON bit is cleared. All configuration registers do not reset: the microcontroller must explicitly clear these registers (this is to ensure that the RST_DCON interrupt can be safely delivered, and any transaction immediately followed by a RESET can be completed). The function address and endpoint registers are reset by an USB reset event. Software sets this bit to drive USB reset state on the bus and initialize the device. USB reset terminates as soon as this bit is cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<USB_CNTR_SPEC> {
        USBRST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Power down This bit is used to completely switch off all USB-related analog parts if it is required to completely disable the USB peripheral for any reason. When this bit is set, the USB peripheral is disconnected from the transceivers and it cannot be used."]
    #[inline(always)]
    #[must_use]
    pub fn pdwn(&mut self) -> PDWN_W<USB_CNTR_SPEC> {
        PDWN_W::new(self, 1)
    }
    #[doc = "Bit 3 - Suspend state enable Software can set this bit when the SUSP interrupt is received, which is issued when no traffic is received by the USB peripheral for 3ms. Software can also set this bit when the L1REQ interrupt is received with positive acknowledge sent. As soon as the suspend state is propagated internally all device activity is stopped, USB clock is gated, USB transceiver is set into low power mode and the SUSPRDY bit is set by hardware. In the case that device application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the microcontroller to stop mode, as in the case of bus powered device application, it must first wait few cycles to see the SUSPRDY=1 acknowledge the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set. Software can set this bit when host application has nothing scheduled for the next frames and wants to enter long term power saving. When set, it stops immediately SOF generation and any other host activity, gates the USB clock and sets the transceiver in low power mode. If any USB transaction is on-going at the time SUSPEN is set, suspend is entered at the end of the current transaction. As soon as suspend state is propagated internally and gets effective the SUSPRDY bit is set. In the case that host application wants to pursue more aggressive power saving by stopping the USB clock source and by moving the micro-controller to STOP mode, it must first wait few cycles to see SUSPRDY=1 acknowledge to the suspend request. This bit is cleared by hardware simultaneous with the WAKEUP flag set."]
    #[inline(always)]
    #[must_use]
    pub fn suspen(&mut self) -> SUSPEN_W<USB_CNTR_SPEC> {
        SUSPEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - L2 remote wakeup / resume driver Device mode The microcontroller can set this bit to send remote wake-up signaling to the host. It must be activated, according to USB specifications, for no less than 1ms and no more than 15ms after which the host PC is ready to drive the resume sequence up to its end. Host mode Software sets this bit to send resume signaling to the device. Software clears this bit to send end of resume to device and restart SOF generation. In the context of remote wake up, this bit is to be set following the WAKEUP interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn l2res(&mut self) -> L2RES_W<USB_CNTR_SPEC> {
        L2RES_W::new(self, 4)
    }
    #[doc = "Bit 5 - L1 remote wakeup / resume driver Device mode"]
    #[inline(always)]
    #[must_use]
    pub fn l1res(&mut self) -> L1RES_W<USB_CNTR_SPEC> {
        L1RES_W::new(self, 5)
    }
    #[doc = "Bit 7 - LPM L1 state request interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn l1reqm(&mut self) -> L1REQM_W<USB_CNTR_SPEC> {
        L1REQM_W::new(self, 7)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn esofm(&mut self) -> ESOFM_W<USB_CNTR_SPEC> {
        ESOFM_W::new(self, 8)
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn sofm(&mut self) -> SOFM_W<USB_CNTR_SPEC> {
        SOFM_W::new(self, 9)
    }
    #[doc = "Bit 10 - USB reset request (Device mode) or device connect/disconnect (Host mode) interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn rst_dconm(&mut self) -> RST_DCONM_W<USB_CNTR_SPEC> {
        RST_DCONM_W::new(self, 10)
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn suspm(&mut self) -> SUSPM_W<USB_CNTR_SPEC> {
        SUSPM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn wkupm(&mut self) -> WKUPM_W<USB_CNTR_SPEC> {
        WKUPM_W::new(self, 12)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn errm(&mut self) -> ERRM_W<USB_CNTR_SPEC> {
        ERRM_W::new(self, 13)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn pmaovrm(&mut self) -> PMAOVRM_W<USB_CNTR_SPEC> {
        PMAOVRM_W::new(self, 14)
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ctrm(&mut self) -> CTRM_W<USB_CNTR_SPEC> {
        CTRM_W::new(self, 15)
    }
    #[doc = "Bit 16 - 512 byte threshold interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn thr512m(&mut self) -> THR512M_W<USB_CNTR_SPEC> {
        THR512M_W::new(self, 16)
    }
    #[doc = "Bit 31 - HOST mode HOST bit selects betweens host or device USB mode of operation. It must be set before enabling the USB peripheral by the function enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn host(&mut self) -> HOST_W<USB_CNTR_SPEC> {
        HOST_W::new(self, 31)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_cntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_cntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_CNTR_SPEC;
impl crate::RegisterSpec for USB_CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_cntr::R`](R) reader structure"]
impl crate::Readable for USB_CNTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_cntr::W`](W) writer structure"]
impl crate::Writable for USB_CNTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB_CNTR to value 0x03"]
impl crate::Resettable for USB_CNTR_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
