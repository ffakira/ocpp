use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root: Vec<u8>,
    pub leaves: Vec<Vec<u8>>,
    pub tree: Vec<Vec<Vec<u8>>>, // levels of the tree
}

impl MerkleTree {
    pub fn new(leaves: &[Vec<u8>]) -> Self {
        let mut tree = Vec::new();
        let mut current_level = leaves.to_vec();
        tree.push(current_level.clone());

        while current_level.len() > 1 {
            let mut next_level = Vec::new();

            for pair in current_level.chunks(2) {
                let hash = if pair.len() == 2 {
                    hash_nodes(&pair[0], &pair[1])
                } else {
                    pair[0].clone()
                };
                next_level.push(hash);
            }

            current_level = next_level.clone();
            tree.push(next_level);
        }

        let root = current_level[0].clone();
        Self {
            root,
            leaves: leaves.to_vec(),
            tree,
        }
    }

    pub fn get_proof(&self, index: usize) -> Vec<(Vec<u8>, bool)> {
        let mut proof = Vec::new();
        let mut idx = index;

        for level in &self.tree[..self.tree.len() - 1] {
            let is_right_node = idx % 2 == 1;
            let sibling_index = if is_right_node { idx - 1 } else { idx + 1 };

            if sibling_index < level.len() {
                proof.push((level[sibling_index].clone(), is_right_node));
            }

            idx /= 2;
        }

        proof
    }

    pub fn verify_proof(leaf: &Vec<u8>, proof: &[(Vec<u8>, bool)], root: &Vec<u8>) -> bool {
        let mut computed_hash = leaf.clone();

        for (sibling_hash, is_right) in proof {
            computed_hash = if *is_right {
                hash_nodes(sibling_hash, &computed_hash)
            } else {
                hash_nodes(&computed_hash, sibling_hash)
            };
        }

        computed_hash == *root
    }
}

fn hash_nodes(left: &Vec<u8>, right: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().to_vec()
}

pub fn hash_leaf(data: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hasher.finalize().to_vec()
}
