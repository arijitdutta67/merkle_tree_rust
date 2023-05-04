use hex;
use sha2::{Digest, Sha256};

fn main() {
    let data = vec!["Hello", "world", "3", "4"];
    let m_tree = MerkleTree::new(&data);
    println!("Root hash = {:?}", m_tree.root());
    for i in 0..m_tree.tree.len() {
        println!("tree[{}] = {}", i, m_tree.tree[i]);
    }
    let proof = m_tree.compute_merkle_proof("world");
    m_tree.verify_merkle_proof("world", &proof);
}

struct MerkleTree {
    levels: usize,
    tree: Vec<String>,
}

struct MerkleProof {
    index: usize,
    hash_path: Vec<String>,
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
        MerkleTree { tree, levels }
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
    
    // Compute merkle proof

    fn compute_merkle_proof(&self, input_value: &str) -> MerkleProof {
        let mut proof = MerkleProof {
            index: 0,
            hash_path: vec![],
        };
        let element = Self::sha256(input_value);
        let index = self
            .tree
            .iter()
            .enumerate()
            .find(|(_, &ref x)| *x == element)
            .map(|(i, _)| i);
        match index {
            Some(idx) => {
                println!("Element found at index {}, computing proof.", idx);
                proof.index = idx;
                //let levels = self.levels; //(self.tree.len() + 1).next_power_of_two().trailing_zeros() as usize;
                let mut reverse_bits = vec![0; self.levels - 1];
                proof.hash_path = vec![String::new(); self.levels - 1];
                for j in 0..self.levels - 1 {
                    reverse_bits[j] = (idx >> j) & 1;
                }
                if reverse_bits[0] == 0 {
                    println!("hash path index = {}", idx + 1);
                    proof.hash_path[0] = self.tree[idx + 1].clone();
                } else {
                    println!("hash path index = {}", idx - 1);
                    proof.hash_path[0] = self.tree[idx - 1].clone();
                }
                let mut offset = 1 << (self.levels - 1);
                let mut j = 1;
                for i in (1..self.levels - 1).rev() {
                    if reverse_bits[j] == 0 {
                        println!("hash path index = {}", offset + (idx / (2 * j)) + 1);
                        proof.hash_path[j] = self.tree[offset + (idx / (2 * j)) + 1].clone();
                    } else {
                        println!("hash path index = {}", offset + (idx / (2 * j)) - 1);
                        proof.hash_path[j] = self.tree[offset + (idx / (2 * j)) - 1].clone();
                    }
                    offset += 1 << i;
                    j += 1;
                }
                proof
            }
            None => {
                // Element not found
                println!("Element not found, returning invalid proof");
                MerkleProof {
                    index: 0,
                    hash_path: vec![String::new(); self.levels - 1],
                }
            }
        }
    }
    
    //verify a Merkle proof
    fn verify_merkle_proof(&self, input_value: &str, proof: &MerkleProof) {
        let element = Self::sha256(input_value);
        let index = proof.index.clone();
        let hash_path = proof.hash_path.clone();
        let mut reverse_bits = vec![0; self.levels - 1];
        for j in 0..self.levels - 1 {
                reverse_bits[j] = (index >> j) & 1;
            }
        let mut temp_hash: String;
        if reverse_bits[0] == 0 {
                temp_hash = Self::sha256(&(element + &hash_path[0]));
            }
            else {
               temp_hash = Self::sha256(&(hash_path[0].clone() + &element));
            }
            let mut j = 1;
            for _i in (1..self.levels - 1).rev() {
                if reverse_bits[j] == 0 {
                    temp_hash = Self::sha256(&(temp_hash + &hash_path[j]));
                }
                else {
                    temp_hash = Self::sha256(&(hash_path[j].clone() + &temp_hash));
                }
                j += 1;
            }
            if temp_hash == Self::root(&self) {
                println!("Proof is correct, membership verified!");
            }
            else {
               println!("Proof is not correct, membership could not be verified!"); 
            }    
    }

}
