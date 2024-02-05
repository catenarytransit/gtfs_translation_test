use gtfs_structures::*;

#[tokio::main]
async fn main() {
   let gtfs_jp_1 = Gtfs::from_path("1").unwrap();

   gtfs_jp_1.print_stats();

// println!("{:#?}", gtfs_jp_1.translations);
}
