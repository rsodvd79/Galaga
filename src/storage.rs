const HIGH_SCORE: &str = "galaga_rust_high_score_v1";

pub fn load(storage: Option<&dyn eframe::Storage>) -> i32 {
    storage
        .and_then(|s| s.get_string(HIGH_SCORE))
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0)
        .max(0)
}

pub fn save<S: eframe::Storage + ?Sized>(storage: Option<&mut S>, score: i32) {
    if let Some(storage) = storage {
        let previous = storage
            .get_string(HIGH_SCORE)
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);
        let score = score.max(previous).max(0);
        storage.set_string(HIGH_SCORE, score.to_string());
        storage.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Default)]
    struct Memory(Option<String>, usize);
    impl eframe::Storage for Memory {
        fn get_string(&self, _: &str) -> Option<String> {
            self.0.clone()
        }
        fn set_string(&mut self, _: &str, value: String) {
            self.0 = Some(value);
        }
        fn remove_string(&mut self, _: &str) {
            self.0 = None;
        }
        fn flush(&mut self) {
            self.1 += 1;
        }
    }
    #[test]
    fn missing_corrupt_or_negative_record_starts_at_zero() {
        for record in [None, Some("broken".into()), Some("-2".into())] {
            assert_eq!(load(Some(&Memory(record, 0))), 0);
        }
        assert_eq!(load(None), 0);
    }
    #[test]
    fn saving_flushes_immediately_and_never_lowers_the_record() {
        let mut memory = Memory(Some("150".into()), 0);
        save(Some(&mut memory), 100);
        assert_eq!(load(Some(&memory)), 150);
        save(Some(&mut memory), 200);
        assert_eq!((load(Some(&memory)), memory.1), (200, 2));
    }
}
