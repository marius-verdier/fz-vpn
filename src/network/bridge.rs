use std::sync::{Arc, Mutex};

#[derive(Serialize, Debug, Clone)]
pub struct Traffics {
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
}

#[derive(Serialize)]
pub enum InfoType {
    State,
    Log,
    Traffic
}

#[derive(Serialize)]
pub struct TunInfo<T> where T: ?Sized + Serialize {
    pub info_type: InfoType,
    pub info: Box<T>
}

#[derive(Clone)]
pub struct TunInfoBridge {
    listener: Option<Arc<Mutex<dyn FnMut(&str) + 'static + Send + Sync>>>
}

impl TunInfoBridge {

    pub fn create() -> Self {
        TunInfoBridge {
            listener: None
        }
    }

    pub fn set_listener(&mut self, listener: impl FnMut(&str) + 'static + Send + Sync) {
        self.listener = Some(Arc::new(Mutex::new(listener)));
    }

    pub fn has_listener(&self) -> bool {
        self.listener.is_some()
    }

    pub fn log<T>(&self, info: TunInfo<T>) where T: ?Sized + Serialize {
        if let Some(listener) = &self.listener {
            let mut listener = listener.lock().unwrap();
            if let Ok(json) = serde_json::to_string(&info) {
                listener(json.as_str());
            } else {
                println!("Error during serialization of TunInfo");
            }
        }
    }
}