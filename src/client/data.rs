use crate::error::Result;
use crate::http::HttpClient;
use crate::types::{Activity, ClosedPosition, Position, PositionValue, Trade};

/// Client for accessing position and portfolio data
///
/// This client provides access to user positions and portfolio values.
/// It does not require authentication.
pub struct DataClient {
    http_client: HttpClient,
}

impl DataClient {
    /// Create a new DataClient
    ///
    /// # Arguments
    /// * `host` - The base URL for the data API (typically different from main CLOB API)
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            http_client: HttpClient::new(host),
        }
    }

    /// Get all positions for a user
    ///
    /// # Arguments
    /// * `user` - The user's wallet address
    ///
    /// # Returns
    /// A list of positions owned by the user
    pub async fn get_positions(&self, user: &str) -> Result<Vec<Position>> {
        let path = format!("/positions?user={}", user);
        self.http_client.get(&path, None).await
    }

    /// Get the total value of positions for a user
    ///
    /// # Arguments
    /// * `user` - The user's wallet address
    ///
    /// # Returns
    /// A list of position values for the user
    pub async fn get_positions_value(&self, user: &str) -> Result<Vec<PositionValue>> {
        let path = format!("/value?user={}", user);
        self.http_client.get(&path, None).await
    }

    /// Get recent trades
    ///
    /// # Arguments
    /// * `user` - User wallet address to filter trades
    ///
    /// # Returns
    /// A list of recent trades
    pub async fn get_trades(&self, user: &str) -> Result<Vec<Trade>> {
        let path = format!("/trades?user={}", user);
        self.http_client.get(&path, None).await
    }

    /// Get recent activity
    ///
    /// # Arguments
    /// * `user` - User wallet address to filter activity
    ///
    /// # Returns
    /// A list of recent activity events
    pub async fn get_activity(&self, user: &str) -> Result<Vec<Activity>> {
        let path = format!("/activity?user={}", user);
        self.http_client.get(&path, None).await
    }

    /// Get closed positions
    ///
    /// # Arguments
    /// * `user` - User wallet address
    ///
    /// # Returns
    /// A list of closed positions for the user
    pub async fn get_closed_positions(&self, user: &str) -> Result<Vec<ClosedPosition>> {
        let path = format!("/closed-positions?user={}", user);
        self.http_client.get(&path, None).await
    }
}
