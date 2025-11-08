use es_core::*;

// Example Events
#[derive(Debug, Clone, PartialEq)]
pub struct UserCreated {
    pub user_id: String,
    pub email: String,
}
event!(UserCreated);

#[derive(Debug, Clone, PartialEq)]
pub struct UserDeleted {
    pub user_id: String,
}
event!(UserDeleted);

#[derive(Debug, Clone, PartialEq)]
pub struct UserUpdated {
    pub user_id: String,
    pub email: String,
}
event!(UserUpdated);

#[derive(Debug, Clone, PartialEq)]
pub struct Transferred {
    pub transfer_id: String,
    pub amount: u64,
}
event!(Transferred);

#[derive(Debug, Clone, PartialEq)]
pub struct TransferFailed {
    pub transfer_id: String,
    pub reason: String,
}
event!(TransferFailed);

// Event Sets
#[derive(Debug, Clone, AwaitedSet)]
pub enum AllUserEvents {
    UserCreated(UserCreated),
    UserDeleted(UserDeleted),
    UserUpdated(UserUpdated),
}

#[derive(Debug, Clone, AwaitedSet)]
pub enum UserWriteEvents {
    UserCreated(UserCreated),
    UserDeleted(UserDeleted),
}

#[derive(Debug, Clone, AwaitedSet)]
pub enum TransferResponse {
    Transferred(Transferred),
    Failed(TransferFailed),
}

#[test]
fn test_event_into_set() {
    let created = UserCreated {
        user_id: "123".to_string(),
        email: "user@example.com".to_string(),
    };

    // Wrap event into set using HasEventSet
    let event_set: UserWriteEvents = created.clone().into_set();

    // Verify it's the right variant
    match event_set {
        UserWriteEvents::UserCreated(e) => {
            assert_eq!(e.user_id, "123");
            assert_eq!(e.email, "user@example.com");
        }
        _ => panic!("Wrong variant"),
    }
}

#[test]
fn test_event_try_from_set() {
    let event_set = UserWriteEvents::UserCreated(UserCreated {
        user_id: "123".to_string(),
        email: "user@example.com".to_string(),
    });

    // Extract event from set using HasEventSet
    let created = UserCreated::try_from_set(event_set).expect("Should extract UserCreated");
    assert_eq!(created.user_id, "123");
    assert_eq!(created.email, "user@example.com");
}

#[test]
fn test_event_try_from_set_wrong_variant() {
    let event_set = UserWriteEvents::UserDeleted(UserDeleted {
        user_id: "123".to_string(),
    });

    // Try to extract wrong event type
    let result = UserCreated::try_from_set(event_set);
    assert!(result.is_err());
}

#[test]
fn test_awaited_set_event_name() {
    let event_set = UserWriteEvents::UserCreated(UserCreated {
        user_id: "123".to_string(),
        email: "user@example.com".to_string(),
    });

    assert_eq!(event_set.event_name(), UserCreated::NAME);
}

#[test]
fn test_awaited_set_awaitable_events() {
    let names = UserWriteEvents::AWAITABLE_EVENTS;
    assert_eq!(names.len(), 2);
    assert!(names.contains(&UserCreated::NAME));
    assert!(names.contains(&UserDeleted::NAME));
}

#[test]
fn test_awaited_set_const_name() {
    assert_eq!(TransferResponse::NAME.as_str(), "TransferResponse");
}

#[test]
fn test_envelope_conversion() {
    let created = UserCreated {
        user_id: "123".to_string(),
        email: "user@example.com".to_string(),
    };

    // Create envelope manually (simulating receiving an event)
    let envelope = DynEventEnvelope {
        name: UserCreated::NAME,
        event: Box::new(created),
    };

    // Convert from envelope to AwaitedSet
    let recovered = UserWriteEvents::try_from_envelope(envelope)
        .expect("Should convert from envelope");

    let created_back: UserCreated = recovered.try_into_event()
        .expect("Should extract event");

    assert_eq!(created_back.user_id, "123");
    assert_eq!(created_back.email, "user@example.com");
}

#[test]
fn test_envelope_wrong_event() {
    let created = UserCreated {
        user_id: "123".to_string(),
        email: "user@example.com".to_string(),
    };

    let envelope = DynEventEnvelope {
        name: UserCreated::NAME,
        event: Box::new(created),
    };

    // Try to convert to TransferResponse (wrong set)
    let result = TransferResponse::try_from_envelope(envelope);
    assert!(result.is_err());
}
