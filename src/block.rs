use std::collections::LinkedList as List;
use std::collections::HashMap;
use hex;
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct BlockChain {
    blocks: List<Block>,
    height: u128,
    utxo_set: HashMap<String, TxOut>, // Unspent Transaction Outputs (UTXO)
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            blocks: List::new(),
            height: 0,
            utxo_set: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        if self.is_valid_block(&block) {
            for tx in &block.transactions {
                for txin in &tx.inputs {
                    self.utxo_set.remove(&txin.prev_txid);
                }
                for (idx, txout) in tx.outputs.iter().enumerate() {
                    self.utxo_set.insert(tx.calculate_txid(), txout.clone());
                }
            }
            self.blocks.push_back(block);
            self.height += 1;
        }
    }

    pub fn is_valid_block(&self, block: &Block) -> bool {
        if block.height > 0 {
            self.get_block_by_hash(&block.prev_hash).is_some()
        } else {
            true // Genesis block
        }
    }

    pub fn get_block_by_hash(&self, hash: &str) -> Option<&Block> {
        self.blocks.iter().find(|b| b.hash == hash)
    }

    pub fn get_block_by_height(&self, height: usize) -> Option<&Block> {
        self.blocks.iter().nth(height)
    }

    pub fn get_block_count(&self) -> usize {
        self.blocks.len()
    }

    pub fn get_transaction(&self, txid: &str) -> Option<&Transaction> {
        for block in &self.blocks {
            if let Some(tx) = block.get_transaction(txid) {
                return Some(tx);
            }
        }
        None
    }

    pub fn get_best_block_hash(&self) -> Option<&str> {
        self.blocks.back().map(|block| block.hash.as_str())
    }
}
#[derive(Clone)]
pub struct Block {
    pub hash: String,
    pub height: u64,
    pub transactions: List<Transaction>,
    pub prev_hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(prev_hash: String) -> Self {
        Block {
            hash: String::new(),
            height: 0,
            transactions: List::new(),
            prev_hash,
            nonce: 0,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.height.to_string());
        hasher.update(&self.prev_hash);
        hasher.update(self.nonce.to_string());
        hex::encode(hasher.finalize())
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push_front(transaction);
        self.hash = self.calculate_hash()
    }

    pub fn get_transaction(&self, txid: &str) -> Option<&Transaction> {
        self.transactions.iter().find(|tx| tx.txid == txid)
    }
}


#[derive(Clone)]
pub struct Transaction {
    pub inputs: List<TxIn>,
    pub outputs: List<TxOut>,
    pub txid: String,
}

impl Transaction {
    pub fn new(inputs: List<TxIn>, outputs: List<TxOut>) -> Self {
        let mut tx = Transaction {
            txid: String::new(),
            inputs,
            outputs,
        };
        tx.txid = tx.calculate_txid();
        tx
    }

    pub fn calculate_txid(&self) -> String {
        let mut hasher = Sha256::new();
        for input in self.inputs.iter() {
            hasher.update(&input.prev_txid);
            hasher.update(input.out.to_string());
            hasher.update(&input.signature);
        }
        for output in self.outputs.iter() {
            hasher.update(&output.public_address);
            hasher.update(output.satoshis.to_string());
        }
        hex::encode(hasher.finalize())
    }
}

#[derive(Clone)]
pub struct TxIn {
    pub prev_txid: String,
    pub out: usize,
    pub signature: String, // to spend the output
}

impl TxIn {
    pub fn new(prev_txid: String, out: usize, signature: String) -> Self {
        TxIn {
            prev_txid,
            out,
            signature,
        }
    }
}


#[derive(Clone)]
pub struct TxOut {
    pub public_address: String,
    pub satoshis: u64,
}

impl TxOut {
    pub fn new(public_address: String, satoshis: u64) -> Self {
        TxOut {
            public_address,
            satoshis,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_txin() {
        let txin = TxIn::new(String::from("prev_output"), 0, String::from("signature"));
        assert_eq!(txin.prev_txid, "prev_output");
        assert_eq!(txin.out, 0);
        assert_eq!(txin.signature, "signature");
    }

    #[test]
    fn test_txout() {
        let txout = TxOut::new(String::from("public_address"), 100);
        assert_eq!(txout.public_address, "public_address");
        assert_eq!(txout.satoshis, 100);
    }

    #[test]
    fn test_transaction() {
        let txin = TxIn::new(String::from("prev_output"), 0, String::from("signature"));
        let txout = TxOut::new(String::from("public_address"), 100);
        let tx = Transaction::new(vec![txin].into_iter().collect(), vec![txout].into_iter().collect());
        assert!(!tx.txid.is_empty());
    }

    #[test]
    fn test_block() {
        let mut block = Block::new(String::from("prev_hash"));
        assert_eq!(block.prev_hash, "prev_hash");
        assert!(block.hash.is_empty());
        assert_eq!(block.transactions.len(), 0);
    }

    #[test]
    fn test_block_add_transaction() {
        let mut block = Block::new(String::from("prev_hash"));
        let txin = TxIn::new(String::from("prev_output"), 0, String::from("signature"));
        let txout = TxOut::new(String::from("public_address"), 100);
        let tx = Transaction::new(vec![txin].into_iter().collect(), vec![txout].into_iter().collect());
        block.add_transaction(tx);
        assert_eq!(block.transactions.len(), 1);
    }

    #[test]
    fn test_blockchain() {
        let blockchain = BlockChain::new();
        assert_eq!(blockchain.get_block_count(), 0);
    }

    #[test]
    fn test_blockchain_add_multiple_blocks() {
        let mut blockchain = BlockChain::new();
        let block1 = Block::new(String::from("prev_hash1"));
        let block2 = Block::new(String::from("prev_hash2"));
        blockchain.add_block(block1);
        blockchain.add_block(block2);
        assert_eq!(blockchain.get_block_count(), 2);
    }
}
