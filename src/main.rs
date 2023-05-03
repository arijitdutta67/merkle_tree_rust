use hex;
use sha2::{Digest, Sha256};

fn main() {
    let data = vec!["Hello", "world", "3"];
    let m_tree = MerkleTree::new(&data);
    println!("Root hash = {:?}", m_tree.root());
    for i in 0..m_tree.tree.len() {
        println!("tree[{}] = {}", i, m_tree.tree[i]);
    }
    println!(" tree len = {}", m_tree.tree.len());
}
#[allow(dead_code)]
struct MerkleTree {
    tree: Vec<String>,
    levels: usize,
}

impl MerkleTree {
    pub fn new(data: &[&str]) -> MerkleTree {
        let mut n = data.len();
        let mut levels = 1;
        while n > 1 {
            levels += 1;
            n = (n + 1) / 2;
        }
        let mut tree = vec![String::new(); (1 << levels) - 1];
        for i in 0..data.len() {
            let hash = Self::sha256(data[i]);
            tree[i] = hash;
        }
        let mut offset_lo = 0;
        let mut offset_hi = 1 << (levels - 1);
        for i in (0..levels - 1).rev() {
            for j in (0..=1 << i).step_by(2) {
                let left_child = tree[offset_lo + j].clone();
                let right_child = tree[offset_lo + j + 1].clone();
                let combined_hash = left_child + &right_child; //[&left_child[..], &right_child[..]].concat();
                // let combined_hash = format!("{}{}", left_child, right_child).as_str();
                let hash = Self::sha256(&combined_hash);
                tree[offset_hi + j / 2] = hash;
            }
            offset_lo = offset_hi;
            offset_hi += 1 << i;
        }
        MerkleTree {
            tree,
            levels,
        }
    }

    fn sha256(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    fn root(&self) -> &str {
        let last_index = self.tree.len() - 1;
        &self.tree[last_index]
    }
}
