
use fluvio_smartmodule::dataplane::smartmodule::{SmartModuleExtraParams};

mod model;
use model::GithubStars;
use fluvio_smartmodule::{smartmodule, Record, RecordData, Result as EyreResult};

#[smartmodule(aggregate)]
pub fn aggregate(accumulator: RecordData, current: &Record) -> EyreResult<RecordData> {
    let mut accumulated_stars: GithubStars =
        serde_json::from_slice(accumulator.as_ref()).unwrap_or_default();

    let current_stars: GithubStars = serde_json::from_slice(current.value.as_ref())?;
    accumulated_stars.star_update =
        accumulated_stars.stargazers_count != current_stars.stargazers_count;
    accumulated_stars.stargazers_count = current_stars.stargazers_count;

    let accumulated_stars: RecordData = accumulated_stars.try_into()?;
    Ok(accumulated_stars)
}




#[smartmodule(init)]
fn init(_params: SmartModuleExtraParams) -> EyreResult<()> {
    // You can refer to the example SmartModules in Fluvio's GitHub Repository
    // https://github.com/infinyon/fluvio/tree/master/smartmodule
    todo!("Provide initialization logic for your SmartModule")
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_check() {
        let res: Result<GithubStars, serde_json::Error> =
            serde_json::from_str("{\"stargazers_count\":950,\"star_update\":false}");
        assert!(res.is_ok());
    }
    #[test]
    fn check_aggregator_init() {
        let accumulated_stars: GithubStars = GithubStars {
            stargazers_count: 1,
            star_update: false,
        };
        let current_stars: GithubStars = GithubStars {
            stargazers_count: 1,
            ..Default::default()
        };
        let out = aggregate(
            accumulated_stars.try_into().unwrap(),
            &current_stars.try_into().unwrap(),
        );
        assert!(out.is_ok());
        let out = out.unwrap();
        let out: GithubStars = out.try_into().unwrap();
        assert_eq!(out.stargazers_count, 1);
        assert!(!out.star_update);
    }
    #[test]
    fn check_aggregator_increment() {
        let accumulated_stars: GithubStars = GithubStars {
            stargazers_count: 1,
            ..Default::default()
        };
        let current_stars: GithubStars = GithubStars {
            stargazers_count: 2,
            ..Default::default()
        };
        let out = aggregate(
            accumulated_stars.try_into().unwrap(),
            &current_stars.try_into().unwrap(),
        );
        assert!(out.is_ok());
        let out = out.unwrap();
        let out: GithubStars = out.try_into().unwrap();
        assert_eq!(out.stargazers_count, 2);
        assert!(out.star_update);
    }
}
