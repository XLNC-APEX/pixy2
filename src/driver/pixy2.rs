use bytemuck::{Pod, Zeroable};
use embedded_hal_async::spi::SpiDevice;

use crate::{Result, error::Error, packets::Request};

pub struct Pixy2<SPI> {
    spi: SPI,
}

impl<SPI> Pixy2<SPI>
where
    SPI: SpiDevice,
{
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }

    pub async fn init(&mut self) -> Result<()> {
        let _ = self.get_version().await?;
        Ok(())
    }

    /// Skips all garbage data before the needed data
    async fn sync(&mut self) -> Result<()> {
        let mut rx = [0u8; 1];
        self.spi.read(&mut rx).await.map_err(|_| Error::SpiError)?;
        let mut prev = rx;
        for _ in 0..127 {
            self.spi.read(&mut rx).await.map_err(|_| Error::SpiError)?;
            if (rx[0] == 0xC1) && (prev[0] == 0xAF) {
                // Skipping header
                for _ in 0..4 {
                    self.spi.read(&mut rx).await.map_err(|_| Error::SpiError)?;
                }
                return Ok(());
            }
            prev[0] = rx[0];
        }
        Err(Error::Timeout)
    }

    pub async fn get_version(&mut self) -> Result<Version> {
        Ok(bytemuck::cast(self.read::<16>(Request::VERSION).await?))
    }

    async fn read<const N: usize>(&mut self, request: u8) -> Result<[u8; N]> {
        let tx: [u8; 4] = [
            0xae, // first byte of no_checksum_sync (little endian -> least-significant byte first)
            0xc1, // second byte of no_checksum_sync
            request, 0x00, // data_length is 0
        ];
        let mut rx = [0u8; N];
        self.spi.write(&tx).await.map_err(|_| Error::SpiError)?;
        self.sync().await?;
        self.spi.read(&mut rx).await.map_err(|_| Error::SpiError)?;
        Ok(rx)
    }
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Version {
    pub hw: u16,
    pub fw_major: u8,
    pub fw_minor: u8,
    pub fw_build: u16,
    pub fw_type_str: [u8; 10],
}
#[cfg(feature = "defmt")]
impl defmt::Format for Version {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "hw: {} fw:{}.{} build: {} type: {:a}",
            self.hw,
            self.fw_major,
            self.fw_minor,
            self.fw_build,
            self.fw_type_str,
        )
    }
}
