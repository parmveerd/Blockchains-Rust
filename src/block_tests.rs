#[cfg(test)]
mod block_tests {
    #[test]
    fn example() {
        assert_eq!(1 + 1, 2);
    }
}

use crate::block::Block;

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::Sha256;
    use digest::Digest;

    #[test]
    fn test_block_mine_serial() {
        // Test mine_serial function to see if it finds the correct proof
        let mut block = Block::initial(5);
        block.mine_serial();
        assert!(block.is_valid());
    }

    #[test]
    fn test_block_mine_valid() {
        // Test valid block with mine function
        let mut block = Block::initial(4);
        block.mine(1);
        assert!(block.is_valid());
    }
    
    #[test]
    fn test_block_mine_invalid() {
        // Test invalid block with set_proof function
        let mut block = Block::initial(20);
        block.set_proof(54321);
        assert!(!block.is_valid());
    }

    #[test]
    fn test_block_next() {
        // Test the initial function with the valid and hash_string function
        let mut b0 = Block::initial(16);
        b0.mine(1);

        assert!(b0.is_valid());
        assert_eq!(b0.hash_string(), "0000000000000000000000000000000000000000000000000000000000000000:0:16::56231");
    }

    #[test]
    fn test_block_hash_for_proof() {
        // Test to see if the hash_for_proof function is working correctly
        let mut block = Block::initial(16);
        let proof = 56231;

        let expected_hash_str = "0000000000000000000000000000000000000000000000000000000000000000:0:16::56231";
        let mut hasher = Sha256::new();
        hasher.update(expected_hash_str.as_bytes());
        let expected_hash = hasher.finalize();

        let actual_hash = block.hash_for_proof(proof);
        assert_eq!(actual_hash, expected_hash);
    }
}




