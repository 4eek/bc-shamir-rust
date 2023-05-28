pub const MIN_SECRET_LEN: usize = 16;
pub const MAX_SECRET_LEN: usize = 32;
pub const MAX_SHARE_COUNT: usize = 16;

mod hazmat;
mod interpolate;

mod error;
pub use error::Error;

mod shamir;
pub use shamir::{split_secret, recover_secret};

#[cfg(test)]
mod tests {
    use super::*;
    use bc_crypto::RandomNumberGenerator;
    use hex_literal::hex;

    struct FakeRandomNumberGenerator;

    impl RandomNumberGenerator for FakeRandomNumberGenerator {
        fn next_u64(&mut self) -> u64 {
            unimplemented!()
        }

        fn random_data(&mut self, size: usize) -> Vec<u8> {
            let mut b = vec![0u8; size];
            self.fill_random_data(&mut b);
            b
        }

        fn fill_random_data(&mut self, data: &mut [u8]) {
            let mut b = 0u8;
            data.iter_mut().for_each(|x| {
                *x = b;
                b = b.wrapping_add(17);
            });
        }
    }

    #[test]
    fn test_split_secret_3_5() {
        let mut rng = FakeRandomNumberGenerator;
        let secret = hex!("0ff784df000c4380a5ed683f7e6e3dcf");
        //println!("secret: {}", hex::encode(secret));
        let shares = split_secret(3, 5, &secret, &mut rng).unwrap();
        assert_eq!(shares.len(), 5);
        //shares.iter().enumerate().for_each(|(index, share)| println!("{}: {}", index, hex::encode(share)));
        assert_eq!(shares[0], hex!("00112233445566778899aabbccddeeff"));
        assert_eq!(shares[1], hex!("d43099fe444807c46921a4f33a2a798b"));
        assert_eq!(shares[2], hex!("d9ad4e3bec2e1a7485698823abf05d36"));
        assert_eq!(shares[3], hex!("0d8cf5f6ec337bc764d1866b5d07ca42"));
        assert_eq!(shares[4], hex!("1aa7fe3199bc5092ef3816b074cabdf2"));

        let recovered_share_indexes = vec![1, 2, 4];
        let recovered_shares = recovered_share_indexes.iter().map(|index| shares[*index].clone()).collect::<Vec<_>>();
        let recovered_secret = recover_secret(&recovered_share_indexes, &recovered_shares).unwrap();
        assert_eq!(recovered_secret, secret);
    }

    #[test]
    fn test_split_secret_2_7() {
        let mut rng = FakeRandomNumberGenerator;
        let secret = hex!("204188bfa6b440a1bdfd6753ff55a8241e07af5c5be943db917e3efabc184b1a");
        //println!("secret: {}", hex::encode(secret));
        let shares = split_secret(2, 7, &secret, &mut rng).unwrap();
        assert_eq!(shares.len(), 7);
        //shares.iter().enumerate().for_each(|(index, share)| println!("{}: {}", index, hex::encode(share)));
        assert_eq!(shares[0], hex!("2dcd14c2252dc8489af3985030e74d5a48e8eff1478ab86e65b43869bf39d556"));
        assert_eq!(shares[1], hex!("a1dfdd798388aada635b9974472b4fc59a32ae520c42c9f6a0af70149b882487"));
        assert_eq!(shares[2], hex!("2ee99daf727c0c7773b89a18de64497ff7476dacd1015a45f482a893f7402cef"));
        assert_eq!(shares[3], hex!("a2fb5414d4d96ee58a109b3ca9a84be0259d2c0f9ac92bdd3199e0eed3f1dd3e"));
        assert_eq!(shares[4], hex!("2b851d188b8f5b3653659cc0f7fa45102dadf04b708767385cd803862fcb3c3f"));
        assert_eq!(shares[5], hex!("a797d4a32d2a39a4aacd9de48036478fff77b1e83b4f16a099c34bfb0b7acdee"));
        assert_eq!(shares[6], hex!("28a19475dcde9f09ba2e9e881979413592027216e60c8513cdee937c67b2c586"));

        let recovered_share_indexes = vec![3, 4];
        let recovered_shares = recovered_share_indexes.iter().map(|index| shares[*index].clone()).collect::<Vec<_>>();
        let recovered_secret = recover_secret(&recovered_share_indexes, &recovered_shares).unwrap();
        assert_eq!(recovered_secret, secret);
    }
}
