#![feature(test)]
extern crate test;

use itertools::{self, Itertools};

use std::collections::{hash_map::RandomState, BTreeSet, HashSet};

fn main() {
    let p1 = p_skipping(REAL, 4);
    let p2 = p_skipping(REAL, 14);
    println!("{} {}", p1, p2);
}

fn p_btree(s: &str, n: usize) -> usize {
    s.as_bytes()
        .windows(n)
        .enumerate()
        .find(|(_, window)| BTreeSet::from_iter(window.iter()).len() == window.len())
        .unwrap()
        .0
        + n
}

fn p_hash(s: &str, n: usize) -> usize {
    s.as_bytes()
        .windows(n)
        .enumerate()
        .find(|(_, window)| {
            HashSet::<_, RandomState>::from_iter(window.iter()).len() == window.len()
        })
        .unwrap()
        .0
        + n
}

fn p_itertools(s: &str, n: usize) -> usize {
    s.as_bytes()
        .windows(n)
        .enumerate()
        .find(|(_, window)| window.iter().all_unique())
        .unwrap()
        .0
        + n
}

/*
test tests::bench_p1_btree     ... bench:     146,111 ns/iter
test tests::bench_p1_hash      ... bench:     161,523 ns/iter
test tests::bench_p1_itertools ... bench:     169,472 ns/iter
test tests::bench_p1_skipping  ... bench:       1,152 ns/iter

test tests::bench_p2_btree     ... bench:     861,028 ns/iter
test tests::bench_p2_hash      ... bench:     692,197 ns/iter
test tests::bench_p2_itertools ... bench:     319,863 ns/iter
test tests::bench_p2_skipping  ... bench:       1,147 ns/iter
*/
// in theory to make this nice we'd what need 1) an iterator that does combinations
// that the j/k loops are doing, and 2) some kind of iterator adapter that lets the inner
// fn choose how many elements to skip, a la the outer loop
fn p_skipping(s: &str, n: usize) -> usize {
    let s = s.as_bytes();
    let mut i = 0;
    'outer: while i + n < s.len() {
        let window = &s[i..i + n];
        // we want to find the furthest most back dupe
        for j in (0..n).rev() {
            for k in (0..j).rev() {
                if window[j] == window[k] {
                    // we found a dupe at indexes (j,k) so we know that
                    // the next k windows must also have a dupe, so we skip them
                    i += k + 1;
                    continue 'outer;
                }
            }
        }
        // we found no dupe, so we're done
        return i + n;
    }
    unreachable!()
}

const TEST1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
const TEST2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
const TEST3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
const TEST4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
const TEST5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn p1() {
        assert_eq!(p_skipping(TEST1, 4), 7);
        assert_eq!(p_skipping(TEST2, 4), 5);
        assert_eq!(p_skipping(TEST3, 4), 6);
        assert_eq!(p_skipping(TEST4, 4), 10);
        assert_eq!(p_skipping(TEST5, 4), 11);
    }

    #[bench]
    fn bench_p1_btree(b: &mut Bencher) {
        b.iter(|| p_btree(REAL, 4));
    }

    #[bench]
    fn bench_p2_btree(b: &mut Bencher) {
        b.iter(|| p_btree(REAL, 14));
    }

    #[bench]
    fn bench_p1_hash(b: &mut Bencher) {
        b.iter(|| p_hash(REAL, 4));
    }

    #[bench]
    fn bench_p2_hash(b: &mut Bencher) {
        b.iter(|| p_hash(REAL, 14));
    }

    #[bench]
    fn bench_p1_itertools(b: &mut Bencher) {
        b.iter(|| p_itertools(REAL, 4));
    }

    #[bench]
    fn bench_p2_itertools(b: &mut Bencher) {
        b.iter(|| p_itertools(REAL, 14));
    }

    #[bench]
    fn bench_p1_skipping(b: &mut Bencher) {
        b.iter(|| p_skipping(REAL, 4));
    }

    #[bench]
    fn bench_p2_skipping(b: &mut Bencher) {
        b.iter(|| p_skipping(REAL, 14));
    }
}

const REAL: &str = "zdnnfgfsgffgllwrwprwrgwwpssznzrnznllstszsttpdptdpdmdsdzsdscsmcmttdllbsbwwtwnwswcchshlhjhfhwfftcchnnfwwbqwqwrqqgmgzmmwzwfzwfzzzsmzzrczcmmhphzhbbbgdbgddmggwwbbttvmtvttfsfttjlttdfdsdqqczqzffbrfbbfbrrmdrrlslshllwzwrzrzzlqldqdjdjwjvjzjrjjcsszjjqfqnfqqsrqrccbhhwphwhbwwlzwwjwfjwfwzfwzffssvjsjddcsdslslrsrfsrffsggdffrcrcdcpprrzbrzbbtstvvqttbqqgfgsggtvtrvtvbbrqrsqrsqsvsbbzmbmgmvgmmrqrzzbbnjjlwjjfssdrdbrbffwrrrjjgcgtgvvjbjjjsqjsqqncntnndcdrcrhhsgstslldwdbwdbdtbdbggpnndhdvhhvrrlzzfjjffzszvvzgzhhqzqttdhdrrwdwzdwdbwwfsfwsfwfqqzwzbzmzwmmvgggvssvwswfwswhwzzqtqrtthhbbjggjppnpfnfmnnghgrhhtvtqtsttbpprzzwqqfhqfflttzffrprwpwspplzpztptgtltjlttwwsrsrprwrsswnwttcscqsqlsqqhshbhlblnnpznnzlnndrnrcncvcqcjqqvhvppjzjddzbzsztzqqlmlnlnblnlwnllswszwzrzddqhdqhqffhfhjjpbbhvbvmmfhmhcccjlclhcllbrlblddnpplggcmmvddmmqzzmqmppnjpjjjzllqjqccrwwhzhnnlmlhhbbtztvtltvtnvtvltlrtrllbttbzzfwfrrjzjbbmgghjhqhlljnjhhhjzjcjvcvwwzczwcwgcclflnnsvvcncbncnqqpjqjnjwjrrgqrrmqmfmmmjgjfgjfftggqdgqddgcdcdwdrdsswqqphpjhhdjjwswfwfnfqnqccvhvzvmzvzzplzljzjpzzhmzmqzqjjprpvrrhqqnbngnnpvpfvpvqvwwrhhdndqdppmcppzddbzzjcjdjfjnfnngqnnchcqqpllpwppgllcblcblbddzhhqsqbssjqqgmgbgzzvhvnhvnhvvpmpvvlddgppzrprmpmbpbjpbpfptpspddcgddqhdhhthrhjrhrvrlrffvbfbvfvbblssftfnfwwrwnnzdznnbwnbwnwlwttszzmmlzlhhpjhhjvvlvwwdnddzwdwgdgdssflfvfzvznzbzrbzzdssphhgttllcjcvcjjdrdqdhdnndlndldcllcnllslvlnvllmglgnnplpmpzzjwjtwtnntrtjtvjtjffhcfclltppftfwwprrwsrwwzdwzdztzccbmccfcfzczbcbsbqsstjtrrpnnfqqfmmchmmwmrwrwzwztzddgzdgzzfwzzrppcscrrgvgvgvtgtsslrsrvsrrdcdscscwcwqqsccwjcjgjvgvpvnvhhchrrgprpvrvsvsttgghdghhmphmmbvbcbsccdbcbnccbnnsjnjhnnzbnnpjbwdpczcvgjpgwfqrmnvwncflvnttwhfgmfqvngpdhbhvlglfhtdqmqtqcgjcqghzvbdghdgvjcsjrlpqvgcdnbpqrcrcvqqdlcpscqbfpsnhzcdbbcssslrjlzsqpprsbmtqhzblvwbswprhztmpcgfqfsgshchrhjmwwhpzsjzrmrvgdgwjrlwpgqhbzrmnmnnsnvzsrlhthgvlpljsjrpbhbzctdqgvdjcmrgtvqjqbcwsprnfmntzpbjcdtlchhjgwpmldmsstbtztfdbgbstgnlwbzrrzmvbrhnrlcwfgwwbfnntbjspqwngbjrvhdcnblqssgjlbcwbbgphhnmfcmdhqdhsnmvdjnwwwjlffswhsmwqrsprftjwtbtcvmpctgvfqvvcjpnwzqldglfbwfzpnqmdlrdpjmjptvwsctlmhmzzgvplglfgsvrfbqbmrhplczbvqpdjjhhvfqswhzhqfgzstwwpbtbsnnlgpshwqgppzbpsfpfvcntbbbzwdnfcgcwzbqwmhjrhpdfvpbzpmfnmllrcqlqhcbzfltzcgccwwqmtsmwchhvbqtdrnsbrchqqcmtfqpddcjplbvdhhtndrrmfdtmbpdvwthvgdccnrcqmpznlvzqzfjqmpvgjtfbtfjnrmlzhwhljrrqnbqzpfhcvncblfggrtbdfjqnlgpbrzmwcvrvjtjscfmcnfjgqzqsphldvhdbpvmghrvsdmvpmvvdmdhwdghtjltmlcmfhvrsvcvpblwhhfcfdqnrsjbcldgbwhtnjntmgvprhbjrcvsmhgtfphcwncpjtngqhvwrmgprstbtdstmttpzcntmzvncwslqlldpnjbtpmsfnwbpwpnlfgdvcqplvlqqjvfftnnvpcmwjrvwqhlrshftrbhcwnczzsnvtnjnrbzzgzfsqhnfwlcgzvvhqcgvqtmcpnhlvdlmwgsvtwbqgrdsrrddszvscbgtlpwpzjrbvwhjnrpprhtzmthbpfzvplzwfdtnwqwtctgjslmcczjvwplsqwgfnfbgdjbsdpwbgflttvvqlhzgmmpjsnwbqqtcdszfqbhgnmbbmrbrgnrzdmzwnjjzjqcwqcqfchjrzlspgbrchcbgwbhvggsqbvdpzbpnwdtqvcjwcwnbjdhsdfmbtwfbfhzwwtnqzhmtvtbfwrsqjzgssvlwszvlmvbslpncnhmsdhcqqfpftztpzbbhsgbnscddbjlgwgjjndgwbrhwmsfdmmsnlwgwdsdltwjfvwnczjrbgcvsfczppltdptlgcdfzgmqpjngstldqgmwhdmfrwwfqwdgswvfdrtsgtvttpcbnhzbscnchpvfjvbcszbwchnbmfrvsswslbzlhgwlvfchdbfthbpdbwwqtmlgwjqtjhzrjzzmrpdwnvfgrnqdcqmwtttmwjvgcmjsddvtlswldzhtppwvhmlghwlgblfttctnglwhtfvqgjmdjcnflsrjvpjwcjfftbdmmcbqvfwnnnzsltllncbstgnhtmpsltgztqzjbbrtqpcvdlnhpnhvmmztpfpplbqjlpqvfsdvhwvstdmqbtnpzrcbdhvdtghqwcppcfzjfjsfwvqrtfgcdzwgzjvrqqsjtnhlbjcmtjcnmtpffwcwhqqphwjsrhpqvnnhhrcnvztfdjzbjggwlgjprbpssgnmtcrvprwbsrfvvphrsgzgbrfnpgtqbbprhfphqntsglrmhzfnwqptlslnhtrhfprjpdcglcffsblnjwczmgwhmmtgsgwljmqlvdglqmzwmtqcvgcrmqjldlsnbssdvrrtltngvrsqbctqlsngqvcphjvhmwsssgwmvgzdctjcmjtpcjhvfcrfhbffdqfjjvpqwgvnlzhgfnfmlrrfvjrdvhzdcvdvmpncvtjbbnczpzmglfqnpbsrsjwgvszsnqvrnvlhmqjjnmsfngbdlpwbqllcptjtlbhrfdvhlrpdlznpvndjzjdtjflqqjdgjjpmnpmjgcglllgcqbfpvdtpbjdnvrclmnlfdrpbmwzgvdhgbzvbhwqfslhshbfcbwrnsjndgjgccllfbzgmcjqcmdnfftnccphqtwmgqgfqlvlwsrprctchqrscwvgpdrwgcfgzjwmzmmsmwzgtzsjtqfggcczcmghlqgnqqjvsrsfrrmwmnrnhbsszmwsqlrggsbdwzzfnhwcggjszfrlffplvcblvphqmzjnzwzdshhdprfrdbcrmbtztcfvgpzpmmgflswphvnvtwhbbhjwffsvqfjlfvzqmhmsmddwdwsqfnnplbqnptbvgjqgmflsbfdtpvdgbfnqmcqznhpqbpwtbfpqllvqwvcftdjjtlsvzbssbtcdzqqqvzlqhfpdthscqmvhpndmnztthvvzccqswswspnqcbncvszrgjshjhdsclrjdnjdczqmcjldbspclgrmwqdvcvpcsvjggfdqlrwlnzptfvcwjsgblpjzgcrrmjqptvdnwr";
