use linear_search::linear_search_extract_interger_number::return_number_base_64;

fn main() {
    let texto_puro = String::from("O(log N) basically means time goes up linearly while the n goes up exponentially. So if it takes 1 second to compute 10 elements, it will take 2 seconds to compute 100 elements, 3 seconds to compute 1000 elements, and so on.");
    let numero = return_number_base_64(&texto_puro);

    println!("{}", numero);
}