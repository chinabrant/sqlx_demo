
pub struct JsonResult {
    pub code: i32,
    pub msg: String,
    pub body: Serialize,
}