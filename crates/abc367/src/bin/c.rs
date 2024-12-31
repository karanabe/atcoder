use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        r: [usize; n],
    }

    let mut anss = vec![];

    fn dfs(index: usize, cur: &mut Vec<usize>, n: usize, k: usize, r: &[usize], anss: &mut Vec<Vec<usize>>) {
        if index == n {
            let sum: usize = cur.iter().sum();
            if sum % k == 0 {
                anss.push(cur.clone());
            }
            return;
        }

        for i in 1..=r[index] {
            cur.push(i);
            dfs(index + 1, cur, n, k, r, anss);
            cur.pop();
        }
    }

    dfs(0, &mut vec![], n, k, &r, &mut anss);

    for ans in anss {
        println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}
