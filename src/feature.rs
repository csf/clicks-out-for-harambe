pub type Feature = f64;
pub type FeatureVec = Vec<Feature>;

pub fn eval_features(headline: String) -> FeatureVec {
    let mut features = Vec::new();

    features.push(f1(&headline));

    features
}

fn f1(headline: &String) -> Feature {
    
    match headline.split_whitespace().count() {
        0 => 0.0,
        x => 1.0 / (x as f64)
    }

}

#[cfg(test)]
mod tests {
    use super::eval_features;

    #[test]
    fn test_f1() {
        let h1 = " This is  it".to_string();
        let h2 = "".to_string();
        let h3 = " This ".to_string();

        assert_eq!(eval_features(h1), vec!(1.0/3.0));
        assert_eq!(eval_features(h2), vec!(0.0));
        assert_eq!(eval_features(h3), vec!(1.0));
    }
}
