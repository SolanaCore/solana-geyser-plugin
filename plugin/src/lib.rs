use agave_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, ReplicaAccountInfoVersions, ReplicaBlockInfoVersions, ReplicaEntryInfoVersions,
    ReplicaTransactionInfoVersions, Result as PluginResult, SlotStatus,
};
use solana_program::{clock::Slot, pubkey::Pubkey};
use solana_sdk::{message::SanitizedMessage, signature::Signature};
use std::sync::Arc;

use serde::Deserialize;

pub mod utils;
pub mod redis;
use redis::*;
use utils::read_config;

/// JSON config file structure
// We use deserialize to convert JSON into this struct
#[derive(Debug, Deserialize)]
pub struct Config {
    redis_url: String,
    target_program_ids: Vec<String>,
}

/// Main plugin struct
#[derive(Default, Debug)]
pub struct Plugin {
    redis: Option<Arc<RedisManager>>,
    target_programs: Vec<Pubkey>,
}

impl Plugin {
    /// Handle transaction messages and publish if matching program is found
    fn handle_transaction_message(
        &self,
        message: &SanitizedMessage,
        slot: Slot,
        signature: &Signature,
    ) {
        // List of the account keys in the transaction message
        let account_keys = message.account_keys();

        // Check if any of the account keys match the target programs
        let hits_program = account_keys
            .iter()
            .any(|key| self.target_programs.contains(key));

        // If a target program is hit, log the transaction and publish to Redis
        if hits_program {
            println!("✅ Transaction at slot {} matches target program", slot);

            if let Some(redis) = &self.redis {
                let payload = format!("Slot: {}, Signature: {:?}", slot, signature);
                if let Err(err) = redis.publish("program_transactions", &payload) {
                    eprintln!("❌ Failed to publish to Redis: {}", err);
                }
            }
        } else {
            println!("ℹ Transaction at slot {} does not match target program", slot);
        }
    }
}

impl GeyserPlugin for Plugin {
    fn name(&self) -> &'static str {
        "plugin"
    }

    fn on_load(&mut self, _config_file: &str, _is_reload: bool) -> PluginResult<()> {
        // Load JSON config
        let config = read_config("config.json").expect("Failed to read config");

        self.target_programs = config
            .target_program_ids
            .iter()
            .map(|id| id.parse::<Pubkey>().unwrap())
            .collect();

        self.redis = Some(Arc::new(RedisManager::new(&config.redis_url).unwrap()));

        println!(
            "🚀 Plugin loaded with {} target programs",
            self.target_programs.len()
        );

        Ok(())
    }

    fn on_unload(&mut self) {
        println!("🛑 Plugin unloaded. Cleaning up resources...");
        // TODO: Add any necessary cleanup logic, like closing Redis connections
    }

    fn notify_end_of_startup(&self) -> PluginResult<()> {
        println!("✅ End of startup notification received. Plugin is now active.");
        Ok(())
    }

    fn update_account(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: Slot,
        is_startup: bool,
    ) -> PluginResult<()> {
        match account {
            // v0_0_1 and v0_0_2 are not supported...
            ReplicaAccountInfoVersions::V0_0_1(_) => {
                unreachable!("ReplicaAccountInfoVersions::V0_0_1 is not supported")
            }
            ReplicaAccountInfoVersions::V0_0_2(_) => {
                unreachable!("ReplicaAccountInfoVersions::V0_0_2 is not supported")
            }
            ReplicaAccountInfoVersions::V0_0_3(info) => {
                println!(
                    "📢 Account updated at slot {} (startup={}): lamports={}, pubkey={:#?}, owner={:#?}, data_len={}",
                    slot, is_startup, info.lamports, info.pubkey, info.owner, info.data.len()
                );
            }
        };

        Ok(())
    }

    fn update_slot_status(
        &self,
        slot: Slot,
        parent: Option<u64>,
        status: &SlotStatus,
    ) -> PluginResult<()> {
        println!(
            "⏳ Slot status updated: slot={}, parent={:?}, status={:?}",
            slot, parent.unwrap(), status
        );
        Ok(())
    }

    fn notify_transaction(
        &self,
        transaction: ReplicaTransactionInfoVersions,
        slot: Slot,
    ) -> PluginResult<()> {
        match transaction {
            ReplicaTransactionInfoVersions::V0_0_1(info) => {
                self.handle_transaction_message(info.transaction.message(), slot, info.signature);
            }
            ReplicaTransactionInfoVersions::V0_0_2(info) => {
                self.handle_transaction_message(info.transaction.message(), slot, info.signature);
            }
        }
        Ok(())
    }

    fn notify_entry(&self, entry: ReplicaEntryInfoVersions) -> PluginResult<()> {
        Ok(())
    }

    fn notify_block_metadata(&self, blockinfo: ReplicaBlockInfoVersions) -> PluginResult<()> {
        match blockinfo {
            ReplicaBlockInfoVersions::V0_0_1(info) => {
                println!(
                    "📦 Block v1: slot={}, blockhash={}",
                    info.slot, info.blockhash
                );
            }
            ReplicaBlockInfoVersions::V0_0_2(info) => {
                println!(
                    "📦 Block v2: slot={}, parent_slot={}, blockhash={}, executed_tx_count={}",
                    info.slot, info.parent_slot, info.blockhash, info.executed_transaction_count
                );
            }
            ReplicaBlockInfoVersions::V0_0_3(info) => {
                println!(
                    "📦 Block v3: slot={}, parent_slot={}, blockhash={}, executed_tx_count={}, entry_count={}",
                    info.slot, info.parent_slot, info.blockhash, info.executed_transaction_count, info.entry_count
                );
            }
            ReplicaBlockInfoVersions::V0_0_4(info) => {
                println!(
                    "📦 Block v4: slot={}, parent_slot={}, blockhash={}, executed_tx_count={}, entry_count={}",
                    info.slot, info.parent_slot, info.blockhash, info.executed_transaction_count, info.entry_count
                );
            }
        }
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        println!("✅ Account data notifications enabled");
        true
    }

    fn transaction_notifications_enabled(&self) -> bool {
        println!("✅ Transaction notifications enabled");
        true
    }

    fn entry_notifications_enabled(&self) -> bool {
        println!("ℹ Entry notifications enabled");
        false
    }
}

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin: Box<dyn GeyserPlugin> = Box::new(Plugin::default());
    Box::into_raw(plugin)
}
