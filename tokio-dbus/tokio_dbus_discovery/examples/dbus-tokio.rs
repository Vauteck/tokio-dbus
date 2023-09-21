use dbus_tokio::connection;
use dbus::nonblock::Proxy;
use std::time::Duration;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Connect to the D-Bus session bus (this is blocking, unfortunately).
    let (resource, conn) = connection::new_system_sync()?;

    // The resource is a task that should be spawned onto a tokio compatible
    // reactor ASAP. If the resource ever finishes, you lost connection to D-Bus.
    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    // Make a "proxy object" that contains the destination and path of our method call.
    let proxy = Proxy::new("org.freedesktop.DBus", "/", Duration::from_secs(5), conn);

    // Call the method and await a response. See the argument guide for details about
    // how to send and receive arguments to the method.
    let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ()).await?;

    // Print all the names.
    for name in names { println!("{}", name); }

    Ok(())
}
