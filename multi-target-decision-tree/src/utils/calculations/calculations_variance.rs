use super::variance::MultiTargetLabelMetrics;

pub(super) fn calculate_loss_vector(
    left_variance_vector: Vec<f64>,
    right_variance_vector: Vec<f64>,
    left_size: f64,
    right_size: f64,
    number_of_targets: usize,
) -> Vec<f64> {
    let total_size = left_size + right_size;
    let left_weight = left_size / total_size;
    let right_weight = right_size / total_size;

    assert_eq!(left_variance_vector.len(), right_variance_vector.len());
    let loss_vector = left_variance_vector
        .iter()
        .zip(right_variance_vector)
        .map(|(&left_element, right_element)| {
            (left_weight * left_element) + (right_weight * right_element)
        })
        .collect();
    loss_vector
}

pub(super) fn calculate_variance_vector(
    multi_target_label_metrics: &MultiTargetLabelMetrics,
    number_of_labels: f64,
    number_of_targets: usize,
) -> Vec<f64> {
    let mut variance_result_vector = vec![0.0; number_of_targets];
    for i in 0..number_of_targets {
        let mean = multi_target_label_metrics.mean_of_labels_vector[i];
        let left = multi_target_label_metrics.sum_of_squared_labels_vector[i];
        let right = number_of_labels * (mean * mean);
        let variance = (left - right) / number_of_labels;
        variance_result_vector[i] = variance;
    }
    variance_result_vector
}

pub(super) fn get_multi_target_label_metrics(
    labels: &Vec<Vec<f64>>,
    number_of_targets: usize,
) -> MultiTargetLabelMetrics {
    let label_sum_vectors = get_label_sum_vectors(labels, number_of_targets);
    let sum_of_labels_vector = label_sum_vectors.0;
    let sum_of_squared_labels_vector = label_sum_vectors.1;
    let number_of_labels = labels.len() as f64;
    let mean_of_labels_vector =
        get_mean_of_labels_vector(number_of_labels, number_of_targets, &sum_of_labels_vector);
    MultiTargetLabelMetrics {
        sum_of_labels_vector,
        sum_of_squared_labels_vector,
        mean_of_labels_vector,
    }
}

fn get_label_sum_vectors(labels: &Vec<Vec<f64>>, number_of_targets: usize) -> (Vec<f64>, Vec<f64>) {
    let mut sum_of_labels_vector = vec![0.0; number_of_targets];
    let mut sum_of_squared_labels_vector = vec![0.0; number_of_targets];
    labels.iter().for_each(|label_vector| {
        for (i, label_value) in label_vector.iter().enumerate() {
            sum_of_labels_vector[i] += label_value;
            sum_of_squared_labels_vector[i] += label_value * label_value;
        }
    });
    (sum_of_labels_vector, sum_of_squared_labels_vector)
}

fn get_mean_of_labels_vector(
    number_of_labels: f64,
    number_of_targets: usize,
    sum_of_labels_vector: &Vec<f64>,
) -> Vec<f64> {
    let mut mean_of_labels_vector = vec![0.0; number_of_targets];
    for (i, sum_of_labels_vector_element) in sum_of_labels_vector.iter().enumerate() {
        mean_of_labels_vector[i] = sum_of_labels_vector_element / number_of_labels;
    }
    mean_of_labels_vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_label_sums() {
        let labels = vec![vec![1., 3., 4.], vec![12., 5., 3.], vec![3., 5., 7.]];
        let label_metrics = super::get_multi_target_label_metrics(&labels, 3);
        println!("{:?}", label_metrics);
    }

    #[test]
    fn test_calculate_variance_vector() {
        let labels = vec![vec![1., 3., 4.], vec![2., 5., 3.], vec![3., -5., 7.]];
        let number_of_targets = 3;
        let label_metrics = super::get_multi_target_label_metrics(&labels, number_of_targets);
        let variance_vector = calculate_variance_vector(
            &label_metrics,
            label_metrics.sum_of_labels_vector.len() as f64,
            number_of_targets,
        );
        println!("{:?}", label_metrics);
        println!("{:?}", variance_vector);
        assert_eq!(variance_vector[0], 2.0 / 3.0);
    }
}
