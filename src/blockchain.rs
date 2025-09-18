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