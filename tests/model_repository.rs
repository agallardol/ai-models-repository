#[cfg(test)]
mod tests {
    use ai_model_catalog::{get_openrouter_model, list_openrouter_model_ids};

    #[test]
    fn test_list_openrouter_model_ids() {
        let model_ids: Vec<&str> = list_openrouter_model_ids().collect();
        assert!(!model_ids.is_empty(), "Model ID list should not be empty");
    }

    #[test]
    fn test_get_openrouter_model() {
        let model = get_openrouter_model("google/gemini-2.5-flash").expect("Model not found");
        assert_eq!(model.id, "google/gemini-2.5-flash");
        assert!(
            model.context_length.unwrap_or(0) == 1_048_576,
            "Context length should be 1,048,576"
        );
    }
}
