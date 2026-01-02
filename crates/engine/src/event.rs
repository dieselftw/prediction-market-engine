use core::error;
use types::event::{Event, EventData, EventStatus};
use bson::oid::ObjectId;

pub async fn create_new_event(
    event_data: EventData
) -> Result<Event, Box<dyn error::Error>> {
    let new_event: Event = Event {
        id: ObjectId::new(),
        title: event_data.title,
        description: event_data.description,
        start_time: event_data.start_time,
        end_time: event_data.end_time,
        outcomes: event_data.outcomes,
        resolved_outcome: None,
        status: EventStatus::Pending,
        metadata: event_data.metadata,
    };
    println!("New event: {:?}", new_event);

    Ok(new_event)
}

// pub async fn get_all_events() -> Result<Vec<Event>, Box<dyn error::Error>> {
//     let events = Event::find().await?;
//     Ok(events)
// }