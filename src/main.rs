use std::rc::Rc;

async fn task_with_blocking_sleeps(id: i32) {
    println!("Task {} started", id);
    let tid = std::thread::current().id();
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!(
        "Blocking sleep task {} started on {:?} and completed on {:?}",
        id,
        tid,
        std::thread::current().id(),
    );
}

async fn task(id: i32) {
    println!("Task {} started", id);
    let tid = std::thread::current().id();
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    println!(
        "Async sleep task {} started on {:?} and completed on {:?}",
        id,
        tid,
        std::thread::current().id(),
    );
}

async fn hold_non_send() {
    let rc = Rc::new(1);
    // uncomment to cause compilation error
    // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("rc: {:?}", rc);
}

// #[tokio::main(flavor = "current_thread")]
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("Started main function on {:?}", std::thread::current().id(),);
    for i in 0..10 {
        tokio::spawn(task(i));
    }
    for i in 10..20 {
        tokio::spawn(task_with_blocking_sleeps(i));
    }

    tokio::spawn(hold_non_send()).await;
    tokio::time::sleep(std::time::Duration::from_secs(10000)).await;
}
