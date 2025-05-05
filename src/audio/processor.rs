use crate::error::{ Result, AudioError };

pub fn normalize_peaks(peaks: &[f32], group_size: usize) -> Result<Vec<f32>> {
    if group_size == 0 {
        return Err(AudioError::InvalidData);
    }
    
    if peaks.is_empty() {
        return Err(AudioError::InvalidData);
    }
    
    let mut reduced_data: Vec<f32> = Vec::new();
    
    for chunk in peaks.chunks(group_size) {
        if chunk.is_empty() {
            return Err(AudioError::ProcessingError);
        }
        
        let max_value = chunk.iter()
            .fold(f32::NEG_INFINITY, |acc, &x| acc.max(x));
        reduced_data.push(max_value);
    }

    Ok(reduced_data)
}