pub mod area;
pub mod countries;
pub mod languages;
pub mod mbid;
pub mod scripts;

pub use crate::{area::Area, countries::Country, languages::Language, mbid::Mbid, scripts::Script};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
