use juniper::FieldResult;

use crate::domain::product::Product;

pub struct ProductQuery;

#[juniper::graphql_object]
impl ProductQuery {
    fn getProductById(id: String) -> FieldResult<Product> {
        Ok(Product::new(
            id,
            String::from("aslfdjjl"),
            String::from(""),
            vec![String::from("Placeholder")],
            2.34,
            String::from("€"),
        ))
    }

    fn getProducts() -> FieldResult<Vec<Product>> {
        let products = vec![Product::new(
            String::from("123123"),
            String::from("aslfdjjl"),
            String::from(""),
            vec![String::from("Placeholder")],
            2.34,
            String::from("€"),
        )];
        Ok(products)
    }

    fn getProductsByCategory(category: String) -> FieldResult<Vec<Product>> {
        let products = vec![Product::new(
            String::from("123123"),
            String::from("aslfdjjl"),
            String::from(""),
            vec![String::from("Placeholder"), category],
            2.34,
            String::from("€"),
        )];
        Ok(products)
    }

    fn getProductsPriceInterval(minPrice: f64, maxPrice: f64) -> FieldResult<Vec<Product>> {
        let products = vec![
            Product::new(
                String::from("123123"),
                String::from("aslfdjjl"),
                String::from(""),
                vec![String::from("Placeholder")],
                minPrice,
                String::from("€"),
            ),
            Product::new(
                String::from("123123"),
                String::from("aslfdjjl"),
                String::from(""),
                vec![String::from("Placeholder")],
                maxPrice,
                String::from("€"),
            ),
        ];
        Ok(products)
    }


    fn getProductsMaxPrice(maxPrice: f64) -> FieldResult<Vec<Product>> {
        let products = vec![
            Product::new(
                String::from("123123"),
                String::from("aslfdjjl"),
                String::from(""),
                vec![String::from("Placeholder")],
                maxPrice,
                String::from("€"),
            ),
            Product::new(
                String::from("123123"),
                String::from("aslfdjjl"),
                String::from(""),
                vec![String::from("Placeholder")],
                maxPrice,
                String::from("€"),
            ),
        ];
        Ok(products)
    }

    fn getProductsMinPrice(minPrice: f64) -> FieldResult<Vec<Product>> {
        let products = vec![
            Product::new(
                String::from("123123"),
                String::from("aslfdjjl"),
                String::from(""),
                vec![String::from("Placeholder")],
                minPrice,
                String::from("€"),
            ),
            Product::new(
                String::from("123123"),
                String::from("aslfdjjl"),
                String::from(""),
                vec![String::from("Placeholder")],
                minPrice,
                String::from("€"),
            ),
        ];
        Ok(products)
    }
}
