use crate::drivers::mmio::*;

const UART_BASE_ADDR:ArchAddr = 0x8000000;
const CR_OFFSET:ArchAddr = 0x0;
const MR_OFFSET:ArchAddr = 0x4;
const IER_OFFSET:ArchAddr = 0x8;
const IDR_OFFSET:ArchAddr = 0xC;
const BAUDGEN_OFFSET:ArchAddr = 0x18;
const RXTOUT_OFFSET:ArchAddr = 0x1C;
const RXWM_OFFSET:ArchAddr = 0x20;
const MODEMCR_OFFSET:ArchAddr = 0x24;
const BAUD_DIV_OFFSET:ArchAddr = 0x34;
const FLOW_DELAY_OFFSET:ArchAddr = 0x38;
const TX_FIFO_TRIG_LV_OFFSET:ArchAddr = 0x44;

pub fn init_uart() -> () {
    mmio_write32(UART_BASE_ADDR  + CR_OFFSET, 0x00000114);
	mmio_write32(UART_BASE_ADDR  + MR_OFFSET, 0x00000020);
	mmio_write32(UART_BASE_ADDR  + IER_OFFSET, 0x00000000);
	mmio_write32(UART_BASE_ADDR  + IDR_OFFSET, 0x00000000);
	mmio_write32(UART_BASE_ADDR  + BAUDGEN_OFFSET, 0x0000007C);
	mmio_write32(UART_BASE_ADDR  + RXTOUT_OFFSET, 0x0000000A);
	mmio_write32(UART_BASE_ADDR  + RXWM_OFFSET, 0x00000038);
	mmio_write32(UART_BASE_ADDR  + MODEMCR_OFFSET, 0x00000003);
	mmio_write32(UART_BASE_ADDR  + BAUD_DIV_OFFSET, 0x00000006);
	mmio_write32(UART_BASE_ADDR  + FLOW_DELAY_OFFSET, 0x00000000);
	mmio_write32(UART_BASE_ADDR  + TX_FIFO_TRIG_LV_OFFSET, 0x00000020);
}