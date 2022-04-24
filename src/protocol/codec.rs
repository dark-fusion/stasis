use bytes::{BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

#[derive(Debug)]
pub struct Codec {
    pub state: State,
}

#[derive(Debug)]
pub enum State {
    Header,
    Data(usize),
}

impl Codec {
    fn parse_header(&self, _src: &mut BytesMut) -> std::io::Result<Option<usize>> {
        todo!()
    }

    fn decode_data(&self, _src: &mut BytesMut, _len: usize) -> Option<BytesMut> {
        todo!()
    }
}

impl Decoder for Codec {
    type Item = BytesMut;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let n = match self.state {
            State::Header => match self.parse_header(src)? {
                Some(n) => {
                    self.state = State::Data(n);
                    n
                }
                None => return Ok(None),
            },
            State::Data(n) => n,
        };

        match self.decode_data(src, n) {
            Some(data) => {
                // Update the decoder state
                self.state = State::Header;

                // Ensure buffer has enough space to read next header
                // src.reserve(HEADER_LEN);

                Ok(Some(data))
            }
            None => Ok(None),
        }
    }

    fn decode_eof(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.decode(buf)? {
            Some(frame) => Ok(Some(frame)),
            None => {
                if buf.is_empty() {
                    Ok(None)
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "bytes remaining on stream",
                    ))
                }
            }
        }
    }
}

impl Encoder<BytesMut> for Codec {
    type Error = std::io::Error;

    fn encode(&mut self, data: BytesMut, buffer: &mut BytesMut) -> Result<(), Self::Error> {
        buffer.reserve(data.len());
        buffer.put(data);
        Ok(())
    }
}
