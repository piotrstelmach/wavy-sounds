use crate::error::{ Result, AudioError };

pub fn normalize_peaks(peaks: &[f32], group_size: usize) -> Result<Vec<f32>> {
    
    if group_size == 0 {
        return Err(AudioError::InvalidData);
    }
    
    if peaks.is_empty() {
        return Err(AudioError::InvalidData);
    }
    
    let mut reduced_data: Vec<f32> = Vec::new();
    
    for i in (0..peaks.len()).step_by(group_size) {
        let end_index = (i + group_size).min(peaks.len());
        let group = &peaks[i..i + end_index];
        
        if group.is_empty() {
            return Err(AudioError::ProcessingError);
        }
        
        let max_value = group.iter().fold(f32::NEG_INFINITY, |acc, &x| acc.max(x));
        reduced_data.push(max_value); // Push the maximum (or minimum) value to the reduced data array
    }

    Ok(reduced_data)
}