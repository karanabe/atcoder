use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let _: String = solve(n, a);
}

fn solve(n: usize, a: Vec<i32>) -> String {
    let mut current = -1;
    #[allow(unused_assignments)]
    let mut num: i32 = 0;
    let mut result: String = "".to_string();
    let mut hashmap: HashMap<i32, usize> = HashMap::new();

    for (index, element) in a.iter().enumerate() {
        hashmap.insert(*element, index);
    }

    for _i in 0..n {
        num = *hashmap.get(&current).unwrap() as i32 + 1;
        print!("{} ", num);
        result = format!("{result}{num} ");
        current = num ;
    }
    println!();
    result.trim_start().trim_end().to_string()
}


Rust (rustc 1.70.0)	cargo build --release --quiet --offline
	["./target/release/main"]

https://img.atcoder.jp/file/language-update/language-list.html

ac-library-rs@=0.1.1
once_cell@=1.18.0
static_assertions@=1.1.0
varisat@=0.2.2
memoise@=0.3.2
argio@=0.2.0
bitvec@=1.0.1
counter@=0.5.7
hashbag@=0.1.11
pathfinding@=4.3.0
recur-fn@=2.2.0
indexing@=0.4.1
amplify@=3.14.2
amplify_derive@=2.11.3
amplify_num@=0.4.1
easy-ext@=1.0.1
multimap@=0.9.0
btreemultimap@=0.1.1
bstr@=1.6.0
az@=1.2.1
glidesort@=0.1.2
tap@=1.0.1
omniswap@=0.1.0
multiversion@=0.7.2
num@=0.4.1
num-bigint@=0.4.3
num-complex@=0.4.3
num-integer@=0.1.45
num-iter@=0.1.43
num-rational@=0.4.1
num-traits@=0.2.15
num-derive@=0.4.0
ndarray@=0.15.6
nalgebra@=0.32.3
alga@=0.9.3
libm@=0.2.7
rand@=0.8.5
getrandom@=0.2.10
rand_chacha@=0.3.1
rand_core@=0.6.4
rand_hc@=0.3.2
rand_pcg@=0.3.1
rand_distr@=0.4.3
petgraph@=0.6.3
indexmap@=2.0.0
regex@=1.9.1
lazy_static@=1.4.0
ordered-float@=3.7.0
ascii@=1.1.0
permutohedron@=0.2.4
superslice@=1.0.0
itertools@=0.11.0
itertools-num@=0.1.3
maplit@=1.0.2
either@=1.8.1
im-rc@=15.1.0
fixedbitset@=0.4.2
bitset-fixed@=0.1.0
proconio@=0.4.5
text_io@=0.1.12
rustc-hash@=1.1.0
smallvec@=1.11.0


http://www.vimgenius.com/lessons/vim-intro/levels/level-2
