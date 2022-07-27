use std::cmp;
use std::borrow::Borrow;
use crate::{PhysicalGpu, sys};

pub struct I2c<G = PhysicalGpu> {
    inner: G,
    display_mask: u32,
    port: Option<u8>,
    port_is_ddc: bool,
    address: u8,
    speed: sys::i2c::I2cSpeed,
}

impl<G> I2c<G> {
    pub fn new(gpu: G, display_mask: u32) -> Self {
        I2c {
            inner: gpu,
            display_mask: display_mask,
            port: None,
            port_is_ddc: false,
            address: 0,
            speed: sys::i2c::I2cSpeed::_100Khz,
        }
    }

    pub fn set_display_mask(&mut self, display_mask: u32) {
        self.display_mask = display_mask;
    }

    pub fn set_address(&mut self, address: u8) {
        self.address = address;
    }

    pub fn set_port(&mut self, port: Option<u8>, port_is_ddc: bool) {
        self.port = port;
        self.port_is_ddc = port_is_ddc;
    }

    pub fn set_speed(&mut self, speed: sys::i2c::I2cSpeed) {
        self.speed = speed;
    }
}

impl<G: Borrow<PhysicalGpu>> I2c<G> {
    pub fn nvapi_read(&self, register: &[u8], bytes: &mut [u8]) -> crate::NvapiResult<usize> {
        // TODO: use i2c_read_ex if port_is_ddc is false? docs say it must be true here
        self.inner.borrow().i2c_read(
            self.display_mask,
            self.port, self.port_is_ddc,
            self.address,
            register, bytes,
            self.speed,
        )
    }

    pub fn nvapi_write(&self, register: &[u8], bytes: &[u8]) -> crate::NvapiResult<()> {
        // TODO: use i2c_write_ex if port_is_ddc is false? docs say it must be true here
        self.inner.borrow().i2c_write(
            self.display_mask,
            self.port, self.port_is_ddc,
            self.address,
            register, bytes,
            self.speed,
        )
    }
}

impl<G> i2c::Master for I2c<G> {
    type Error = crate::Error;
}

impl<G> i2c::Address for I2c<G> {
    fn set_slave_address(&mut self, addr: u16, tenbit: bool) -> crate::Result<()> {
        if tenbit {
            Err(sys::ArgumentRangeError.into())
        } else {
            self.address = addr as u8;
            Ok(())
        }
    }
}

impl<G: Borrow<PhysicalGpu>> i2c::ReadWrite for I2c<G> {
    fn i2c_read(&mut self, value: &mut [u8]) -> Result<usize, Self::Error> {
        self.nvapi_read(&[], value)
            .map_err(Into::into)
    }

    fn i2c_write(&mut self, value: &[u8]) -> Result<(), Self::Error> {
        self.nvapi_write(&[], value)
            .map_err(Into::into)
    }
}

impl<G: Borrow<PhysicalGpu>> i2c::Smbus for I2c<G> {
    fn smbus_write_quick(&mut self, value: bool) -> Result<(), Self::Error> {
        if value {
            self.nvapi_read(&[], &mut []).map(drop)
        } else {
            self.nvapi_write(&[], &[])
        }.map_err(Into::into)
    }

    fn smbus_read_byte(&mut self) -> Result<u8, Self::Error> {
        let mut buf = [0];
        self.nvapi_read(&[], &mut buf)
            .map_err(Into::into)
            .map(|_| buf[0])
    }

    fn smbus_write_byte(&mut self, value: u8) -> Result<(), Self::Error> {
        self.nvapi_write(&[], &[value])
            .map_err(Into::into)
    }

    fn smbus_read_byte_data(&mut self, command: u8) -> Result<u8, Self::Error> {
        let mut buf = [0];
        self.nvapi_read(&[command], &mut buf)
            .map_err(Into::into)
            .map(|_| buf[0])
    }

    fn smbus_write_byte_data(&mut self, command: u8, value: u8) -> Result<(), Self::Error> {
        self.nvapi_write(&[command], &[value])
            .map_err(Into::into)
    }

    fn smbus_read_word_data(&mut self, command: u8) -> Result<u16, Self::Error> {
        let mut buf = [0, 0];
        self.nvapi_read(&[command], &mut buf)
            .map_err(Into::into)
            .map(|_| buf[0] as u16 | (buf[0] as u16) << 8)
    }

    fn smbus_write_word_data(&mut self, command: u8, value: u16) -> Result<(), Self::Error> {
        self.nvapi_write(&[command], &[value as u8, (value >> 8) as u8])
            .map_err(Into::into)
    }

    fn smbus_process_call(&mut self, command: u8, value: u16) -> Result<u16, Self::Error> {
        unimplemented!()
    }

    fn smbus_read_block_data(&mut self, command: u8, value: &mut [u8]) -> Result<usize, Self::Error> {
        let mut buf = [0; 33];
        self.nvapi_read(&[command], &mut buf)
            .map_err(Into::into)
            .map(|len| {
                let len = cmp::min(cmp::min(len, buf[0] as usize), value.len());
                value[..len].copy_from_slice(&buf[1..1 + len]);
                buf[0] as usize
            })
    }

    fn smbus_write_block_data(&mut self, command: u8, value: &[u8]) -> Result<(), Self::Error> {
        self.nvapi_write(&[command, value.len() as _], value)
            .map_err(Into::into)
    }
}

impl<G: Borrow<PhysicalGpu>> i2c::BlockTransfer for I2c<G> {
    fn i2c_read_block_data(&mut self, command: u8, value: &mut [u8]) -> Result<usize, Self::Error> {
        // TODO: nvapi docs say with register set, value cannot be longer than 16 bytes??
        self.nvapi_read(&[command], value)
            .map_err(Into::into)
    }

    fn i2c_write_block_data(&mut self, command: u8, value: &[u8]) -> Result<(), Self::Error> {
        // TODO: nvapi docs say with register set, value cannot be longer than 16 bytes??
        self.nvapi_write(&[command], value)
            .map_err(Into::into)
    }
}
