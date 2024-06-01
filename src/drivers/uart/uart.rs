use core::fmt;
use crate::drivers::mmio::{mmio_read32, mmio_write32};

const CR_OFFSET:usize = 0x0;
const MR_OFFSET:usize = 0x4;
const IER_OFFSET:usize = 0x8;
const IDR_OFFSET:usize = 0xC;
const BAUDGEN_OFFSET:usize = 0x18;
const RXTOUT_OFFSET:usize = 0x1C;
const RXWM_OFFSET:usize = 0x20;
const MODEMCR_OFFSET:usize = 0x24;
const BAUD_DIV_OFFSET:usize = 0x34;
const FLOW_DELAY_OFFSET:usize = 0x38;
const TX_FIFO_TRIG_LV_OFFSET:usize = 0x44;
const SR_OFFSET:usize = 0x2C;
const FIFO_OFFSET:usize = 0x30;

const BM_SR_TXFULL:u32 = 0x00000010;
const BM_SR_RXEMPTY:u32 = 0x00000002;

pub struct StdioUart {
	base_addr:usize,
}

impl StdioUart {
	pub fn new(addr:usize) -> Self {
		Self {
			base_addr: addr,
		}
	}

	pub fn init_uart(&mut self) -> () {
    	mmio_write32(self.base_addr + CR_OFFSET, 0x00000114);
		mmio_write32(self.base_addr + MR_OFFSET, 0x00000020);
		mmio_write32(self.base_addr + IER_OFFSET, 0x00000000);
		mmio_write32(self.base_addr + IDR_OFFSET, 0x00000000);
		mmio_write32(self.base_addr + BAUDGEN_OFFSET, 0x0000007C);
		mmio_write32(self.base_addr + RXTOUT_OFFSET, 0x0000000A);
		mmio_write32(self.base_addr + RXWM_OFFSET, 0x00000038);
		mmio_write32(self.base_addr + MODEMCR_OFFSET, 0x00000003);
		mmio_write32(self.base_addr + BAUD_DIV_OFFSET, 0x00000006);
		mmio_write32(self.base_addr + FLOW_DELAY_OFFSET, 0x00000000);
		mmio_write32(self.base_addr + TX_FIFO_TRIG_LV_OFFSET, 0x00000020);
	}

	pub fn poll_out(&mut self, c:u8) {
		let mut reg_val:u32 = mmio_read32(self.base_addr + SR_OFFSET);

		while (reg_val & BM_SR_TXFULL) == BM_SR_TXFULL {
			reg_val = mmio_read32(self.base_addr + SR_OFFSET);
		}

		mmio_write32(self.base_addr + FIFO_OFFSET, c as u32);

		reg_val = mmio_read32(self.base_addr + SR_OFFSET);

		while (reg_val & BM_SR_TXFULL) == BM_SR_TXFULL {
			reg_val = mmio_read32(self.base_addr + SR_OFFSET);
		}
	}

	pub fn poll_in(&mut self) -> u8 {
		let c:u32;

		/* Wait until there is data */
		while (mmio_read32(self.base_addr + SR_OFFSET) &
				BM_SR_RXEMPTY) == BM_SR_RXEMPTY {
				
		}

		c = mmio_read32(self.base_addr+ FIFO_OFFSET);

		c as u8
	}
}

impl fmt::Write for StdioUart {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		for c in s.chars() {
			self.poll_out(c as u8);
		}

		Ok(())
	}
}

