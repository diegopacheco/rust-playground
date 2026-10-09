use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use summer::plugin::service::Service;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct NewTask {
    pub title: String,
}

#[derive(Default)]
pub struct TaskStore {
    last_id: u64,
    tasks: Vec<Task>,
}

#[derive(Clone, Default, Service)]
pub struct TaskService {
    #[inject(func = Default::default())]
    store: Arc<Mutex<TaskStore>>,
}

impl TaskService {
    pub fn list(&self) -> Vec<Task> {
        self.store.lock().unwrap().tasks.clone()
    }

    pub fn find(&self, id: u64) -> Option<Task> {
        self.list().into_iter().find(|t| t.id == id)
    }

    pub fn create(&self, title: &str) -> Option<Task> {
        let title = title.trim();
        if title.is_empty() {
            return None;
        }
        let mut store = self.store.lock().unwrap();
        store.last_id += 1;
        let task = Task {
            id: store.last_id,
            title: title.to_string(),
            done: false,
        };
        store.tasks.push(task.clone());
        Some(task)
    }

    pub fn complete(&self, id: u64) -> Option<Task> {
        let mut store = self.store.lock().unwrap();
        let task = store.tasks.iter_mut().find(|t| t.id == id)?;
        task.done = true;
        Some(task.clone())
    }

    pub fn remove(&self, id: u64) -> bool {
        let mut store = self.store.lock().unwrap();
        let before = store.tasks.len();
        store.tasks.retain(|t| t.id != id);
        store.tasks.len() < before
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn created_tasks_start_pending_with_trimmed_title() {
        let service = TaskService::default();
        let task = service.create("  write poc  ").unwrap();
        assert_eq!(task.title, "write poc");
        assert!(!task.done);
        assert_eq!(service.list(), vec![task]);
    }

    #[test]
    fn blank_titles_are_rejected_and_not_stored() {
        let service = TaskService::default();
        assert!(service.create("   ").is_none());
        assert!(service.list().is_empty());
    }

    #[test]
    fn ids_are_never_reused_after_a_removal() {
        let service = TaskService::default();
        let first = service.create("first").unwrap();
        assert!(service.remove(first.id));
        let second = service.create("second").unwrap();
        assert_ne!(first.id, second.id);
    }

    #[test]
    fn completing_marks_only_the_requested_task() {
        let service = TaskService::default();
        let first = service.create("first").unwrap();
        let second = service.create("second").unwrap();
        assert!(service.complete(first.id).unwrap().done);
        assert!(!service.find(second.id).unwrap().done);
    }

    #[test]
    fn unknown_ids_are_reported_as_missing() {
        let service = TaskService::default();
        assert!(service.find(42).is_none());
        assert!(service.complete(42).is_none());
        assert!(!service.remove(42));
    }

    #[test]
    fn clones_share_the_same_store() {
        let service = TaskService::default();
        let clone = service.clone();
        service.create("shared").unwrap();
        assert_eq!(clone.list().len(), 1);
    }
}
