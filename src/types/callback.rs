use crate::payloads::Payload;

pub struct MyCallbackFormat {
    payload: Payload,
}

impl MyCallbackFormat {
    pub fn new(payload: Payload) -> Self {
        Self { payload }
    }

    pub fn callback_text(&self) -> String {
        serde_json::to_string(&self.payload).unwrap()
    }
}
