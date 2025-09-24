pub fn create_blockchain(genesis_address: &str) -> Blockchain{
    let db = sled::open(current_dir().unwrap().join("data")).unwrap();
    let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();
    let data = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
    let tip_hash;

    if data.is_none(){
        let coinbase = Transaction::new_coinbase_tx(genesis_address);
        let genesis = Block::generate_genesis_block(&coinbase_tx);
        self::update_blocks_tree(&blocks_tree, &block);
        tip_hash = String::from(genesis.get_hash());
    else{
        tip_hash = String::from_utf8(data.unwrap().to_vec()).unwrap();
    }
    Blockchain{
        tip_hash: Arc::new(RwLock::new(tip_hash)),
        db,
    }
    }
}
pub fn new_blockchain() -> Blockchain{
    let db = sled::open(current_dir().unwrap().join("data")).unwrap();
    let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();
    let tip_bytes = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap().expect("Not exit a blockchain, create one first.");
    
    blockchain{
        tip_hash: Arc::new(RwLock::new(tip_hash)),
        db,
    }  
}

pub fn get_db(&self) -> &Db{
    &self.db
}

pub fn get_tip_hash(&self) -> String{
    self.tip_hash.read().unwrap().to_string()
}

pub fn set_tip_hash(&self, new_tip_hash: &str){
    let mut tip_hash = self.tip_hash.write().unwrap();
    *tip_hash = String::from(new_tip_hash); //*el * es para acceder al contenido protegido dentro de la variable por rwlockguard
}

pub fn iterator(&self) -> BlockchainIterator{
    BlockchainIterator::new(self.get_tip_hash(),self.db.clone())
}

pub struct BlockchainIterator{
    db: Db,
    current_hash: String,
}

impl BlockchainIterator{
    fn new(tip_hash: String, db: Db) -> BlockchainIterator{
        
    }

    pub fn next(&mut self) -> Option<Block>{
        
    }

}

pub fn mine_block(&self, transactions: &[Transaction]) -> Block {
    for trasaction in transactions {
    if trasaction.verify(self) == false {
    panic!("ERROR: Invalid transaction")
    }
    }
    let best_height = self.get_best_height();
    let block = Block::new_block(self.get_tip_hash(), transactions, best_height + 1);
    let block_hash = block.get_hash();
    let blocks_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
    Self::update_blocks_tree(&blocks_tree, &block);
    self.set_tip_hash(block_hash);
    block
}