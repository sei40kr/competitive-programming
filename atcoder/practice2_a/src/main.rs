use proconio::input;

struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parents: (0..size).collect(),
            ranks: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            return x;
        }

        self.parents[x] = self.find(self.parents[x]);
        self.parents[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        match self.ranks[root_x].cmp(&self.ranks[root_y]) {
            std::cmp::Ordering::Less => {
                self.parents[root_x] = root_y;
            }
            std::cmp::Ordering::Greater => {
                self.parents[root_y] = root_x;
            }
            std::cmp::Ordering::Equal => {
                self.parents[root_y] = root_x;
                self.ranks[root_x] += 1;
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        tuvs: [(i32, usize, usize); q]
    };

    let mut uf = UnionFind::new(n);
    tuvs.iter().for_each(|&(t, u, v)| {
        if t == 0 {
            uf.union(u, v);
        } else {
            println!("{}", if uf.find(u) == uf.find(v) { "1" } else { "0" });
        }
    });
}
