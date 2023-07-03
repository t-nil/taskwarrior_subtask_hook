use std::fmt::Error;

use task_hookrs::task::Task;

use task_hookrs::uda::*;
use uuid::Uuid;

// remember to add to UDAs if you want to modify manually
const SUBTASK_UDA: &str = "objective"; // parent is already in use

fn main() {
    println!("Hello, world!");
    let mut t: Task;
    t.due();
}

/*
struct Subtask {
    t: Task,
    fn parent(&self) -> Uuid;

}

...
impl Task for
 */

fn subtask(t: &Task) -> Result<Uuid, String> {
    match t.uda().get(SUBTASK_UDA) {
        Some(UDAValue::Str(uuid)) => {
            if uuid.as_str() == "" {
                Err("UDA is set but empty".to_owned())
            } else {
                Uuid::try_parse(uuid).or_else(|err| Err(err.to_string()))
            }
        }
        Some(_) => Err("wrong UDA type".to_owned()),
        _ => Err("uda not set".to_owned()),
    }
}

#[test]
fn test_subtask() {
    task::bu
}

fn on_modify<I>(ts: I) -> I
where
    I: Iterator<Item = Task>,
{
    //ts.filter(|t| )
}
