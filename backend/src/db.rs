use futures_signals::signal::Mutable;
use once_cell::race::OnceBox;
use sqlx::{Pool, Postgres};


pub fn db_connection() -> &'static Mutable<Option<Pool<Postgres>>>  {
    static INSTANCE: OnceBox<Mutable<Option<Pool<Postgres>>>> = OnceBox::new();
    INSTANCE.get_or_init(move || Box::new(Mutable::new(None)))
}
