mod client;

mod main;
mod price;
mod orders_list;

pub use self::client::Client;
pub use self::main::MainPage;
pub use self::price::PriceListPage;
pub use self::orders_list::OrdersListPage;
