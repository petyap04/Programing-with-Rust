use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Event {
    Login { user: String, timestamp: u64 },
    Logout { user: String, timestamp: u64 },
    Purchase { user: String, item: String, amount: f64, timestamp: u64 },
    Error { code: i32, message: String, timestamp: u64 },
}

#[derive(Debug, Default)]
struct EventLog {
    events: Vec<Event>,
}

impl EventLog {
    fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    fn user_spent(&self, user: &str) -> f64 {
        self.events.iter().fold(0.0, |acc, ev| match ev {
            Event::Purchase { user: u, amount, .. } if u == user => acc + *amount,
            _ => acc,
        })
    }

    fn summaries_by_type(&self) -> HashMap<String, usize>{
        let result = HashMap::new();
        for ev in &self.events {
            match ev {
                Event::Login { .. } => result.insert("Login", )
                Event::Logout { .. } => summary.logouts += 1,
                Event::Purchase { .. } => summary.purchases += 1,
                Event::Error { .. } => summary.errors += 1,
            }
        }
    }

    fn summaries_by_type(&self) -> EventSummary {
        let mut summary = EventSummary::default();
        for ev in &self.events {
            match ev {
                Event::Login { .. } => summary.logins += 1,
                Event::Logout { .. } => summary.logouts += 1,
                Event::Purchase { .. } => summary.purchases += 1,
                Event::Error { .. } => summary.errors += 1,
            }
        }
        summary
    }

    fn filter_events<'a>(&'a self, user: Option<&str>, after: Option<u64>) -> Vec<&'a Event> {
        let mut result = Vec::new();

        for ev in &self.events {
            let ts = match ev {
                Event::Login { timestamp, .. }
                | Event::Logout { timestamp, .. }
                | Event::Purchase { timestamp, .. }
                | Event::Error { timestamp, .. } => *timestamp,
            };

            if let Some(after_ts) = after {
                if ts <= after_ts {
                    continue;
                }
            }

            if let Some(u) = user {
                let matches_user = match ev {
                    Event::Login { user, .. }
                    | Event::Logout { user, .. }
                    | Event::Purchase { user, .. } => user == u,
                    Event::Error { .. } => false,
                };
                if !matches_user {
                    continue;
                }
            }

            result.push(ev);
        }

        result
    }
}

fn main(){
    let mut log = EventLog::default();
    log.add_event(Event::Login { user: "anna".into(), timestamp: 100 });
    log.add_event(Event::Purchase { user: "anna".into(), item: "ябълка".into(), amount: 2.5, timestamp: 110 });
    log.add_event(Event::Purchase { user: "ivan".into(), item: "банан".into(), amount: 1.2, timestamp: 120 });
    log.add_event(Event::Logout { user: "anna".into(), timestamp: 130 });
    log.add_event(Event::Error { code: 404, message: "Not Found".into(), timestamp: 140 });

    println!("Anna spent: {:.2}", log.user_spent("anna"));
    println!("Summary: {:?}", log.summaries_by_type());
    println!("Filtered (anna after 105): {:#?}", log.filter_events(Some("anna"), Some(105)));
}