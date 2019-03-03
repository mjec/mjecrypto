use super::test_strings;
use mjecrypto::hash::sha256::hash;
use test::Bencher;

#[bench]
fn becnhmark_0_empty(b: &mut Bencher) {
    b.iter(|| hash(&test_strings::EMPTY));
}

#[bench]
fn becnhmark_1_eight_bytes(b: &mut Bencher) {
    b.iter(|| hash(&test_strings::EIGHT_BYTES));
}

#[bench]
fn becnhmark_2_one_kb(b: &mut Bencher) {
    b.iter(|| hash(&test_strings::ONE_KB));
}

#[bench]
fn becnhmark_3_one_mb(b: &mut Bencher) {
    b.iter(|| hash(&test_strings::ONE_MB));
}

#[cfg(feature = "benchmark-comparison")]
mod comparison {
    use super::super::test_strings;
    use crypto::digest::Digest;
    use crypto::sha2::Sha256;
    use test::Bencher;

    #[bench]
    fn becnhmark_0_empty(b: &mut Bencher) {
        b.iter(|| {
            let mut hasher = Sha256::new();
            hasher.input(&test_strings::EMPTY);
        });
    }

    #[bench]
    fn becnhmark_1_eight_bytes(b: &mut Bencher) {
        b.iter(|| {
            let mut hasher = Sha256::new();
            hasher.input(&test_strings::EIGHT_BYTES);
        });
    }

    #[bench]
    fn becnhmark_2_one_kb(b: &mut Bencher) {
        b.iter(|| {
            let mut hasher = Sha256::new();
            hasher.input(&test_strings::ONE_KB);
        });
    }

    #[bench]
    fn becnhmark_3_one_mb(b: &mut Bencher) {
        b.iter(|| {
            let mut hasher = Sha256::new();
            hasher.input(&test_strings::ONE_MB);
        });
    }
}
