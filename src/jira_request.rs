pub trait JIRARequest {
    fn base_url(&self) -> &String;
    fn path(&self) -> &String;
    fn method(&self) -> &String;
    fn parameters(&self) -> &String;
}