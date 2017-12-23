//! Parse ITM packets from bytes and streams.

use error::{Error, ErrorKind, Result};
use heapless::Vec as HVec;
use packet::{self, Packet, Instrumentation};
use std::io::Read;

pub struct Decoder<R: Read> {
    inner: R,
}

impl<R: Read> Decoder<R> {
    // TODO: Builder pattern.

    pub fn new(inner: R) -> Decoder<R> {
        Decoder::<R> {
            inner: inner,
        }
    }

    pub fn read_packet(&mut self) -> Result<Packet> {
        let mut header = [0; 1];
        self.inner.read_exact(&mut header)?;
        let header = header[0];
        match header & 0b111 {
            0b001|0b010|0b011 => {
                // Instrumentation packet.
                let mut ud = Instrumentation {
                    payload: HVec::new(),
                    port: header >> 3,
                };

                let payload_size =
                    match header & 0b11 {
                        0b01 => 1,
                        0b10 => 2,
                        0b11 => 4,
                        _ => unreachable!(), // Contradicts match on last 3 bits.
                    };
                ud.payload.resize_default(payload_size)
                    .expect("payload_size <= payload.capacity");
                self.inner.read_exact(&mut *ud.payload)?;

                Ok(Packet {
                    header: header,
                    kind: packet::Kind::Instrumentation(ud),
                })
            },
            _ => {
                return Err(Error::from(ErrorKind::UnknownHeader(header)));
            }
        }
    }
}

// TODO: Parse tests.
