use crate::queue::{Task, WorkQueue};
use digest::consts::U32;
use sha2::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::sync;


type Hash = GenericArray<u8, U32>;

#[derive(Debug, Clone)]
pub struct Block {
    pub prev_hash: Hash,
    pub generation: u64,
    pub difficulty: u8,
    pub data: String,
    pub proof: Option<u64>,
}

impl Block {
    pub fn initial(difficulty: u8) -> Block {
        let prev_hash = Hash::default();
        let generation = 0;
        let data = String::new();
        let proof = None;
        Block {
            prev_hash,
            generation,
            difficulty,
            data,
            proof,
        }
    }

    pub fn next(previous: &Block, data: String) -> Block {
        let prev_hash = previous.hash();
        let generation = previous.generation + 1;
        let difficulty = previous.difficulty;
        let proof = None;
        Block {
            prev_hash,
            generation,
            difficulty,
            data,
            proof,
        }
    }

    pub fn hash_string_for_proof(&self, proof: u64) -> String {
        let mut output = String::new();
        write!(&mut output, "{:02x}:", self.prev_hash).unwrap();
        write!(&mut output, "{}:", self.generation).unwrap();
        write!(&mut output, "{}:", self.difficulty).unwrap();
        write!(&mut output, "{}:", self.data).unwrap();
        write!(&mut output, "{}", proof).unwrap();
        output
    }

    pub fn hash_string(&self) -> String {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_string_for_proof(p)
    }

    pub fn hash_for_proof(&self, proof: u64) -> Hash {
        let string = self.hash_string_for_proof(proof);
        let mut hash = Sha256::new();
        hash.update(string.as_bytes());
        hash.finalize()
    }
    

    pub fn hash(&self) -> Hash {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_for_proof(p)
    }

    pub fn set_proof(&mut self, proof: u64) {
        self.proof = Some(proof);
    }

    pub fn is_valid_for_proof(&self, proof: u64) -> bool {
        let hash = self.hash_for_proof(proof);
        let n_bytes = self.difficulty / 8;
        let n_bits = self.difficulty % 8;
        
        for i in (32 - n_bytes as usize)..32 {
            if hash[i] != 0 {
                return false;
            }
        }
        
        if n_bits > 0 && hash[32 - n_bytes as usize - 1] >= 1 << (8 - n_bits) {
            return false;
        }
        
        true
    }

    pub fn is_valid(&self) -> bool {
        if self.proof.is_none() {
            return false;
        }
        self.is_valid_for_proof(self.proof.unwrap())
    }

    pub fn mine_serial(self: &mut Block) {
        let mut p = 0u64;
        while !self.is_valid_for_proof(p) {
            p += 1;
        }
        self.proof = Some(p);
    }

    pub fn mine_range(self: &Block, workers: usize, start: u64, end: u64, chunks: u64) -> u64 {
        let clone = sync::Arc::new(self.clone());
        
        let mut len = (end-start)/chunks;
        if len == 0{
            len = 1;
        }

        let mut q_work: WorkQueue<MiningTask> = WorkQueue::new(workers);

        for chunk in 0..chunks {
            let chunk_start = start + chunk * len;
            let chunk_end = start + (chunk + 1) * len;
            q_work.enqueue(MiningTask::new(clone.clone(), chunk_start, chunk_end));

        }

        for _ in 0..chunks {
            let r = q_work.recv();
            q_work.shutdown();
            return r;
        }
        
        0
    }

    pub fn mine_for_proof(self: &Block, workers: usize) -> u64 {
        let range_start: u64 = 0;
        let range_end: u64 = 8 * (1 << self.difficulty); // 8 * 2^(bits that must be zero)
        let chunks: u64 = 2345;
        self.mine_range(workers, range_start, range_end, chunks)
    }

    pub fn mine(self: &mut Block, workers: usize) {
        self.proof = Some(self.mine_for_proof(workers));
    }
}


struct MiningTask {
    block: sync::Arc<Block>,
    start: u64,
    end: u64,
}

impl MiningTask {
    fn new(block: sync::Arc<Block>, start: u64, end: u64) -> MiningTask {
        MiningTask { 
            block, 
            start, 
            end, 
        }
    }
}

impl Task for MiningTask {
    type Output = u64;
    
    fn run(&self) -> Option<u64> {
        for p in self.start..=self.end {
            if self.block.is_valid_for_proof(p) {
                return Some(p);
            }
        }
        None
    }
}
