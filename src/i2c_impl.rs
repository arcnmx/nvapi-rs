use std::cmp;
use {i2c, sys, PhysicalGpu};

pub struct I2c {
    inner: PhysicalGpu,
    display_mask: u32,
    port: Option<u8>,
    port_is_ddc: bool,
    address: u8,
    speed: sys::i2c::I2cSpeed,
}

impl I2c {
    pub fn new(gpu: PhysicalGpu, display_mask: u32) -> Self {
        I2c {
            inner: gpu,
            display_mask: display_mask,
            port: None,
            port_is_ddc: false,
            address: 0,
            speed: sys::i2c::I2cSpeed::_100Khz,
        }
    }

    pub fn set_port(&mut self, port: Option<u8>, port_is_ddc: bool) {
        self.port = port;
        self.port_is_ddc = port_is_ddc;
    }

    pub fn set_speed(&mut self, speed: sys::i2c::I2cSpeed) {
        self.speed = speed;
    }

    pub fn nvapi_read(&self, register: &[u8], bytes: &mut [u8]) -> sys::Result<usize> {
        // TODO: use i2c_read_ex if port_is_ddc is false? docs say it must be true here
        self.inner.i2c_read(
            self.display_mask,
            self.port, self.port_is_ddc,
            self.address,
            register, bytes,
            self.speed,
        )
    }

    pub fn nvapi_write(&self, register: &[u8], bytes: &[u8]) -> sys::Result<()> {
        // TODO: use i2c_write_ex if port_is_ddc is false? docs say it must be true here
        self.inner.i2c_write(
            self.display_mask,
            self.port, self.port_is_ddc,
            self.address,
            register, bytes,
            self.speed,
        )
    }
}

impl i2c::Master for I2c {
    type Error = sys::Status;
}

impl i2c::Address for I2c {
    fn set_slave_address(&mut self, addr: u16, tenbit: bool) -> sys::Result<()> {
        if tenbit {
            Err(sys::Status::InvalidArgument)
        } else {
            self.address = addr as u8;
            Ok(())
        }
    }
}

impl i2c::ReadWrite for I2c {
    fn i2c_read(&mut self, value: &mut [u8]) -> Result<usize, Self::Error> {
        self.nvapi_read(&[], value)
    }

    fn i2c_write(&mut self, value: &[u8]) -> Result<(), Self::Error> {
        self.nvapi_write(&[], value)
    }
}

impl i2c::Smbus for I2c {
    fn smbus_write_quick(&mut self, value: bool) -> Result<(), Self::Error> {
        if value {
            self.nvapi_read(&[], &mut []).map(drop)
        } else {
            self.nvapi_write(&[], &[])
        }
    }

    fn smbus_read_byte(&mut self) -> Result<u8, Self::Error> {
        let mut buf = [0];
        self.nvapi_read(&[], &mut buf)
            .map(|_| buf[0])
    }

    fn smbus_write_byte(&mut self, value: u8) -> Result<(), Self::Error> {
        self.nvapi_write(&[], &[value])
    }

    fn smbus_read_byte_data(&mut self, command: u8) -> Result<u8, Self::Error> {
        let mut buf = [0];
        self.nvapi_read(&[command], &mut buf)
            .map(|_| buf[0])
    }

    fn smbus_write_byte_data(&mut self, command: u8, value: u8) -> Result<(), Self::Error> {
        self.nvapi_write(&[command], &[value])
    }

    fn smbus_read_word_data(&mut self, command: u8) -> Result<u16, Self::Error> {
        let mut buf = [0, 0];
        self.nvapi_read(&[command], &mut buf)
            .map(|_| buf[0] as u16 | (buf[0] as u16) << 8)
    }

    fn smbus_write_word_data(&mut self, command: u8, value: u16) -> Result<(), Self::Error> {
        self.nvapi_write(&[command], &[value as u8, (value >> 8) as u8])
    }

    fn smbus_process_call(&mut self, command: u8, value: u16) -> Result<u16, Self::Error> {
        unimplemented!()
    }

    fn smbus_read_block_data(&mut self, command: u8, value: &mut [u8]) -> Result<usize, Self::Error> {
        let mut buf = [0; 33];
        self.nvapi_read(&[command], &mut buf)
            .map(|len| {
                let len = cmp::min(cmp::min(len, buf[0] as usize), value.len());
                value[..len].copy_from_slice(&buf[1..1 + len]);
                buf[0] as usize
            })
    }

    fn smbus_write_block_data(&mut self, command: u8, value: &[u8]) -> Result<(), Self::Error> {
        self.nvapi_write(&[command, value.len() as _], value)
    }
}

impl i2c::BlockTransfer for I2c {
    fn i2c_read_block_data(&mut self, command: u8, value: &mut [u8]) -> Result<usize, Self::Error> {
        // TODO: nvapi docs say with register set, value cannot be longer than 16 bytes??
        self.nvapi_read(&[command], value)
    }

    fn i2c_write_block_data(&mut self, command: u8, value: &[u8]) -> Result<(), Self::Error> {
        // TODO: nvapi docs say with register set, value cannot be longer than 16 bytes??
        self.nvapi_write(&[command], value)
    }
}
