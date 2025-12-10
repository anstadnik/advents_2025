pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
            true
        } else {
            false
        }
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut groups = vec![Vec::new(); self.parent.len()];
        for i in 0..self.parent.len() {
            groups[self.find(i)].push(i);
        }
        groups.into_iter().filter(|g| !g.is_empty()).collect()
    }

    pub fn one_group(&mut self) -> bool {
        (0..self.parent.len()).all(|i| self.find(i) == self.find(0))
    }
}
