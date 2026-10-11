use std::error::Error;

use fiat::IpAddressProvider;

pub(crate) async fn client_country(ip_addresses: &dyn IpAddressProvider, ip_address: &str, country: Option<&str>) -> Result<String, Box<dyn Error + Send + Sync>> {
    match country {
        Some(country) => Ok(country.to_string()),
        None => Ok(ip_addresses.get_ip_address(ip_address).await?.alpha2),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use async_trait::async_trait;
    use primitives::IPAddressInfo;

    use super::*;

    #[derive(Default)]
    struct CountingIpAddressProvider {
        lookups: AtomicUsize,
    }

    #[async_trait]
    impl IpAddressProvider for CountingIpAddressProvider {
        async fn get_ip_address(&self, ip_address: &str) -> Result<IPAddressInfo, Box<dyn Error + Send + Sync>> {
            self.lookups.fetch_add(1, Ordering::SeqCst);
            Ok(IPAddressInfo {
                alpha2: "DE".to_string(),
                state: String::new(),
                ip_address: ip_address.to_string(),
            })
        }
    }

    #[tokio::test]
    async fn test_client_country() {
        let ip_addresses = CountingIpAddressProvider::default();

        assert_eq!(client_country(&ip_addresses, "192.0.2.1", Some("US")).await.unwrap(), "US");
        assert_eq!(ip_addresses.lookups.load(Ordering::SeqCst), 0);

        assert_eq!(client_country(&ip_addresses, "192.0.2.1", None).await.unwrap(), "DE");
        assert_eq!(ip_addresses.lookups.load(Ordering::SeqCst), 1);
    }
}
