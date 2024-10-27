use futures_util::stream::StreamExt;
use zbus::{proxy, Connection};

#[proxy(
    default_service = "org.kde.KWin",
    default_path = "/org/freedesktop/ScreenSaver",
    interface = "org.freedesktop.ScreenSaver"
)]
trait ScreenSaverManager {
    // Defines signature for D-Bus signal named `JobNew`
    #[zbus(signal)]
    fn active_changed(&self, active: bool) -> zbus::Result<()>;
}

pub async fn watch_screen_lock() -> zbus::Result<()> {
    println!("Waiting for information about screen lock!!!");
    let connection = Connection::session().await?;
    // `Systemd1ManagerProxy` is generated from `Systemd1Manager` trait
    let systemd_proxy = ScreenSaverManagerProxy::new(&connection).await?;
    // Method `receive_job_new` is generated from `job_new` signal
    let mut active_changed_stream = systemd_proxy.receive_active_changed().await?;

    while let Some(msg) = active_changed_stream.next().await {
        // struct `JobNewArgs` is generated from `job_new` signal function arguments
        let args: ActiveChangedArgs = msg.args().expect("Error parsing message");

        println!("ScreenLock activated: {}", args.active);
    }

    panic!("Stream ended unexpectedly");
}
