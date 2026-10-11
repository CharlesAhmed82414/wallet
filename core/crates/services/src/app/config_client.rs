use std::error::Error;
use std::sync::Arc;

use fiat::IpAddressProvider;
use primitives::{ConfigResponse, ConfigVersions, Features, Release, SwapConfig, SwapProvider};

use super::client_country;
use super::repository::Repository;
use crate::assets::AssetCatalogClient;

#[derive(Clone)]
pub struct ConfigClient {
    repository: Arc<dyn Repository>,
    ip_address_provider: Arc<dyn IpAddressProvider>,
    asset_catalog: Arc<AssetCatalogClient>,
}

impl ConfigClient {
    pub(crate) fn new(repository: Arc<dyn Repository>, ip_address_provider: Arc<dyn IpAddressProvider>, asset_catalog: Arc<AssetCatalogClient>) -> Self {
        Self {
            repository,
            ip_address_provider,
            asset_catalog,
        }
    }

    pub async fn get_config(&self, ip_address: &str, country: Option<&str>) -> Result<ConfigResponse, Box<dyn Error + Send + Sync>> {
        let (features, releases, catalog) = tokio::try_join!(self.get_features(ip_address, country), self.get_releases(), self.asset_catalog.get())?;

        let response = ConfigResponse {
            features,
            releases,
            versions: ConfigVersions {
                fiat_on_ramp_assets: catalog.fiat_on_ramp_assets.version as i32,
                fiat_off_ramp_assets: catalog.fiat_off_ramp_assets.version as i32,
                swap_assets: catalog.swap_assets.version as i32,
            },
            swap: SwapConfig {
                enabled_providers: SwapProvider::all().iter().map(|provider| provider.as_ref().to_string()).collect(),
            },
        };
        Ok(response)
    }

    async fn get_features(&self, ip_address: &str, country: Option<&str>) -> Result<Features, Box<dyn Error + Send + Sync>> {
        let country = client_country(self.ip_address_provider.as_ref(), ip_address, country).await?;
        let policies = self.repository.get_features().await?;
        Ok(Features::for_country(&policies, &country))
    }

    async fn get_releases(&self) -> Result<Vec<Release>, Box<dyn Error + Send + Sync>> {
        Ok(self.repository.get_releases().await?)
    }
}
