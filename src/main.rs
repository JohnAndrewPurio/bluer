#[tokio::main]
async fn main() -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;

    adapter.set_powered(true).await?;

    let adapter_addr = adapter.address().await?;
    let adapter_addr_type = adapter.address_type().await?;

    // Advertising is necessary for device to be connectable.
    println!(
        "Advertising in Bluetooth adapter {} with {} address {}",
        adapter.name(),
        &adapter_addr_type,
        &adapter_addr
    );

    Ok(())
}