pub mod cfg;
pub mod handler;
#[path = "generated/header_generated.rs"]
pub mod header;
pub mod server;

#[cfg(test)]
mod tests {
    #[test]
    fn test_thread_affinity() {
        let core_ids = core_affinity::get_core_ids().unwrap();
        core_ids.into_iter().for_each(|id| {
            println!("{}", id.id);
        });
    }
}
