use futures::executor::block_on;

fn main() {
    let m = async_main();
    block_on(m);
}

async fn async_main() {
    let f1 = learn_and_sing_song();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}

async fn learn_and_sing_song() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let s = learn_song().await;
    sing_song(s).await;
}

struct Song;
async fn learn_song() -> Song {
    println!("learn song");
    Song {}
}
async fn sing_song(_song: Song) {
    println!("sing song");
}
async fn dance() {
    println!("dance");
}
