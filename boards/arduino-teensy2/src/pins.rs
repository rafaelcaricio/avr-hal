use atmega32u4_hal::port::PortExt;

avr_hal_generic::impl_board_pins! {
    #[port_defs]
    use atmega32u4_hal::port;

    /// Generic DDR that works for all ports
    pub struct DDR {
        portb: crate::atmega32u4::PORTB,
        portc: crate::atmega32u4::PORTC,
        portd: crate::atmega32u4::PORTD,
        porte: crate::atmega32u4::PORTE,
        portf: crate::atmega32u4::PORTF,
    }

    /// Reexport of the Teensy's pins, with the names they have on the board
    pub struct Pins {
        /// `D2` / `RX`
        ///
        /// * `RX` (UART)
        /// * `INT2`: External Interrupt
        pub d2: portd::pd2::PD2,
        /// `D3` / `TX`
        ///
        /// * `TX` (UART)
        /// * `INT3`: External Interrupt
        pub d3: portd::pd3::PD3,
        /// `D6` / `LED_BUILTIN`
        ///
        /// * Onboard LED
        pub led: portd::pd6::PD6,
    }
}
