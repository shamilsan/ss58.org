use base58::{FromBase58, ToBase58};
use blake2::{Blake2b512, Digest};

const SS58_MIN_LEN: usize = 35;
const SS58_MAX_LEN: usize = 36;

pub fn address_to_key(address: &str) -> Result<(u16, String), String> {
    let address = address
        .from_base58()
        .map_err(|e| format!("Base58 conversion error: {e:?}"))?;
    let len = address.len();
    if !(SS58_MIN_LEN..=SS58_MAX_LEN).contains(&len) {
        Err("SS58 address has wrong length".to_string())
    } else {
        // Verify the checksum
        let checksum = &address[len - 2..len];
        let mut hasher = Blake2b512::new();
        hasher.update(b"SS58PRE");
        hasher.update(&address[0..len - 2]);
        let checksum_calc = &hasher.finalize()[0..2];
        if checksum != checksum_calc {
            return Err(format!(
                "Invalid checksum: input {checksum:?} is not equal to calculated {checksum_calc:?}"
            ));
        }

        // Get the key
        let key = format!("0x{}", hex::encode(&address[len - 34..len - 2]));

        // Get the network prefix
        let prefix_buf = &address[0..len - 34];
        let prefix = if prefix_buf.len() == 1 {
            prefix_buf[0] as u16
        } else {
            let prefix_hi = ((prefix_buf[1] & 0x3F) as u16) << 8;
            let prefix_lo = ((prefix_buf[0] << 2) | (prefix_buf[1] >> 6)) as u16;
            prefix_hi | prefix_lo
        };

        Ok((prefix, key))
    }
}

pub fn key_to_address(prefix: u16, key: &str) -> Result<String, String> {
    let formatted_key = if key.starts_with("0x") {
        &key[2..key.len()]
    } else {
        key
    };

    let raw_key = hex::decode(formatted_key);
    match raw_key {
        Err(e) => Err(format!("Hex decoding error: {e:?}")),
        Ok(mut raw_key) => {
            if raw_key.len() != 32 {
                Err(format!(
                    "Public key has wrong length: {} != 32",
                    raw_key.len()
                ))
            } else {
                let mut hasher = Blake2b512::new();
                hasher.update(b"SS58PRE");
                let simple_prefix: u8 = (prefix & 0x3F) as _;
                let full_prefix = 0x4000 | ((prefix >> 8) & 0x3F) | ((prefix & 0xFF) << 6);
                let prefix_hi: u8 = (full_prefix >> 8) as _;
                let prefix_low: u8 = (full_prefix & 0xFF) as _;
                if prefix == simple_prefix as u16 {
                    hasher.update([simple_prefix]);
                } else {
                    hasher.update([prefix_hi]);
                    hasher.update([prefix_low]);
                }
                hasher.update(&raw_key);
                let checksum = hasher.finalize();

                let mut raw_address: Vec<u8> = Vec::with_capacity(64);
                if prefix == simple_prefix as u16 {
                    raw_address.push(simple_prefix);
                } else {
                    raw_address.push(prefix_hi);
                    raw_address.push(prefix_low);
                }
                raw_address.append(&mut raw_key);
                raw_address.extend_from_slice(&checksum[0..2]);

                Ok(raw_address[..].to_base58())
            }
        }
    }
}

pub fn copy(text: &str) {
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.default_view())
        .and_then(|win| win.navigator().clipboard())
        .map(|clipboard| clipboard.write_text(text));
}
