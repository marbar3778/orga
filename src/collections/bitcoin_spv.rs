use super::Deque;
use crate::state::State;
use crate::Result;
use bitcoin::consensus::{Decodable, Encodable};
use bitcoin::BlockHeader;
use ed::{Decode, Encode};
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};

struct BitcoinAdapter<T> {
    inner: T,
}

impl<T> Deref for BitcoinAdapter<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for BitcoinAdapter<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Encodable> Encode for BitcoinAdapter<T> {
    fn encode(&self) -> ed::Result<Vec<u8>> {
        let mut dest: Vec<u8> = Vec::new();
        self.encode_into(&mut dest)?;
        Ok(dest)
    }

    fn encode_into<W: Write>(&self, mut dest: &mut W) -> ed::Result<()> {
        let mut header_bytes: Vec<u8> = Vec::new();
        match self.inner.consensus_encode(dest) {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    fn encoding_length(&self) -> ed::Result<usize> {
        let mut _dest: Vec<u8> = Vec::new();
        match self.inner.consensus_encode(_dest) {
            Ok(inner) => Ok(inner),
            Err(e) => Err(e.into()),
        }
    }
}

impl<T: Decodable> Decode for BitcoinAdapter<T> {
    fn decode<R: Read>(input: R) -> ed::Result<Self> {
        let decoded_bytes = Decodable::consensus_decode(input);
        match decoded_bytes {
            Ok(inner) => Ok(Self { inner }),
            Err(e) => {
                let std_e =
                    std::io::Error::new(std::io::ErrorKind::Other, "Failed to decode header");
                Err(std_e.into())
            }
        }
    }
}

#[derive(Encode, Decode)]
pub struct SPVBlockHeader {
    height: u32,
    inner: BitcoinAdapter<BlockHeader>,
}

#[derive(State)]
pub struct BitcoinSPV {
    deque: Deque<SPVBlockHeader>,
}

impl BitcoinSPV {
    fn add(&mut self, header: SPVBlockHeader) -> Result<()> {
        Ok(())
    }

    fn add_all<const N: usize>(&mut self, headers: [SPVBlockHeader; N]) -> Result<()> {
        Ok(())
    }

    fn add_iter<T: IntoIterator<Item = SPVBlockHeader>>(&mut self, iter: T) -> Result<()> {
        Ok(())
    }

    fn get_by_height(&self, height: u32) -> Option<SPVBlockHeader> {
        None
    }

    fn get_by_hash(&self, hash: [u8; 32]) -> Option<SPVBlockHeader> {
        None
    }

    fn height(&self) -> u32 {
        42
    }

    fn verify_headers<const N: usize>(&self, headers: [SPVBlockHeader; N]) -> Result<()> {
        Ok(())
    }

    fn verify_header_collection<T: IntoIterator<Item = SPVBlockHeader>>(
        &self,
        iter: T,
    ) -> Result<()> {
        Ok(())
    }
}
