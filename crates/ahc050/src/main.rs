#[allow(unused_imports)]
use proconio::{
    fastout, input, input_interactive,
    marker::{Bytes, Chars, Isize1, Usize1},
    source::line::LineSource,
};

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};

#[allow(unused_imports)]
use ac_library::{
    math,
    // crt(r: &[i64], m: &[i64]) -> (i64, i64)
    // floor_sum(n: i64, m: i64, a: i64, b: i64) -> i64
    // inv_mod(x: i64, m: i64) -> i64
    // pow_mod(x: i64, n: i64, m: u32) -> u32
    Dsu,
    // new(size: usize) -> Self
    // merge(&mut self, a: usize, b: usize) -> usize
    // same(&mut self, a: usize, b: usize) -> bool
    // leader(&mut self, a: usize) -> usize
    // size(&self, a: usize) -> usize
    // groups(&self) -> Vec<Vec<usize>>
    FenwickTree,
    // new(n: usize, e: T) -> Self
    // accum(&self, idx: usize) -> T
    // add<U: Clone>(&mut self, idx: usize, val: U)
    // sum<R>(&self, range: R) -> T
    Max,
    SccGraph,
    // new(n: usize) -> Self
    // add_edge(&self, from: usize, to: usize)
    // scc(&self) -> Vec<Vec<usize>>
    Segtree,
};

#[allow(unused_imports)]
use num::{BigInt, Zero};

#[allow(unused_imports)]
use std::io::{self, BufReader, StdinLock, Write};

#[allow(unused_imports)]
use rand::{prelude::*, rngs::StdRng, seq::SliceRandom, thread_rng, Rng, SeedableRng};
#[allow(unused_imports)]
use rand_distr::{Distribution, Normal};
#[allow(unused_imports)]
use std::time::{Duration, Instant};


const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const TL_SEC: f64 = 1.8;
// ---- Beam ----
const DEPTH: usize = 2;
const BEAM : usize = 40;
const CAND : usize = 6;
// ---- SA ----
const SA_TIME: f64 = 1.2;
const T_MAX : f64 = 1.5;
const T_MIN : f64 = 0.05;


#[inline] fn inside(x:isize,y:isize,n:usize)->bool{0<=x&&x<n as isize&&0<=y&&y<n as isize}
type Dest = Vec<Vec<[usize;4]>>;

fn build_dest(board:&[Vec<bool>],n:usize)->Dest{
    let mut d=vec![vec![[0;4];n];n];
    for x in 0..n{
        for y in 0..n{
            if board[x][y]{continue;}
            for (k,&(dx,dy)) in DIRS.iter().enumerate(){
                let(mut cx,mut cy)=(x as isize,y as isize);
                loop{
                    let(nx,ny)=(cx+dx,cy+dy);
                    if !inside(nx,ny,n)||board[nx as usize][ny as usize]{break;}
                    cx=nx;cy=ny;
                }
                d[x][y][k]=(cx as usize)*n+cy as usize;
            }
        }
    }
    d
}
fn update_dest_one(dest:&mut Dest,board:&[Vec<bool>],rx:usize,ry:usize,n:usize){
    // col
    let mut last=-1;
    for x in 0..n{ if board[x][ry]{last=x as isize;continue;}
        dest[x][ry][0]=((last+1)as usize)*n+ry;}
    last=n as isize;
    for x in (0..n).rev(){ if board[x][ry]{last=x as isize;continue;}
        dest[x][ry][1]=((last-1)as usize)*n+ry;}
    // row
    last=-1;
    for y in 0..n{ if board[rx][y]{last=y as isize;continue;}
        dest[rx][y][2]=rx*n+(last+1) as usize;}
    last=n as isize;
    for y in (0..n).rev(){ if board[rx][y]{last=y as isize;continue;}
        dest[rx][y][3]=rx*n+(last-1) as usize;}
}
fn propagate(dest:&Dest,board:&[Vec<bool>],prob:&[Vec<f64>],n:usize)->Vec<Vec<f64>>{
    let mut nx=vec![vec![0.0;n];n];
    for x in 0..n{for y in 0..n{
        if board[x][y]{continue;}
        let p=prob[x][y]*0.25;
        if p==0.0{continue;}
        for d in 0..4{
            let id=dest[x][y][d];
            nx[id/n][id%n]+=p;
        }
    }}
    nx
}

fn simulate_suffix(seq:&[(usize,usize)],
                   board0:&[Vec<bool>],
                   dest0 :&Dest,
                   prob0 :&[Vec<f64>],
                   surv0 :f64,
                   n:usize)->f64
{
    let mut board=board0.to_vec();
    let mut dest =dest0.clone();
    let mut prob =prob0.to_vec();
    let mut surv =surv0;
    let mut exp  =0.0;

    for &(bx,by) in seq{
        let mut nxt=propagate(&dest,&board,&prob,n);
        let q=nxt[bx][by];
        surv*=1.0-q;
        exp += surv;
        board[bx][by]=true;
        nxt[bx][by]=0.0;
        if q<1.0{
            let inv=1.0/(1.0-q);
            for row in &mut nxt{for p in row{*p*=inv;}}
        }
        prob=nxt;
        update_dest_one(&mut dest,&board,bx,by,n);
    }
    exp
}

#[derive(Clone)]
struct Node{board:Vec<Vec<bool>>,dest:Dest,prob:Vec<Vec<f64>>,sc:f64,first:(usize,usize)}
fn choose_next(board:&[Vec<bool>],dest:&Dest,prob:&[Vec<f64>],surv:f64,n:usize)->(usize,usize){
    // root
    let nxt0=propagate(dest,board,prob,n);
    let mut cand:Vec<(usize,usize,f64)>=Vec::new();
    for x in 0..n{for y in 0..n{ if board[x][y]{continue;} cand.push((x,y,nxt0[x][y]));}}
    cand.sort_by(|a,b|a.2.partial_cmp(&b.2).unwrap());
    cand.truncate(CAND);

    let mut beam:Vec<Node>=Vec::new();
    for &(x,y,q) in &cand{
        let mut b1=board.to_vec(); let mut d1=dest.clone(); let mut p1=nxt0.clone();
        b1[x][y]=true; p1[x][y]=0.0;
        let factor=1.0-q;
        if factor>0.0{
            let inv=1.0/factor;
            for row in &mut p1{for v in row{*v*=inv;}}
        }
        update_dest_one(&mut d1,&b1,x,y,n);
        let sc=surv*(1.0-q);
        beam.push(Node{board:b1,dest:d1,prob:p1,sc,first:(x,y)});
    }
    // depth 2
    let mut pool=Vec::new();
    for node in &beam{
        let nxt=propagate(&node.dest,&node.board,&node.prob,n);
        let mut cand2:Vec<(usize,usize,f64)>=Vec::new();
        for x in 0..n{for y in 0..n{ if node.board[x][y]{continue;} cand2.push((x,y,nxt[x][y]));}}
        cand2.sort_by(|a,b|a.2.partial_cmp(&b.2).unwrap());
        cand2.truncate(CAND);
        for &(x,y,q) in &cand2{
            let sc=node.sc*(1.0-q);
            pool.push((sc,node.first));
        }
    }
    pool.sort_by(|a,b|b.0.partial_cmp(&a.0).unwrap());
    pool[0].1
}


fn template_coords(n:usize)->Vec<(usize,usize)>{
    let mut v=Vec::with_capacity(120);

    v.extend([(0,0),(0,n-1),(n-1,n-1),(n-1,0)]);

    for i in (4..n-4).step_by(4){
        v.push((0,i));
        v.push((i,n-1));
        v.push((n-1,n-1-i));
        v.push((n-1-i,0));
    }

    let c=n/2;
    for off in (0..=c).step_by(3){
        v.push((c,off));
        if off!=c{v.push((c,n-1-off));}
        v.push((off,c));
        if off!=c{v.push((n-1-off,c));}
    }
    v
}


fn greedy_step(board:&[Vec<bool>],dest:&Dest,prob:&[Vec<f64>],n:usize)->(usize,usize,f64){
    let nxt=propagate(dest,board,prob,n);
    let mut best=(std::f64::INFINITY,0usize,0usize);
    for x in 0..n{for y in 0..n{
        if board[x][y]{continue;}
        let key=(nxt[x][y],x,y);
        if key.partial_cmp(&best).unwrap()==Ordering::Less{best=key;}
    }}
    (best.1,best.2,best.0)
}


#[inline]fn choose_two(r:&mut StdRng,len:usize)->(usize,usize){
    let i=r.gen_range(0..len);
    let mut j=r.gen_range(0..len-1);
    if j>=i{j+=1;}
    if i<j{(i,j)}else{(j,i)}
}
#[inline]fn choose_range(r:&mut StdRng,len:usize)->(usize,usize){
    let l=r.gen_range(0..len-1);let r2=r.gen_range(l+1..len);(l,r2)
}


fn main(){
    input!{n:usize,m:usize,rows:[String;n]}

    let mut board=vec![vec![false;n];n];
    for i in 0..n{
        for (j,ch) in rows[i].bytes().enumerate(){
            if ch==b'#'{board[i][j]=true;}
        }
    }
    let mut dest=build_dest(&board,n);
    let mut prob=vec![vec![0.0;n];n];
    let mut empty=Vec::new();
    for x in 0..n{for y in 0..n{if !board[x][y]{empty.push((x,y));}}}
    let inv=1.0/empty.len() as f64;
    for &(x,y) in &empty{prob[x][y]=inv;}

    let mut seq=Vec::<(usize,usize)>::new();
    let mut surv=1.0;
    let mut base_exp=0.0;


    for (x,y) in template_coords(n){
        if board[x][y]{continue;}
        let (bx,by)= (x,y);

        let mut nxt=propagate(&dest,&board,&prob,n);
        let q=nxt[bx][by];
        surv*=1.0-q;
        base_exp+=surv;
        board[bx][by]=true;
        nxt[bx][by]=0.0;
        if q<1.0{
            let invf=1.0/(1.0-q);
            for row in &mut nxt{for p in row{*p*=invf;}}
        }
        prob=nxt;
        update_dest_one(&mut dest,&board,bx,by,n);
        seq.push((bx,by));
        empty.retain(|&(a,b)|!(a==bx&&b==by));
    }

    // ---------- ② ビーム 50 手 ----------
    for _ in 0..50{
        if empty.is_empty(){break;}
        let (bx,by)=choose_next(&board,&dest,&prob,surv,n);
        let mut nxt=propagate(&dest,&board,&prob,n);
        let q=nxt[bx][by];
        surv*=1.0-q;
        base_exp+=surv;
        board[bx][by]=true;
        nxt[bx][by]=0.0;
        if q<1.0{
            let invf=1.0/(1.0-q);
            for row in &mut nxt{for p in row{*p*=invf;}}
        }
        prob=nxt;
        update_dest_one(&mut dest,&board,bx,by,n);
        seq.push((bx,by));
        empty.retain(|&(a,b)|!(a==bx&&b==by));
    }

    // SA
    let start=Instant::now();
    let mut rem_seq=empty.clone();
    let mut best_seq=rem_seq.clone();
    let mut best_val=simulate_suffix(&rem_seq,&board,&dest,&prob,surv,n);
    let mut cur_val=best_val;
    let mut rng=StdRng::seed_from_u64(202507);
    while start.elapsed().as_secs_f64()<SA_TIME && rem_seq.len()>5{
        let progress=start.elapsed().as_secs_f64()/SA_TIME;
        let temp=T_MAX.powf(1.0-progress)*T_MIN.powf(progress);

        let action=rng.gen_range(0..100);
        let len=rem_seq.len();
        let (l,r) = if action<80 { choose_two(&mut rng,len) } else { choose_range(&mut rng,len) };

        // apply
        if action<80 { rem_seq.swap(l,r); } else { rem_seq[l..=r].reverse(); }
        let val=simulate_suffix(&rem_seq,&board,&dest,&prob,surv,n);
        let delta=val-cur_val;
        if delta>0.0 || rng.gen::<f64>() < (delta/temp).exp(){
            cur_val=val;
            if val>best_val{best_val=val;best_seq=rem_seq.clone();}
        }else{
            // undo
            if action<80 { rem_seq.swap(l,r);} else { rem_seq[l..=r].reverse(); }
        }
    }
    // reverse
    seq.extend(best_seq.clone());

    for &(bx,by) in &best_seq{
        // state already updated during SA simulate, skip
        board[bx][by]=true;
        update_dest_one(&mut dest,&board,bx,by,n);
        prob[bx][by]=0.0; // irrelevant
    }

    while seq.len()<n*n - m{
        let (bx,by,minp)=greedy_step(&board,&dest,&prob,n);
        seq.push((bx,by));
        board[bx][by]=true;
        update_dest_one(&mut dest,&board,bx,by,n);
        prob[bx][by]=0.0;
    }

    for (x,y) in seq{
        println!("{} {}",x,y);
    }
}
