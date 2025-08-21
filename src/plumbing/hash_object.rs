use sha1::{Sha1, Digest};

pub fn hash_object(data: String) -> String {
    let header = format!("blob {}\0", data.len());

    let mut hasher = Sha1::new();

    let blob = header + &data;

    hasher.update(blob.as_bytes());

    let hash = format!("{:x}", hasher.finalize());

    hash
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_a_valid_hash() {
        let content = String::from("Hello, world!\n");

        let hash = hash_object(content);

        assert_eq!(hash, String::from("af5626b4a114abcb82d63db7c8082c3c4756e51b"));
    }
}
