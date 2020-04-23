//! Simplified Chinese


use super::data::gb2312::{
    HI_LIMIT,
    LO_SIZE,
    DECODE_MAP,
    HI_MAP,
    ENCODE_MAP,
};


pub fn encode(src: &str, dst: &mut Vec<u8>) {
    for c in src.chars() {
        let c = u32::from(c) as u16;
        if c <= 0x7F {
            dst.push(c as u8);
        } else if c >= 0xA0 {
            let hi = usize::from(c >> 8);
            let lo = usize::from(c & 0xFF);

            let pos = HI_MAP[hi] * 0xFF + lo;
            let code = ENCODE_MAP[pos];
            if code != 0x0000 {
                dst.push((code >> 8) as u8);
                dst.push((code & 0xFF) as u8);
            } else {
                dst.push(b'?');
            }
        }
    }
}


pub fn decode(src: &[u8], dst: &mut String) {
    let mut skip = 0;
    let size = src.len();

    while skip < size {
        let c = src[skip];
        if c <= 0x7F {
            dst.push(c as char);
            skip += 1;
        } else {
            skip += 1;
            if skip >= size {
                dst.push('?');
                break;
            }

            let hi = usize::from(c) & 0x7F;
            let lo = usize::from(src[skip]) & 0x7F;
            skip += 1;

            if lo < 0x21 || hi < 0x21 || hi > HI_LIMIT {
                dst.push('?');
                continue;
            }

            let shift = (hi - 0x21) * LO_SIZE + (lo - 0x21);
            let c = u32::from(DECODE_MAP[shift]);

            if c == 0 {
                dst.push('?');
                continue;
            }

            dst.push(unsafe { std::char::from_u32_unchecked(c) })
        }
    }
}


pub fn encode_to_vec(src: &str) -> Vec<u8> {
    let mut ret = Vec::new();
    encode(src, &mut ret);
    ret
}


pub fn decode_to_string(src: &[u8]) -> String {
    let mut ret = String::new();
    decode(src, &mut ret);
    ret
}


pub fn bound(src: &[u8], limit: usize) -> usize {
    if limit == 0 {
        return limit;
    }

    if src.len() <= limit {
        return src.len();
    }

    let mut cnt_limit = limit;

    while cnt_limit > 0 {
        let next = cnt_limit - 1;
        if src[next] < 0xA1 {
            break;
        }
        cnt_limit = next;
    }

    ((limit - cnt_limit) & !1) + cnt_limit
}
