use item::Item;

#[derive(Debug, Serialize, Deserialize)]
pub struct AlfredResult {
    pub items: Vec<Item>,
}
