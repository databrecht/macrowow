pub use es_derive2::AwaitedSet;
pub use downcast_rs::{impl_downcast, Downcast};
pub use eyre::Report;
use nutype::nutype;

// Module declarations
pub mod event;
pub mod aggregate;
pub mod subscriber;
pub mod projector;

// ============================================================================
// Core Traits
// ============================================================================

/// Base trait for all events - provides runtime type information
pub trait DynEvent: Downcast + Send + Sync {
    fn name(&self) -> EventName<'static>;
}

impl_downcast!(DynEvent);

/// Concrete event type with compile-time name
pub trait Event: DynEvent + Sized + 'static {
    const NAME: EventName<'static>;
}

/// Marker trait indicating an event expects a specific awaited set when injected.
///
/// This trait is implemented via the `InjectableEvent` derive macro for events
/// that specify which awaited sets they expect via `#[es(awaits(...))]`.
pub trait ExpectsAwaitedSet<S: AwaitedSet>: Event {}

/// Event with idempotency support
pub trait Idempotent {
    fn get_idempotency_key(&self) -> Result<IdempotencyKey, IdempotencyKeyError>;
}

/// Event with correlation support
pub trait Correlated {
    fn get_correlation_id(&self) -> Result<CorrelationId, CorrelationIdError>;
    fn expected_correlation_group_status(&self) -> ExpectedCorrelationGroupStatus;
}

/// Type-erased event envelope
pub struct DynEventEnvelope {
    pub name: EventName<'static>,
    pub event: Box<dyn DynEvent>,
}

impl DynEventEnvelope {
    /// Helper to downcast the event to a concrete type
    pub fn downcast<T: Event>(self) -> Result<T, Box<dyn DynEvent>> {
        self.event.downcast::<T>().map(|boxed| *boxed)
    }
}

/// Bidirectional relationship: Event E can be in AwaitedSet S
pub trait HasEventSet<S: AwaitedSet>: Event {
    /// Wrap this event into the AwaitedSet
    fn into_set(self) -> S;

    /// Try to extract this event from the AwaitedSet
    fn try_from_set(set: S) -> Result<Self, S>;
}

/// A set of events that can be awaited as a response
pub trait AwaitedSet: Sized + Send {
    const NAME: EventSetName;
    const AWAITABLE_EVENTS: &'static [EventName<'static>];

    fn event_name(&self) -> EventName<'static>;

    /// Convert from type-erased envelope (downcast + wrap in variant)
    fn try_from_envelope(envelope: DynEventEnvelope) -> Result<Self, Report>;

    /// Try to extract a specific event type from this set
    fn try_into_event<E>(self) -> Result<E, Self>
    where
        E: Event + HasEventSet<Self>,
    {
        E::try_from_set(self)
    }

    /// Wrap a concrete event into this set
    fn from_event<E>(event: E) -> Self
    where
        E: Event + HasEventSet<Self>,
    {
        event.into_set()
    }
}

// ============================================================================
// Helper Types
// ============================================================================

#[nutype(
    validate(not_empty, len_char_max = 1024),
    derive(Debug, PartialEq, Clone, Eq, Hash, Serialize, Deserialize, Display)
)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    pub fn to_base64(&self) -> String {
        use base64::{engine::general_purpose, Engine as _};
        general_purpose::STANDARD.encode(self.clone().into_inner())
    }
}

#[nutype(
    validate(not_empty, len_char_max = 1024),
    derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize, Display, AsRef)
)]
pub struct CorrelationId(String);

impl CorrelationId {
    pub fn generate() -> Result<Self, Report> {
        use uuid::Uuid;
        Self::try_new(Uuid::new_v4().to_string())
            .map_err(|e| eyre::eyre!("Failed validation when generating CorrelationId: {}", e))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpectedCorrelationGroupStatus {
    /// Correlation group should be new (first event in the group)
    New,
    /// Correlation group should already exist
    Exists,
    /// Correlation group can be new or existing
    Any,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventName<'a>(&'a str);

impl<'a> EventName<'a> {
    pub const fn new(name: &'a str) -> Self {
        Self(name)
    }

    pub fn as_str(&self) -> &str {
        self.0
    }
}

#[nutype(
    const_fn,
    derive(
        Debug, PartialEq, Clone, Copy, Eq, Display, AsRef, Deref, Hash, PartialOrd, Ord, Serialize
    )
)]
pub struct EventSetName(&'static str);

impl EventSetName {
    pub fn as_str(&self) -> &str {
        self.into_inner()
    }
}

impl std::borrow::Borrow<str> for EventSetName {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl From<EventSetName> for serde_json::Value {
    fn from(event_set_name: EventSetName) -> Self {
        event_set_name.into_inner().into()
    }
}

// ============================================================================
// Handler Output Trait
// ============================================================================

/// Trait for handler output types
pub trait HandlerOutput {}

