use std::collections::HashMap;
use itertools::zip;
use counter::Counter;

struct NaiveBayes {
    model: Option<NaiveBayesParameters>
}

struct NaiveBayesParameters {
    priors: HashMap<u16, f64>,
    probabilities: HashMap<u16, HashMap<String, f64>>,
}

impl NaiveBayes {
    pub fn train(&mut self, x: Vec<&str>, y: Vec<u16>){
        let mut words: HashMap<u16, Vec<String>> = HashMap::new();
        let mut probabilities: HashMap<u16, HashMap<String, f64>> = HashMap::new();
        let priors: HashMap<u16, f64> = y.iter().collect::<Counter<_>>().iter().map(|(label, count)| (**label, *count as f64 / y.len() as f64)).collect();

        for (text, label) in zip(x, y).into_iter() {
            for word in text.split(" ") {
                words.entry(label)
                    .or_insert(vec![])
                    .push(String::from(word))
            }
        }

        for (label, words) in words.iter() {
            let counter = words.iter().collect::<Counter<_>>();

            for (word, count) in counter.iter () {
                let mut map = HashMap::new();
                probabilities.entry(*label)
                    .or_insert(map)
                    .insert(String::from(*word), (*count as f64 / words.len() as f64));
            }
        }

        self.model = Option::from(NaiveBayesParameters {
            priors,
            probabilities,
        })
    }

    pub fn predict(&self, text: &str) -> u16 {
        let mut model = &self.model.as_ref().expect("model not initialized");
        let mut predictions: HashMap<u16, f64> = HashMap::new();
        for (label, prior) in &model.priors {
            let mut probability: f64 = 1.;
            let mut default_probability: f64 = &model.probabilities[&label]
                .values()
                .into_iter()
                .fold(0., |mut sum: f64, val| sum + *val ) / model.probabilities[&label]
                                                                                .values().len() as f64 ;
            for word in text.split(" ") {
                let mut mul = model.probabilities[&label].get(word).unwrap_or(&default_probability);
                probability *= mul;
            }
            probability *= prior;
            predictions.insert(*label, probability);
        }
        *predictions.iter().max_by( |a, b| a.1.partial_cmp(&b.1).unwrap()).map(|(k, _v)| k).expect("didn't find maximum prediction")

    }

    pub fn new() -> NaiveBayes {
        NaiveBayes {
            model: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spam_prediction()  {
        // lazily define texts as histograms of words rather than actual messages
        let texts = vec!["dear dear dear dear dear dear dear dear",
                         "friend friend friend friend friend",
                         "lunch lunch lunch",
                         "money",
                         "dear dear",
                         "friend",
                         "money money money money"
                        ];
        let mut naive_bayes = NaiveBayes::new();
        naive_bayes.train(texts, vec![0, 0, 0, 0, 1, 1, 1]);
        let benign_text = "dear friend";
        let malicious_text = "give me your money";
        let pred_benign = naive_bayes.predict(benign_text);
        let pred_malicious = naive_bayes.predict(malicious_text);
        println!("prediction for '{}':, {}", benign_text, pred_benign);
        println!("prediction for '{}':, {}", malicious_text, pred_malicious);
        assert_eq!(0, pred_benign);
        assert_eq!(1, pred_malicious);
    }
}