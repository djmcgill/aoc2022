#![feature(iter_array_chunks)]
#![feature(unchecked_math)]
#![feature(slice_split_at_unchecked)]
use std::{
    collections::{hash_map::RandomState, BTreeSet, HashSet},
    hint::unreachable_unchecked,
};

pub fn p1_btreeset(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let count = bytes.len();
            let set1 = BTreeSet::from_iter(&bytes[..count / 2]);
            let set2 = BTreeSet::from_iter(&bytes[count / 2..]);
            let first_intersection = set1.intersection(&set2).next().unwrap();
            priority(**first_intersection) as u32
        })
        .sum()
}

pub fn p1_hashset_defaulthash(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let count = bytes.len();
            let set1 = HashSet::<_, RandomState>::from_iter(&bytes[..count / 2]);
            let set2 = HashSet::<_, RandomState>::from_iter(&bytes[count / 2..]);
            let first_intersection = set1.intersection(&set2).next().unwrap();
            priority(**first_intersection) as u32
        })
        .sum()
}

pub fn p1_hashset_identityhash(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let count = bytes.len();
            let set1 = HashSet::<_, IdentityByteHasherBuilder>::from_iter(&bytes[..count / 2]);
            let set2 = HashSet::<_, IdentityByteHasherBuilder>::from_iter(&bytes[count / 2..]);
            let first_intersection = set1.intersection(&set2).next().unwrap();
            priority(**first_intersection) as u32
        })
        .sum()
}

pub fn p1_hashset_identityhash_manual(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let count = bytes.len();
            let mut set1 = HashSet::with_capacity_and_hasher(58, IdentityByteHasherBuilder);
            for byte in &bytes[..count / 2] {
                set1.insert(*byte - b'A');
            }
            for byte in &bytes[count / 2..] {
                if set1.contains(&(*byte - b'A')) {
                    return priority(*byte) as u32;
                }
            }
            unreachable!()
        })
        .sum()
}

pub fn p1_array_filter_byteset(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let count = bytes.len();

            // [A..Z] \[ \\ \] ^ _ ` [a..z]
            let mut values = [false; 58];
            for x in &bytes[..count / 2] {
                values[(*x - b'A') as usize] = true;
            }
            for x in &bytes[count / 2..] {
                if values[(*x - b'A') as usize] {
                    return priority(*x) as u32;
                }
            }
            unreachable!()
        })
        .sum()
}

pub fn p1_array_filter_byteset_unsafe(input: &str) -> u32 {
    unsafe {
        input
            .lines()
            .map(|line| {
                let bytes = line.as_bytes();
                let count = bytes.len();

                // [A..Z] \[ \\ \] ^ _ ` [a..z]
                let mut values = [false; 58];
                let halves = bytes.split_at_unchecked(count.unchecked_shr(1));
                for x in halves.0 {
                    *values.get_unchecked_mut(x.unchecked_sub(b'A') as usize) = true;
                }
                for x in halves.1 {
                    if *values.get_unchecked(x.unchecked_sub(b'A') as usize) {
                        return priority_unsafe(*x) as u32;
                    }
                }
                unreachable_unchecked()
            })
            .sum()
    }
}

pub fn p2_btreeset(input: &str) -> u32 {
    input
        .lines()
        .array_chunks::<3>()
        .map(|chunk| {
            let set1 = BTreeSet::from_iter(chunk[0].as_bytes());
            let set2 = BTreeSet::from_iter(chunk[1].as_bytes());
            let set3 = BTreeSet::from_iter(chunk[2].as_bytes());
            let set_intersection = BTreeSet::from_iter(set1.intersection(&set2).cloned());
            let first_triple_intersection = set3.intersection(&set_intersection).next().unwrap();
            priority(**first_triple_intersection) as u32
        })
        .sum()
}

pub fn p2_hashset_defaulthash(input: &str) -> u32 {
    input
        .lines()
        .array_chunks::<3>()
        .map(|chunk| {
            let set1 = HashSet::<_, RandomState>::from_iter(chunk[0].as_bytes());
            let set2 = HashSet::<_, RandomState>::from_iter(chunk[1].as_bytes());
            let set3 = HashSet::<_, RandomState>::from_iter(chunk[2].as_bytes());
            let set_intersection =
                HashSet::<_, RandomState>::from_iter(set1.intersection(&set2).cloned());
            let first_triple_intersection = set3.intersection(&set_intersection).next().unwrap();
            priority(**first_triple_intersection) as u32
        })
        .sum()
}

pub fn p2_hashset_identityhash(input: &str) -> u32 {
    input
        .lines()
        .array_chunks::<3>()
        .map(|chunk| {
            let set1 = HashSet::<_, IdentityByteHasherBuilder>::from_iter(chunk[0].as_bytes());
            let set2 = HashSet::<_, IdentityByteHasherBuilder>::from_iter(chunk[1].as_bytes());
            let set3 = HashSet::<_, IdentityByteHasherBuilder>::from_iter(chunk[2].as_bytes());
            let set_intersection = HashSet::<_, IdentityByteHasherBuilder>::from_iter(
                set1.intersection(&set2).cloned(),
            );
            let first_triple_intersection = set3.intersection(&set_intersection).next().unwrap();
            priority(**first_triple_intersection) as u32
        })
        .sum()
}

pub fn p2_hashset_identityhash_manual(input: &str) -> u32 {
    input
        .lines()
        .array_chunks::<3>()
        .map(|chunk| {
            let mut set1 = HashSet::with_capacity_and_hasher(58, IdentityByteHasherBuilder);
            for x in chunk[0].as_bytes() {
                set1.insert(*x - b'A');
            }
            let mut set2 = HashSet::with_capacity_and_hasher(58, IdentityByteHasherBuilder);
            for x in chunk[1].as_bytes() {
                let x = *x - b'A';
                if set1.contains(&x) {
                    set2.insert(x);
                }
            }
            for x in chunk[2].as_bytes() {
                if set2.contains(&(*x - b'A')) {
                    return priority(*x) as u32;
                }
            }

            unreachable!()
        })
        .sum()
}

pub fn p2_array_filter_byteset(input: &str) -> u32 {
    input
        .lines()
        .array_chunks::<3>()
        .map(|chunk| {
            let mut values = [0u8; 58];
            for x in chunk[0].as_bytes() {
                let value = &mut values[(*x - b'A') as usize];
                if *value == 0 {
                    *value = 1;
                }
            }
            for x in chunk[1].as_bytes() {
                let value = &mut values[(*x - b'A') as usize];
                if *value == 1 {
                    *value = 2;
                }
            }

            for x in chunk[2].as_bytes() {
                if values[(*x - b'A') as usize] == 2 {
                    return priority(*x) as u32;
                }
            }
            unreachable!()
        })
        .sum()
}

pub fn p2_array_filter_byteset_2(input: &str) -> u32 {
    input
        .lines()
        .array_chunks::<3>()
        .map(|chunk| {
            let mut values = [0u8; 58];
            for x in chunk[0].as_bytes() {
                values[(*x - b'A') as usize] |= 1;
            }
            for x in chunk[1].as_bytes() {
                values[(*x - b'A') as usize] |= 2;
            }
            for x in chunk[2].as_bytes() {
                if values[(*x - b'A') as usize] == 3 {
                    return priority(*x) as u32;
                }
            }
            unreachable!()
        })
        .sum()
}

pub fn p2_array_filter_byteset_unsafe(input: &str) -> u32 {
    unsafe {
        input
            .lines()
            .array_chunks::<3>()
            .map(|chunk| {
                let mut values = [0u8; 58];
                for x in chunk.get_unchecked(0).as_bytes() {
                    *values.get_unchecked_mut((x.unchecked_sub(b'A')) as usize) |= 1;
                }
                for x in chunk.get_unchecked(1).as_bytes() {
                    *values.get_unchecked_mut((x.unchecked_sub(b'A')) as usize) |= 2;
                }
                for x in chunk.get_unchecked(2).as_bytes() {
                    if *values.get_unchecked((x.unchecked_sub(b'A')) as usize) == 3 {
                        return priority_unsafe(*x) as u32;
                    }
                }
                unreachable_unchecked()
            })
            .sum()
    }
}

#[derive(Default)]
struct IdentityByteHasherBuilder;
impl std::hash::BuildHasher for IdentityByteHasherBuilder {
    type Hasher = IdentityByteHasher;
    fn build_hasher(&self) -> Self::Hasher {
        IdentityByteHasher(0)
    }
}

struct IdentityByteHasher(u8);
impl std::hash::Hasher for IdentityByteHasher {
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    fn write(&mut self, _bytes: &[u8]) {
        panic!()
    }

    fn write_u8(&mut self, i: u8) {
        self.0 = i;
    }
}

fn priority_old(c: u8) -> u8 {
    match c {
        b'a'..=b'z' => c - b'a' + 1,
        b'A'..=b'Z' => c - b'A' + 27,
        _ => unreachable!(),
    }
}
// maps: [A..Z] \[ \\ \] ^   _  ` [a..z]
// to: [27..52] 53 54 55 56 57 58 [1..26]
fn priority(c: u8) -> u8 {
    // maps: [A..Z] \[ \\ \] ^   _  ` [a..z]
    // to:  [0..25] 26 27 28 29 30 31 [32..58]
    let x = c - b'A';

    // maps: [A..Z] \[ \\ \] ^   _  ` [a..z]
    // to:  [26..51] 52 53 54 55 56 57 [58..84]
    let x = x + 26;

    // maps: [A..Z] \[ \\ \] ^   _  ` [a..z]
    // to:  [26..51] 52 53 54 55 56 57 [0..25]
    let x = x % 58;

    // maps: [A..Z] \[ \\ \] ^   _  ` [a..z]
    // to:  [27..52] 53 54 55 56 57 58 [1..26]
    x + 1
}

// maps: [A..Z] \[ \\ \] ^   _  ` [a..z]
// to: [27..52] 53 54 55 56 57 58 [1..26]
#[inline(always)]
unsafe fn priority_unsafe(c: u8) -> u8 {
    // maps: [A..Z] \[ \\ \] ^   _  ` [a..z]
    // to:  [0..25] 26 27 28 29 30 31 [32..58]
    // let x = c.unchecked_sub(b'A');

    // maps: [A..Z] \[ \\ \] ^   _  ` [a..z]
    // to:  [26..51] 52 53 54 55 56 57 [58..84]
    let x = c.unchecked_sub(b'A' - 26);

    // maps: [A..Z] \[ \\ \] ^   _  ` [a..z]
    // to:  [26..51] 52 53 54 55 56 57 [0..25]
    let x = x % 58;

    // maps: [A..Z] \[ \\ \] ^   _  ` [a..z]
    // to:  [27..52] 53 54 55 56 57 58 [1..26]
    x.unchecked_add(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let ans = 157;
        assert_eq!(ans, p1_btreeset(TEST));
        assert_eq!(ans, p1_hashset_defaulthash(TEST));
        assert_eq!(ans, p1_hashset_identityhash(TEST));
        assert_eq!(ans, p1_hashset_identityhash_manual(TEST));
        assert_eq!(ans, p1_array_filter_byteset(TEST));
        assert_eq!(ans, p1_array_filter_byteset_unsafe(TEST));
    }

    #[test]
    fn p2_test() {
        let ans = 70;
        assert_eq!(ans, p2_btreeset(TEST));
        assert_eq!(ans, p2_hashset_defaulthash(TEST));
        assert_eq!(ans, p2_hashset_identityhash(TEST));
        assert_eq!(ans, p2_hashset_identityhash_manual(TEST));
        assert_eq!(ans, p2_array_filter_byteset(TEST));
        assert_eq!(ans, p2_array_filter_byteset_2(TEST));
        assert_eq!(ans, p2_array_filter_byteset_unsafe(TEST));
    }

    #[test]
    fn p1_real() {
        let ans = 7428;
        assert_eq!(ans, p1_btreeset(REAL));
        assert_eq!(ans, p1_hashset_defaulthash(REAL));
        assert_eq!(ans, p1_hashset_identityhash(REAL));
        assert_eq!(ans, p1_hashset_identityhash_manual(REAL));
        assert_eq!(ans, p1_array_filter_byteset(REAL));
        assert_eq!(ans, p1_array_filter_byteset_unsafe(REAL));
    }

    #[test]
    fn p2_real() {
        let ans = 2650;
        assert_eq!(ans, p2_btreeset(REAL));
        assert_eq!(ans, p2_hashset_defaulthash(REAL));
        assert_eq!(ans, p2_hashset_identityhash(REAL));
        assert_eq!(ans, p2_hashset_identityhash_manual(REAL));
        assert_eq!(ans, p2_array_filter_byteset(REAL));
        assert_eq!(ans, p2_array_filter_byteset_2(REAL));
        assert_eq!(ans, p2_array_filter_byteset_unsafe(REAL));
    }

    #[test]
    fn priority_test() {
        for x in b'A'..=b'Z' {
            assert_eq!(priority_old(x), priority(x));
        }
        for x in b'a'..=b'z' {
            assert_eq!(priority_old(x), priority(x));
        }
    }
}

pub const TEST: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

pub const REAL: &str = r#"BccTFfTPTsffdDDqsq
lGGLQwFhDgWdqvhW
wbLNjGjlwLFrpSbllrHnHHRmmJVBmZJRRVcBTc
vvGLllBBLtllJnJFMZNjFcNG
bdhrhTgmhRrpLJMMNJgNffnf
qhRmTpzpzVzmTTbmVhWWWpVvDtsLVlwBVHvSsDCvvBDl
sLlhhthVRndBZzwBdV
PfmsPsqsGFqrSQpqBDDwpddDDM
mFjvSFrjPSvLJWvbRssN
gWnWQtMMDQbQvMDjjcwsvqcwsSSqZq
HzPJTtmtZJJsqfBJ
hCFPrNNPVNzFtQhdQMWRhhDd
sNhmsQZdNdsztNpHGCdjcBcwCvGvCv
brSlRfFFppHwfTpf
MbRlRpqMnqRpMhVtsQQQVWsM
HsdttdfHrHrwdhftMHMSDnJFwlQSgQZQZggZSW
LjCTqBCmmmTqgFQQFQjlnnDJ
GCTLBCmTBDBDPTTzqLCBpVcctcHbtbrcMMsdsdtcPtdb
QnJLdNLfLRQjpLlPflfQnlnswRvwsHVWVHWVVDZwWHcrwV
zFbgtqCBhtgBhsDZVDNHwvDv
zgCGCFCFGtMpJjlGTnlnLN
ZFhJZbvZVmFpFnJbnZFbLnbHPrjjQdBdCrMPQMQHrrrrVC
gTzlqmNlTflmtTRHdDPqMjHDMrjdQd
wcRSwglcNSwgWmNzwNlJZLpphbbnFhvLWhnphb
lRRrcbRfQmwwBLSLlvjjSvvFtS
WbghzhHWbsdHhVtGLVSSvLFJtSGC
WWbWzPHDqPfBqRZrmqnZ
frgLHMcHLrSZHPHcHrPNmMJtTCTCbJfhdwVbfJwwTbdd
DpDWRjQvqjDGFWhdVwQCnJgTnTbT
vvqggRsRpcssPrcsrN
MccbcZjmbbNSbjllbRqrhstVTMtqRtssTW
qJdwCLdCzQQzwHLQLTtsTtsTRVTVWWTJst
GdnpFHzwQCdpvpbccPbSbq
CbQhZpTbTgMMgptzrdGtGzlVNlrh
FWvmmRHSmjqnSjqnSjPqjnmNLGVGJlGWLlLrLWlVrGzNGW
FwvvBjmBnqmVvFnvnVPMDspDQTfZMCMTgDQTCB
VmrsHllHVLTdZtRclL
bjwMNQGCjGjjPwpRQnRLTtRQFvcdvt
MbWgjMDpNLMjNpwGpjbPWgfsHsqsHSrrHHJsgBSBzB
HFlwVhfLBZZLTLFHwqWCqWwQpWwSpqWC
PttsNjdtPtcmcDTnDdtjjDMpCCCCmSmqqGqmvMvbCqpC
jndRrTtPdgNPPnLrVhVLHhVJHffV
NJZVqHNNNJNqCdqZZVjtzCDMtzrtrFhhMFsPCS
TbwgwWRdQgfggQgvMPDSPszhSbSDszFs
QpRmWglwTQgGWTvGRvQRdwnjNNqqqBqVlcqVVNncVVHV
gqBFHLFDNCBClHgbLFbllLggVSTWJVWWpjvJpSSWJjBjZvjW
mczdshQcsGftdmcwcfhdzQVrhJZJTZhTVTDjpWpZvvWT
twGdRtcGnRgDqFRC
fsPjCjgRpwjPpsGgQwPfSZcdSZdWVHzSzcdzHRWz
MBLTTBMvbMmvJTLvTDvczNdZNHdHcJNzNqSZVG
rTlDlbBbrsjCCGnG
FsmBPtzHdmmvcvdWpcWCvM
RMJGjGGJJGLDRNJJnfGRRSvplTDccrTwSWClvCrwww
nnRqNGJLVLRRZNNZzBHMHVFVmzBFbhmb
MmTSpBMBCCMsbbrVvwwSLb
RffNRFRThtqHhHHqZNrLVvwQtsGvssdvvsGb
DThHghZfgqhFWRNHhhRWqMCnMzzznlPnBlgzMpnPCz
GhlMNMdpMZHHhhRLLjqSjjqvvmSh
PcTJDBDcBnBbTFcDwnPTQSLqLrqvWjwWLqRSvsRmWm
PDcDJbQTFBCBcncgQPcDgnHgVmppHVMddmZglzVHdMmt
BqFJqJGpBVnJqnjjmwvrNwVPHPrd
ZMbQmhWstZScWjdswwwHNdzHvj
ZZtQCtgZQWSbMMhmMSWQfGfGDBpRJnJqTFFJBgTR
tVcPGGqwgJPqtJtqZZwcZffsfrcNWBnSWWFrfhWBnz
dRMCMQLvNssBWFFR
pvjjQvmDMpHLdPPtgPPTssTHsq
WDBlFBbGdmBrqWjhWcqZCq
RPSRPtncngwgwzhCwghC
cssVsPpVncQMRsVQpRPnRcfJfTTmTdDBLbFlDblTDDDLJf
CZgCCzgzsCDZDzbbBclgvcjcnnjFBqgv
VLGThLWhWdThlWRHVTLTTjNjjdjrnBNBcrNcqmqqcc
VTRWLhpLWHRMPGRGRplVhSfttJpZzwstsbDtwbJzZt
fqhZRLhwZwVSLbCMCJdJCHQGZWdW
ztvpjtpcvgzppPvjvPlDQLnMlHDGWnQDLQWDdJ
PpsPzjPzgLrjzBgLggzrmrVSrTSfSqFVFmVhffFm
qMFDRLNRRJJphbhSgStpptbj
rZrPZZNCCCGrlfsCzBbgstBwtvbtvbvBth
CfVrdndzZGrfzrzdrGddPnNCFMRDJWRDDnRWDFqFDJJFTJJT
dMDbndBMQWnnbDstnMbMQQwspFgsFFjRrqCCFrrprqrgjc
SlLzZmHZSTVGSPVmSPlSmrpFvFjjpgrpgFcvrcFcWG
PHZZTmNVHSWJlHPNLSzPLLHwDBJtbDbnnnMffDbwtnMndQ
tHBzNCztLBRBtrjvSjfnjvvzfpjj
gQwWqlnqWqJgJnDplfflddjdsfdpFf
gcwwmnDccTPWWgJbZNNbZHZCRRTrLB
ZhjgtrJNfDNpqbhqQmqpmb
cFLcwGGHwcGdwFCQjnnnVcmqmjCp
HvvlzjWHjFWTggWStDrDZT
GPjjQtPQbjwWqrmnsjmnqn
SNGdfLLGdlldZSSRWznFsNqFRnqsWs
DdMpMfZMhfZMpDfbDQQbGQVgccgcVt
hHGGGTlddWGgpRdcvwDCDwzgLJzCwzwgwL
MFSbZSnnFJWwQBNMzw
tSjrSFSfqjqqttPnssqjssbhGmWGhcmvldcmhHRhGRhTRf
qdBjBTNndbnqnLmtZmZvvtLvHd
hDJCpfnGhJfDPzGzzCnGPmpZZVLZvHttZHQLHgZLHt
JDrfCDJhGMhFhPzMrCCnrGSwswswFsblTqRlRwjcjRsqBT
tJPRSZCSJJCnmvvvQMrpqLVwqLqMcCCM
hhGGGfsdfTGlfggjMLVVFsMVwMMbqbLF
hdTGGhGhdhfhhwlgNfdhZnZtHHRNtZHnnZmHHzmn
HlgRZglZDWZgfVbdznHddTvV
MhShhQMSFShFPPQplMPmwppwbzdtVbFtfvfbzVbvbTntzbVT
lMwpmJLLLNGwBrcZCjrGCGrD
qjjWRLjNjtGRRWTCghNwsgwcbwmC
HPMBpVGMPMvvVBPswmhTbCwgmsVThT
vDSMSvflBlHpvMMfSSpMFWZdtZWdGWGddqfFtRqQ
ZmnGQfnZgdmRGQGvgnnmHCbbchhpMLrpcbLpdLpzbz
NBFPBWFsSVNJlFqLDLwcwrPLzzCwhp
qlWJqqWFJJjNqJWsFVsVqqRCggtmvvjQZgQnZQmvmHRt
RdCsJbdsVJtRvdzBzrBcjzMTqT
wNSNnnHhnwhHfBHqHjBDJMJr
GnSPLwlFwwLWSwpNWLSNpVZtvPsVJsmRCmtgsvsRsv
GPMwMMPCBPwBGsCGGWqBsslQhVQFccphvhWpmcFhVLmV
rbNnHLLHfHfZFfQQQZml
DzSDNtrztbgCMMBLLCsBts
rTtrVbrrhbbGGhbbbbRqccdBcdBcvRvBSRRV
fDqQDqLFQgQgZFMlFNRgvCNvRRvHvRBHHR
mfPqmFsqqsbJJtGtbt
vNHvgsSGSFDHvspvtSGwJwJNJrllhwhcnwJlwm
dqWdWfQdqQVWwnfMnlcfwzMs
VqQQqBRTqsBQWLppZLpFSHZbvvDt
gdcldHQlQndnHMzCjvCFrzjSFtbLtv
NJGmZZJZGTDsWWJNmDVmsCrSvfPrvTtSPbFFCFFvCT
BDDsmJZVBJwBRsGGDmZNBGqMdnghhcwgbngpqqMgnpgc
VhhvVwmvmwTPCwPwmDRgDCsgWSnfRMSWSM
ptHZZtlZzqbZttHbzrFqnDSMShMgSRRMngDWgrGh
hBttqBhBzlFhtHvTJQJTcvBQJPPv
FjfzfGjsjBfQfMLBNg
VlrppVwrpVSdScwTjVnCCQLQNCMBhWgV
jvtSrSjjtSZpqtHPDPJqRzGR
ZSmbSDswfCDDHBFFvWHJ
jcjcVjltntQMltnVrdNnNjdQgFzWHqzBWGWQvzHwgqqJGz
nccLllwwhLCCSLTmmRTP
HBSnnJSfHvBfNWMNrvnMrZlblFbsbHwsPFVHTsHFls
VLmDLhRgRbZFRwjZ
zQDmCLLDDLGttGGgtLvvVSfqJBSVftSnrJnN
CgGhbgVMNgVVbjrrtcfTDTfvTscrTvSSss
qqFzFBzsPZHmddmsmvWSnHJnncDfWnDJvv
ZwdPqdqpdPlPwdRlsdBqBMjNQjGVgbQgQhGVCCtgbR
CGFTTMLGPgmPfGfCwTPSSFNQDLNQnscQNccWdQLqvqds
pbJlzhZRHtjrbrbjHHrplRWVNsNvddZNWDQVVcnQVscs
pzJtlJlBhJJHJjHhfPfTTFMDmMPBMSgm
gzCBPDDzgvLvgPLgNThPlVZccJTmrZNV
sndSnpsdMSnRMRpjShhNJZJhJjrVcTljTZ
SRRsdnGwRSpptnfMSSpdQtfdWqvbwFgbDJJzCzqqWqWbLWWz
BDnsPDlmmwcnCLLLwPtFTtTtFRqjRrrSVFqn
dfhhzMGzWJhpMWhHWggTHJVFVSSqbqqjdRRtvqqvFrtF
JMGfHffhQhQTNcTTBLCQmm
bfZBvvRRRzFfFFLRvFzZCcQlScchLlGNhSQGGVQh
mwJqTbsHmjbTNcNhQGGJGcVS
mPTgtsnPjwHHmmmbbRDgfpdgBpzvZD
lSnRStHtTZdjrHjnqJglbqgchhDCPCPc
swFBzvBNLpBBsvszvDJhCzgDDCgbbJQzDq
VswvBFmvpSrShmRRMn
vWBBSrWnZfCWVchwhbcjVN
RdHQQpRPJZLTtJgNcNGgbhGh
zpzDzTdqQRqRzzlRDsBBnBfMmCCSZDBS
ZtGSZVpPDtVbQjbwBDzbbL
WcFvTFnTMnnMcnhmQhmhBbBQzCQjVb
RRWsgvgnfqgpGVNg
WJTrJJCzLqCqBTWLsCCqzmPPQrjwHQQGpwGHHmRPVw
bMvnDFnFSbSlGgnpmGQVpRjV
DZSdFZdQZZclsscWqWLhcBhs
PzLlRRNjjRQzvPNQsvddlZfchhWWZJHSlhChDhHhcc
MMfngVwtpVMqVrwrMBgmtGqCHDWqJSDJWhHHSSSJJHHD
tBgnMTMMrgVmrBwMmGfnNjjsdLbbbTQvjNTNsQLz
ZZBZRmPmgpgZGLWLQWslSWmLQL
DnHJJjzqrJffrDnHzJjnMbQWSltttSbStvFQSstSvCLF
MzJDnfzHwHlljJJnqrMjfPGdBRpRBGBRhwVdGGRTBG
CnZCpMFNnFvvNdpHVrWghgtFVFlLRWhh
cBsSBsGcjGcJZDrggtgtVDVrgWmh
wTBJbcbzffJbZccjSbSjBfccNCNMPqqPPdvnHvTvnMTNdnvQ
vMgPmvQmWDMpGpjBbMMH
CcVJNcdNgdhtCVpjBBRppfRTGbph
sFsgNlcdFlJFFwFstNJcvzDWZqqWvqLqzLzmzDwQ
LLVLVsPPVVPCLLrjCNNNgmRdJNdCdfMJpB
DTZZHTWbwwpWbSWDBmJMSFFNmgRRBggf
ZvzDWqDnDwnZTpzZTzWvphPtqhQsltVtPhPhsQrPch
zDgWmDgrpCLmwgWTrjlJBQRJjbFGrcbQ
hSMvqvHtqsdVHlJDcclMBjRMQJ
sqvHsSstSdqhVVvZdqVHZDgwWpZZLLmfmmwfNfWCgmLg
SQWcTnWVWbZWWBcVPnZVbnrNrMFMdqFNqdMqqFhrDQvq
plGLlLGpJLhCGrRqGDDDrdzNGr
wgflHCCCJmpLjCLHtjjgLCtBZswnsTBPVPVsBcPTbZBBSh
vvlMQvvdjdGtVCTJlVJVfJ
FLrFqwwZgNrFWqZwgqrZBLWcBTtppztVbfRJztJbztfztT
WNmmFJwwrFFnNmDgmjdGQMdHMsPvPjsHDG
hGmZHdSRdMmhMZSHlvbTvRbRlVtCTlCR
znnfzgPPDpPfDcgnZTJvJNCZbJVCcNJV
nrLLfQznprrppgprWrnPzQzLSjGsZmHmhBdsqWhdhBMWhdqd
zhtNFSFwRFLCsNrNNBdl
BmQBPjDpBTDgHllgHc
npBjjpQpjGbMnmPpjPQWpwZhtbzJfhwvwtSwhhFFbh
PZcZbcPlbSprcQbbdCwWRSttgtgvWfjC
TGVLVHHmTVHGDTDnGDhgWjwvCjwwwRLLgBjWBR
CHTGsnHVVcJPPcNsNJ
tTqGSSGPGfVfTpqGTbbcVWJLdjtvdzjJthCjlhdlzQ
wwFBZMmZBmgnjzlCWBBjBLjv
rNMsDZnMMWSfDcDWPR
vLzbsczhLmmnlNvrNQHfWd
SMSFqMwjFFDVSZwVTMDjSQlQfNlRrQRWdQfRrWrqHN
CPTwGZDTFCPSjFTSPSFbPgpLscPczmcBLbfgpb
QnQnpFjsbFcSSvCMNvqVSrqq
WfzfTfzzPgHTfwfWtgRLMJDvjMmMVtvDJJCVtqmC
wfWRgPzdgRTWBBWHPBHHBRLTlcZdhjnbZQcGZGpsnphjshbG
pHzPTsBHzqqtQCZZshlWjf
DFFbnvJMDMljjtQjfCbQ
dgJFDGwgmGlMSggGdgdDDlvLNTzpBzLzzwpTBLzqPBczLT
jccNVNdwnclRwlbwlVjdcpJSpGpSllBHgGHZpJpppf
mDThTmsnDsSBpZfmmgBf
MrFrhCshqvWvnWzTWQtzVbRRQwQRNQjdwQ
tBnLJfnQtzRCffmNjSRjZjNZSRrP
dVdMVMvMghHzPhzZhHND
dGWGgplWGVMdMMzCTsbLtLTCLpnT
ftNfNDdSBdrMTdrjMM
cHgHGHzGgJhrPLqSrrJTqp
mVFnQnhQGHFznFhBBbSDRBlSVBBRfS
VqqPBPcPbQHgfrrpcSDR
tpMnsztnGnthhzTtGTGTzzWgJNDlMfSRlRDgRNRrDRHfrg
zWtTFWzTwphChnCzFhzWZGGvBvLqmVmbVQqjqjmBPqBLCP
ZZgZnhrmwmnmgmvrghPmgTGcTSGSMSldgcCQCSqW
LzFLDBfHzHCCqCFGcSlS
HBjRJDLpHpJsJVJqsnhP
PqrqmvmrwzznnPDpjVpDLfDtPGLt
sdRhRWFhShhFccZZsSsNbsNcjCtLpMVWCjptGCfMftBDLMtG
bVZcsdlhdNSbZRSshRcbbqlmnqHmmwzrJlzzgQlmvz
pnrcNGqmrGqnchGhqdWdTlldtQtlMsTq
DvSLgzLSMfbgggCLCwbSSLLtfsWTQTsWllssdltRQQtttQ
bCCzzPPgDPjPvwSzDbwpVNnpnjjrhpnVZGMhcp
jWbGtDdqCqZjdHwcwZMBVQmcvZ
PnTflPRRrlgLTTRlTzFPPQQBcNvHBncQpHMwHNBMwm
fFFRrFLJgRcJglgRzTzrLqWCjtqGGDsjCjdbGdqdhJ
GwbvGqMsDMbpMGzzgRzgpBLjhcch
WFTFNZTZSCcBggBFcrss
CJWWlsWlCtqGJmMGwJ
CGCVhprTrthCZTCNtVGtZDZNdlPPdPwmmvrcbmPmdQRvQWmw
LfzLzssfgHjLFjFLfjMfHsLHmPwdcQWQQlscwlPdQclbPvlw
LBFHjgMzqqjfJqLMzffHzqgHhhJDZSGVDVChCDDpDNNpNtDW
TbzVlmNTVVtnTSWNwDDrpGcwdp
fQQMFbhCfLgfQCsdDcHpsWpdSDsGrr
ghvPLQMfZhjjvPLhbQFQBZqJlTnnnVzRmtRzlmBl
JCLLLwVDwCQsNwwJHmfrMZpMfMMrfPQSMZpS
WlFlzFRnznthqWRGbMpVMbbMPtMjMj
vTnvzqllhdhqTwVBLcJHmmmC
tczhtcJJJbtclWrtJBWJBtJtpqPRSPfpBRgqRfPmpRqddSmM
HCvnsQLNCQwLnDsNHLwQfPSSpPSMfnPddRMmmGpp
wDQwjNwQNHjTHNFDCNmCFNWtWzhJbWbVrhtWccVFlrlV
sPRpCndBCGpCGHttSdvTbWvgdjST
wcmDwqcwmGDTNvjWtrbSrc
lwlVLVGqZGlLzVHHBBBCHBHRPCCz
wdmhffzzphrjqtzRbrrq
CRgGTGTFssZsllHNBlHsFJRjcrtjtPDPcbCtDrLcrjctrc
NHMRHTlFgGNwpQvMpwVvww
MPLJNPqmFWmDFjGS
nbsZtwbZlbZlGlFDDMpVlF
hvsbbbZtvfhhRZbZsfzMbMZbqcrqdrNrNqLgqhrLhJgddLNL
SnMLpRDGlZSZNlnMZpCwjLwzFrHBWCFWBBBr
TttvvtbtVcsJtRsvtQdzWJrBjCCHBWzrFFhCjC
VmvsQdgPbdgVTvgPMSDfPPZfDRRNDNMl
LjngLCNhDNFNhFDhcMqrqqZMcSZnHTMc
PPJwtGlfszGwWtzwQJBPGslJSqRTHZvgcRRZrMSTMMTtRTTc
llPPJWzQPmWmVNgDbLFCLb
NPFlLNBLprpdmmdPBmJnLrdjMVDjMSdqgggQTVDqWMdqVs
vZTbTZRwvvGRTRjWDqSqqQgDGsGq
vzZcfRRZbwbRHRtwZCChBmFNtJBPLBlBJJLpmFTP
pmvZmmTjQFfnvPPHHv
SczhzfbsLNhfccNFsWFRPrDnPDnVnW
BtBbSdtzLBwSLwBmTZpQMpZmZmfZ
PqPQZqtQQLDqrnqdjqdVwVbz
MGRGWMgJHGlRRHfSwfzCfCVVQzbwjrjn
JsmHmSJJmSMWMlTWQBFhLTvhDFZhBQDZ
sbgbbFGTTFNMbMNFWrjsrvWzHWPzPPpf
CVmhVqSqCZmJQhPpHzvZvtzWHjHP
CnmdCQCdnFTnNgRpwT
blZjhbZWVttjWjWLCLVVZCZQjMDQHsBsBQfMDQwjHDwBHH
NdcJdFcJqgpJpNnDQLMFsBnLSnnS
LJrJdJrzvdrrpcNdNcrVlZWbZzZVRhRtVlPttC
mhRtNNtrtBQQrtrBBmQlZwHHqHZSVHHGshSVDwhS
gpdPMTcsLscMccTpbLdHSfGfqwHZDHHqZqHZ
zbzvMpPLppLzLMjTBQRmssjBWRQjlmrN
VzzvggdvFdmffwmGpd
HNbnJTRTmCwwrRpR
LlTnWhLlhLJmLmtZtPcPcVFFPSZgZt
jHcZjHlHzLHHnSNSfL
pQWRrwPwrRWBWBPWBRrpdPmzhShsSFFNShLhnnvPtvSNNs
dzbmwVwbbBmGcqDbgllJCC
ndnvvzJDHvzHHHjnHjCCSDLgbSFwNFVbFVTL
mcQmQtpWTQGlmpTtMtqtpqTFsLcVSSscNCwLNLbwbbLNwV
pRZTQlhmtGWqqWnPHdnhjHrBjPPd
TtLpNHspTcLNNsLpthhsfmtjRSRlWWbzSwSRGwbWlWSSvlmS
qBJVnZZdJVZrZndbPbWwRzSMVGbbVS
QCdnBFBndBQDnrqrnqqNhpNNLpHthsThjGCTLL
phCgcdrFbPLpgrbFHqQqzzlbGWGqQbHW
SFTvTnVVMRnNTNfSHjHQMDlHwDWlQwDz
TZmTvsFNmvTtsggpdJLBBsCs
PBBWQjvsPsHVsNMcSzNDjcGggS
ZtrTfTrrrrdCqpdtLNnMLLqNcgMzgHLq
CZFmdTrJtbZrBvWHVVvHbPQW
djcrrBljMrTdCTcdCClClMlqRvtNqqSRwFbNbwvNBNpSzq
QhPmGJnPVGVHHNzSqpzFwztF
nhgPFmsnLPGLhPDJhGTcDjMfrMMjMZWfjfWj
"#;
