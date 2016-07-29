mod client;

mod main;
mod price;
mod orders_list;
mod order_reg;
mod luck;

pub use self::client::Client;
pub use self::main::MainPage;
pub use self::price::PriceListPage;
pub use self::orders_list::OrdersListPage;
pub use self::order_reg::OrderRegPage;
pub use self::luck::CheckYourLuckPage;
