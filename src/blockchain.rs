pub struct App {
    pub blocks: Vec,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub timestamp: u64,
    pub nonce: u64,
    pub hash: String,
    pub previous_hash: String,
    pub data: String,
}

impl App {
    fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            timestamp: Utc::now().timestamp(),
            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
            previous_hash: String::from("genesis"),
            data: String::from("genesis"),
            nonce: 1234,
        };
        self.blocks.push(genesis_block);
    }

    fn try_add_block(&mut self, block: Block) {
        let latest_block = self.blocks.last();
    }
}
