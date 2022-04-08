pub struct Utf16Char {
    pub repr: [u16; 2],
    pub len: usize,
}

impl Utf16Char {
    const fn encode(ch: u32) -> Self {
        if ch <= 0xffff {
            Self {
                repr: [ch as u16, 0],
                len: 1,
            }
        } else {
            let payload = ch - 0x10000;
            let hi = (payload >> 10) | 0xd800;
            let lo = (payload & 0x3ff) | 0xdc00;
            Self {
                repr: [hi as u16, lo as u16],
                len: 2,
            }
        }
    }

    #[cfg(test)]
    pub fn as_slice(&self) -> &[u16] {
        &self.repr[..self.len]
    }
}

pub struct EncodeUtf16Iter {
    str: &'static [u8],
    index: usize,
}

impl EncodeUtf16Iter {
    pub const fn new(str: &'static [u8]) -> Self {
        Self { str, index: 0 }
    }

    pub const fn next(self) -> Option<(Self, Utf16Char)> {
        if self.index >= self.str.len() {
            None
        } else {
            let (index, ch) = decode_utf8(self.str, self.index);
            Some((Self { index, ..self }, Utf16Char::encode(ch)))
        }
    }
}

// (&str bytes, index) -> (new index, decoded char)
const fn decode_utf8(s: &[u8], i: usize) -> (usize, u32) {
    let b0 = s[i];
    match b0 {
        // one-byte seq
        0b0000_0000..=0b0111_1111 => {
            let decoded = b0 as u32;
            (i + 1, decoded)
        }
        // two-byte seq
        0b1100_0000..=0b1101_1111 => {
            let decoded = ((b0 as u32 & 0x1f) << 6) | (s[i + 1] as u32 & 0x3f);
            (i + 2, decoded)
        }
        // 3 byte seq
        0b1110_0000..=0b1110_1111 => {
            let decoded = ((b0 as u32 & 0x0f) << 12)
                | ((s[i + 1] as u32 & 0x3f) << 6)
                | (s[i + 2] as u32 & 0x3f);
            (i + 3, decoded)
        }
        // 3 byte seq
        0b1111_0000..=0b1111_0111 => {
            let decoded = ((b0 as u32 & 0x07) << 18)
                | ((s[i + 1] as u32 & 0x3f) << 12)
                | ((s[i + 2] as u32 & 0x3f) << 6)
                | (s[i + 3] as u32 & 0x3f);
            (i + 4, decoded)
        }
        // continuation bytes, or never-valid bytes.
        0b1000_0000..=0b1011_1111 | 0b1111_1000..=0b1111_1111 => {
            #[allow(unconditional_panic)]
            {
                // replace this with unreachable!() when possible in const fn.
                const NOT_POSSIBLE_FOR_VALID_UTF8: [(); 0] = [];
                let _ = NOT_POSSIBLE_FOR_VALID_UTF8[0];
            }
            (s.len(), 0xfffd)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_utf8() {
        for c in '\u{0}'..=core::char::MAX {
            let mut buf;
            for off in 0..4 {
                // Ensure we see garbage if we read outside bounds.
                buf = [0xff; 8];
                let len = c.encode_utf8(&mut buf[off..(off + 4)]).len();
                let (end_idx, decoded) = super::decode_utf8(&buf, off);
                assert_eq!(
                    (end_idx, decoded),
                    (off + len, c as u32),
                    "failed for U+{code:04X} ({ch:?}) encoded as {buf:#x?} over {range:?}",
                    code = c as u32,
                    ch = c,
                    buf = &buf[off..(off + len)],
                    range = off..(off + len),
                );
            }
        }
    }

    #[test]
    fn encode_utf16() {
        for c in '\u{0}'..=core::char::MAX {
            assert_eq!(
                c.encode_utf16(&mut [0u16; 2]),
                Utf16Char::encode(c as u32).as_slice(),
                "failed for U+{:04X} ({:?})",
                c as u32,
                c
            );
        }
    }
}
