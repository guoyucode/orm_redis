use proc_macro::TokenStream;

mod redis;
mod utils;


#[proc_macro_derive(RedisHget)]
pub fn cache_query_macro(input: TokenStream) -> TokenStream {
    redis::cache_query_macro(input)
}

#[proc_macro_derive(RedisZrange)]
pub fn cache_query_zrange_macro(input: TokenStream) -> TokenStream{
    redis::cache_query_zrange_macro(input)
}

#[test]
fn test() {}
