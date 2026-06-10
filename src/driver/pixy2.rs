use bytemuck::{Pod, Zeroable};
use embedded_hal_async::spi::{Operation, SpiDevice};

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

    async fn read<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut rx = [0u8; N];
        self.spi.read(&mut rx).await.map_err(|_| Error::SpiError)?;
        Ok(rx)
    }

    pub async fn get_blocks(&mut self, sigmap: u8, max_blocks: u8) -> Result<Block> {
        // let tx: [u8; 6] = [
        //     Request::NO_CHECKSUM_SYNC_L,
        //     Request::NO_CHECKSUM_SYNC_H,
        //     2,
        //     Request::BLOCKS,
        //     sigmap, // first byte of no_checksum_sync (little endian -> least-significant byte first)
        //     max_blocks,
        // ];
        let header = RequestHeader {
            checksum_sync: Request::NO_CHECKSUM_SYNC,
            length: 2,
            packet_type: Request::BLOCKS,
        };

        self.spi.write(tx).await.map_err(|_| Error::SpiError)?;
        // let head = self.read_header().await?;
        // if head.t
        Err(Error::SpiError)
    }

    async fn transmit(&mut self, header: &RequestHeader, data: &[u8]) -> Result<ResponseHeader> {
        let tx = bytemuck::bytes_of(header);
        self.spi
            .transaction(&mut [Operation::Write(tx), Operation::Write(data)])
            .await
            .map_err(|_| Error::SpiError)?;
        self.sync().await?;
        self.read_header().await
    }

    async fn read_header(&mut self) -> Result<ResponseHeader> {
        let mut rx = [0u8; 4];
        self.spi.read(&mut rx).await.map_err(|_| Error::SpiError)?;
        Ok(bytemuck::cast(rx))
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
            "hw: {:x} fw:{}.{} build: {} type: {:a}",
            self.hw,
            self.fw_major,
            self.fw_minor,
            self.fw_build,
            self.fw_type_str,
        )
    }
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct RequestHeader {
    pub checksum_sync: u16,
    pub packet_type: u8,
    pub length: u8,
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ResponseHeader {
    pub packet_type: u8,
    pub length: u8,
    pub checksum: u16,
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Block {
    pub signature: u16,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub angle: i16,
    pub index: u8,
    pub age: u8,
}
//   uint16_t m_signature;
//   uint16_t m_x;
//   uint16_t m_y;
//   uint16_t m_width;
//   uint16_t m_height;
//   int16_t m_angle;
//   uint8_t m_index;
//   uint8_t m_age;
