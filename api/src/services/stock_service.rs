use crate::{
    errors::ApplicationError,
    infrastructure::db::stock_repository::ProductStockRepository,
    models::{
        dto::{product_dto::ListProductsParams, stock_dto::ProductWithStockDto},
        ids::WorkspaceId,
    },
};

#[derive(Clone)]
pub struct StockService {
    pub stock_repository: ProductStockRepository,
}

impl StockService {
    pub fn new(stock_repository: ProductStockRepository) -> Self {
        Self { stock_repository }
    }

    pub async fn list_products_with_stock(
        &self,
        workspace_id: WorkspaceId,
        params: ListProductsParams,
    ) -> Result<Vec<ProductWithStockDto>, ApplicationError> {
        let search = params.search.unwrap_or_default();

        let products = self
            .stock_repository
            .list_products_with_stock(workspace_id, &search)
            .await?;

        Ok(products
            .into_iter()
            .map(|(product, stock)| ProductWithStockDto { product, stock })
            .collect())
    }
}
