use {
    crate::{parallel_bigtable_client::BufferedBigtableClient},
    log::*,
    solana_bigtable_geyser_models::models::slots,
    solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPluginError,
    std::time::SystemTime,
};

impl BufferedBigtableClient {
    /// Update or insert a single account
    pub async fn update_slot(
        &mut self,
        slot: u64,
        parent: Option<u64>,
        status: &str,
    ) -> Result<(), GeyserPluginError> {
        let slot_cells = vec![(
            slot.to_string(),
            slots::Slot {
                slot,
                parent,
                status: status.to_string(),
                updated_on: Some(slots::UnixTimestamp {
                    timestamp: SystemTime::now().elapsed().unwrap().as_secs() as i64,
                }),
            },
        )];

        let client = self.client.lock().unwrap();
        let result = client
            .client
            .put_protobuf_cells_with_retry::<slots::Slot>("slot", &slot_cells, true)
            .await;
        match result {
            Ok(_size) => Ok(()),
            Err(err) => {
                error!("Error persisting into the database: {}", err);
                Err(GeyserPluginError::Custom(Box::new(err)))
            }
        }
    }
}
