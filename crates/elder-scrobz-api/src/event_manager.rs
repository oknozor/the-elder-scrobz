use elder_scrobz_model::events::ScrobzEvent;
use std::{collections::BTreeMap, time::Instant};
use tokio::sync::broadcast;
use tracing::debug;

#[derive(Debug)]
pub struct EventManager {
    cache: BTreeMap<Instant, ScrobzEvent>,
    sse_sender: broadcast::Sender<ScrobzEvent>,
}

impl EventManager {
    pub fn new(sse_sender: broadcast::Sender<ScrobzEvent>) -> Self {
        Self {
            cache: BTreeMap::new(),
            sse_sender,
        }
    }

    pub fn sse_sender(&self) -> &broadcast::Sender<ScrobzEvent> {
        &self.sse_sender
    }

    pub fn handle_connect(&mut self) {
        self.purge_now_playing();
        debug!("Sending cached events");
        for event in self.cache.values() {
            let _ = self.sse_sender.send(event.clone());
        }
    }

    pub fn push(&mut self, event: ScrobzEvent) {
        debug!("Pushing event {event:?} to cache and sending to SSE");
        match &event {
            ScrobzEvent::PlayingNow { user, .. } => {
                self.cache.retain(|_, e| {
                    !matches!(
                        e, ScrobzEvent::PlayingNow {
                            user: existing_user,
                            ..
                        } if existing_user == user
                    )
                });
            }
        }

        let _ = self.sse_sender.send(event.clone());
        self.cache.insert(Instant::now(), event);
    }

    fn purge_now_playing(&mut self) {
        self.cache.retain(|insert_timestamp, event| match event {
            ScrobzEvent::PlayingNow { track_duration, .. } => {
                let elapsed_time = insert_timestamp.elapsed();
                elapsed_time.as_millis() <= *track_duration as u128
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_cache_new() {
        let (tx, _rx) = broadcast::channel(100);
        let manager = EventManager::new(tx);
        assert_eq!(manager.cache.len(), 0);
    }

    #[test]
    fn test_push_playing_now_event() {
        let (tx, _rx) = broadcast::channel(100);
        let mut manager = EventManager::new(tx);
        let event = ScrobzEvent::PlayingNow {
            user: "test_user".to_string(),
            track_duration: 180000, // 3 minutes
            track_name: "test".to_string(),
            artist: "test".to_string(),
            album: "test".to_string(),
            cover_art_url: Some("test.com".to_string()),
        };

        manager.push(event);
        assert_eq!(manager.cache.len(), 1);
    }

    #[test]
    fn test_push_replaces_existing_playing_now_for_same_user() {
        let (tx, _rx) = broadcast::channel(100);
        let mut manager = EventManager::new(tx);

        let event1 = ScrobzEvent::PlayingNow {
            user: "test_user".to_string(),
            track_duration: 180000,
            track_name: "test".to_string(),
            artist: "test".to_string(),
            album: "test".to_string(),
            cover_art_url: Some("test.com".to_string()),
        };

        let event2 = ScrobzEvent::PlayingNow {
            user: "test_user".to_string(),
            track_duration: 240000,
            track_name: "test".to_string(),
            artist: "test".to_string(),
            album: "test".to_string(),
            cover_art_url: Some("test.com".to_string()),
        };

        manager.push(event1);
        assert_eq!(manager.cache.len(), 1);

        manager.push(event2);
        assert_eq!(manager.cache.len(), 1);
    }

    #[test]
    fn test_push_allows_different_users() {
        let (tx, _rx) = broadcast::channel(100);
        let mut manager = EventManager::new(tx);

        let event1 = ScrobzEvent::PlayingNow {
            user: "user1".to_string(),
            track_duration: 180000,
            track_name: "test".to_string(),
            artist: "test".to_string(),
            album: "test".to_string(),
            cover_art_url: Some("test.com".to_string()),
        };

        let event2 = ScrobzEvent::PlayingNow {
            user: "user2".to_string(),
            track_duration: 240000,
            track_name: "test".to_string(),
            artist: "test".to_string(),
            album: "test".to_string(),
            cover_art_url: Some("test.com".to_string()),
        };

        manager.push(event1);
        manager.push(event2);
        assert_eq!(manager.cache.len(), 2); // Should have both events
    }

    #[test]
    fn test_purge_now_playing_removes_expired_events() {
        let (tx, _rx) = broadcast::channel(100);
        let mut manager = EventManager::new(tx);

        let expired_event = ScrobzEvent::PlayingNow {
            user: "test_user".to_string(),
            track_duration: 1,
            track_name: "test".to_string(),
            artist: "test".to_string(),
            album: "test".to_string(),
            cover_art_url: Some("test.com".to_string()),
        };

        let old_instant = Instant::now() - std::time::Duration::from_secs(1);
        manager.cache.insert(old_instant, expired_event);

        assert_eq!(manager.cache.len(), 1);

        manager.purge_now_playing();
        assert_eq!(manager.cache.len(), 0);
    }
}
