use std::process;

#[cfg(windows)]
pub fn handle() {
    use tokio::signal::windows::{ctrl_c, ctrl_shutdown};

    tokio::spawn(async {
        let mut ctrl_shutdown_signal = ctrl_shutdown().expect("failed to listen for ctrl_shutdown_signal");
        ctrl_shutdown_signal.recv().await;
        tracing::info!("got CTRL-SHUTDOWN. airdo is exiting");
        process::exit(0);
    });

    tokio::spawn(async {
        let mut ctrl_c_signal = ctrl_c().expect("failed to listen for ctrl_c_signal");
        ctrl_c_signal.recv().await;
        tracing::info!("got CTRL-C. airdo is exiting");
        process::exit(0);
    });
}

#[cfg(unix)]
pub fn handle() {
    use tokio::signal::ctrl_c;
    use tokio::signal::unix::{signal, SignalKind};

    tokio::spawn(async {
        let mut sig = signal(SignalKind::terminate()).expect("failed to listen for SIGTERM signal");
        sig.recv().await;
        tracing::info!("got SIGTERM signal. airdo is exiting");
        process::exit(0);
    });
    
    tokio::spawn(async {
        ctrl_c().await.expect("failed to listen for ctrl_c event");
        tracing::info!("got CTRL-C. airdo is exiting");
        process::exit(0);
    });
}
