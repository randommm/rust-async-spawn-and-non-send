use rand::{thread_rng, Rng};
use std::rc::Rc;
use tokio::task::JoinSet;

fn get_random_number() -> u64 {
    thread_rng().gen_range(100..250)
}

async fn task(id: i32) {
    println!("Task {} started", id);
    let tid = std::thread::current().id();

    // async sleep, task yields thread back to async runtime
    tokio::time::sleep(std::time::Duration::from_millis(get_random_number())).await;

    // blocking sleep, task does **not** yield thread back to async runtime
    std::thread::sleep(std::time::Duration::from_millis(get_random_number()));

    println!(
        "Task {} started on {:?} and completed on {:?}",
        id,
        tid,
        std::thread::current().id(),
    );
}

async fn hold_non_send() {
    let rc = Rc::new(1);
    // uncomment to cause a compilation error
    // tokio::time::sleep(std::time::Duration::from_millis(get_random_number())).await;
    let _ = rc.clone();
}

// #[tokio::main(flavor = "current_thread")]
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("Started main function on {:?}", std::thread::current().id(),);

    let mut set = JoinSet::new();
    for i in 0..10 {
        set.spawn(task(i));
    }
    while set.join_next().await.is_some() {}

    tokio::spawn(hold_non_send()).await.unwrap();
}
