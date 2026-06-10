use embedded_hal_async::spi::{Operation, SpiDevice};
use zerocopy::{FromBytes as _, IntoBytes as _};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{
    Result,
    error::Error,
    packets::{self, Request, Response},
};

const BUF_SIZE: usize = 0x104;
pub struct Pixy2<SPI> {
    spi: SPI,
    buf: [u8; BUF_SIZE],
}

impl<SPI> Pixy2<SPI>
where
    SPI: SpiDevice,
{
    pub fn new(spi: SPI) -> Self {
        Self {
            spi,
            buf: [0u8; BUF_SIZE],
        }
    }

    pub async fn init(&mut self) -> Result<()> {
        let _ = self.get_version().await?;
        Ok(())
    }

    /// Skips all garbage data before the needed data
    async fn sync(&mut self) -> Result<()> {
        let mut cur = self.read_byte().await?;
        let mut prev = cur;
        for _ in 0..127 {
            // #[cfg(feature = "defmt")]
            // defmt::dbg!(cur);
            cur = self.read_byte().await?;
            if (cur == packets::CHECKSUM_SYNC_H) && (prev == packets::CHECKSUM_SYNC_L) {
                return Ok(());
            }
            prev = cur;
        }
        Err(Error::Timeout)
    }

    pub async fn get_version(&mut self) -> Result<Version> {
        let res = self.transmit_header(&RequestHeader::version()).await?;
        if res.packet_type == Response::VERSION {
            let v = self.read::<16>().await?;
            Ok(Version::read_from_bytes(&v).unwrap())
        } else {
            Err(Error::Busy)
        }
    }

    async fn read<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut rx = [0u8; N];
        self.spi.read(&mut rx).await.map_err(|_| Error::SpiError)?;
        Ok(rx)
    }

    // async fn read_into<const N: usize>(&mut self, rx: &mut [u8; N]) -> Result<()> {
    //     self.spi.read(rx).await.map_err(|_| Error::SpiError)
    // }

    pub async fn get_blocks(&mut self, sigmap: u8, max_blocks: u8) -> Result<&Block> {
        let res_header = self
            .transmit(&RequestHeader::blocks(), &[sigmap, max_blocks])
            .await?;
        if res_header.packet_type == Response::BLOCKS {
            let len = res_header.length as usize;
            if len > BUF_SIZE {
                return Err(Error::BufferWontFit);
            }
            self.spi
                .read(&mut self.buf[0..len])
                .await
                .map_err(|_| Error::SpiError)?;
            // Ok(<[Block]>::ref_from_bytes(&self.buf[0..len]).unwrap())
            let (blocks, _remainder) = Block::ref_from_prefix(&self.buf[0..len]).unwrap();
            Ok(blocks)
        } else {
            Err(Error::Busy)
        }
    }

    async fn transmit_header(&mut self, header: &RequestHeader) -> Result<ResponseHeader> {
        self.spi
            .write(header.as_bytes())
            .await
            .map_err(|_| Error::SpiError)?;
        self.sync().await?;
        self.read_header().await
    }

    async fn transmit(&mut self, header: &RequestHeader, data: &[u8]) -> Result<ResponseHeader> {
        self.spi
            .transaction(&mut [Operation::Write(header.as_bytes()), Operation::Write(data)])
            .await
            .map_err(|_| Error::SpiError)?;
        self.sync().await?;
        self.read_header().await
    }

    async fn read_header(&mut self) -> Result<ResponseHeader> {
        let mut rx = [0u8; 4];
        self.spi.read(&mut rx).await.map_err(|_| Error::SpiError)?;
        Ok(ResponseHeader::read_from_bytes(&rx).unwrap())
    }

    async fn read_byte(&mut self) -> Result<u8> {
        let mut rx = [0u8];
        self.spi.read(&mut rx).await.map_err(|_| Error::SpiError)?;
        Ok(rx[0])
    }
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable)]
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

#[derive(Clone, Copy, FromBytes, IntoBytes, KnownLayout, Immutable)]
#[repr(C)]
pub struct RequestHeader {
    pub checksum_sync: u16,
    pub packet_type: u8,
    pub length: u8,
}

impl RequestHeader {
    pub fn new(checksum_sync: u16, packet_type: u8, length: u8) -> Self {
        Self {
            checksum_sync,
            packet_type,
            length,
        }
    }

    pub fn version() -> Self {
        Self::new(packets::NO_CHECKSUM_SYNC, Request::VERSION, 0)
    }

    pub fn blocks() -> Self {
        Self {
            checksum_sync: packets::NO_CHECKSUM_SYNC,
            length: 2,
            packet_type: Request::BLOCKS,
        }
    }
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable)]
#[repr(C)]
pub struct ResponseHeader {
    pub packet_type: u8,
    pub length: u8,
    pub checksum: u16,
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Clone, Copy)]
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
