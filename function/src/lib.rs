extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use regex::Regex;

#[proc_macro]
pub fn log_info(input: TokenStream) -> TokenStream {
    let re = Regex::new(r"(\d{4}) - (\d{2}) - (\d{2})").unwrap();
    let mut output = "".to_owned();

    for cap in re.captures_iter(&input.to_string()[..]) {
        output.push_str(&format!(
            "Year is : {} Month is : {} Day is : {}\n",
            &cap[1], &cap[2], &cap[3]
        ));
    }

    TokenStream::from(quote! {
        println!("{}", #output);
    })
}
